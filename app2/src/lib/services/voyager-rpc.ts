import {
  FetchHttpClient,
  HttpBody,
  HttpClient,
  HttpClientRequest,
  HttpClientResponse,
} from "@effect/platform"
import { Array as A, Effect, Option, pipe, Schema } from "effect"

const VOY_RUN_URL = "https://voy.run"

const JsonRpcRequest = Schema.Struct({
  jsonrpc: Schema.Literal("2.0"),
  method: Schema.String,
  params: Schema.Array(Schema.Unknown),
  id: Schema.Number,
})

// Extract chain ID from universal chain ID (remove family prefix)
export function extractChainId(universalChainId: string): string {
  const parts = universalChainId.split(".")
  return parts.length > 1 ? parts[1] : universalChainId
}

// Schema transform to remove revision prefix from height (e.g., "5-1493366" -> "1493366")
const HeightWithRevisionPrefix = Schema.String.pipe(
  Schema.transform(
    Schema.String,
    {
      decode: (height) => {
        const dashIndex = height.indexOf("-")
        return dashIndex !== -1 ? height.substring(dashIndex + 1) : height
      },
      encode: (height) => height, // No need to encode back to prefixed form
    },
  ),
)

const JsonRpcResponse = Schema.Union(
  Schema.Struct({
    id: Schema.Number,
    jsonrpc: Schema.Literal("2.0"),
    result: HeightWithRevisionPrefix,
  }),
  Schema.Struct({
    id: Schema.Number,
    jsonrpc: Schema.Literal("2.0"),
    error: Schema.Struct({
      code: Schema.Number,
      message: Schema.String,
    }),
  }),
)

type JsonRpcResponse = Schema.Schema.Type<typeof JsonRpcResponse>

// Fetch heights for a single batch of chain IDs
const fetchHeightsBatch = (chainIds: string[], startId: number = 1) =>
  Effect.gen(function*() {
    const requests = chainIds.map((chainId, index) => ({
      jsonrpc: "2.0" as const,
      method: "voyager_queryLatestHeight",
      params: [extractChainId(chainId), true],
      id: startId + index,
    }))

    const httpClient = (yield* HttpClient.HttpClient).pipe(
      HttpClient.withTracerDisabledWhen(() => true),
    )

    const body = yield* HttpBody.json(requests)
    const response = yield* httpClient.post(VOY_RUN_URL, {
      body,
    })

    const json = yield* response.json
    const decoded = yield* Schema.decodeUnknown(Schema.Array(JsonRpcResponse))(json)

    return { requests, responses: decoded }
  })

// Fetch finalized heights for multiple chains with smart retry
export const fetchFinalizedHeights = (universalChainIds: string[]) =>
  Effect.gen(function*() {
    const heightMap = new Map<string, Option.Option<string>>()

    yield* Effect.log("Fetching finalized heights").pipe(
      Effect.annotateLogs({
        chainCount: universalChainIds.length,
        chainIds: universalChainIds.map(extractChainId),
      }),
    )

    // Initial batch request
    const initialResult = yield* fetchHeightsBatch(universalChainIds).pipe(
      Effect.catchAll(error =>
        Effect.logError("Failed to fetch heights batch", error).pipe(
          Effect.andThen(Effect.succeed({
            requests: [],
            responses: [] as JsonRpcResponse[],
          })),
        )
      ),
    )

    // Process successful responses and collect failed IDs
    const failedChainIds: string[] = []
    const successfulResponses = new Map<number, JsonRpcResponse>()

    // Map responses by ID for easier lookup
    initialResult.responses.forEach(response => {
      successfulResponses.set(response.id, response)
    })

    // Process each chain ID
    universalChainIds.forEach((ucid, index) => {
      const requestId = index + 1
      const response = successfulResponses.get(requestId)

      if (response && "result" in response) {
        heightMap.set(ucid, Option.some(response.result))
      } else {
        // Mark as failed for retry
        failedChainIds.push(ucid)
        heightMap.set(ucid, Option.none())
      }
    })

    // Retry failed requests if any
    if (failedChainIds.length > 0) {
      yield* Effect.log("Retrying failed requests").pipe(
        Effect.annotateLogs({
          count: failedChainIds.length,
          chainIds: failedChainIds.map(extractChainId),
        }),
      )

      const retryResult = yield* fetchHeightsBatch(failedChainIds, 1000).pipe(
        Effect.catchAll(error =>
          Effect.logError("Failed to retry heights batch", error).pipe(
            Effect.andThen(Effect.succeed({
              requests: [],
              responses: [] as JsonRpcResponse[],
            })),
          )
        ),
      )

      // Process retry responses
      const retryResponses = new Map<number, JsonRpcResponse>()
      retryResult.responses.forEach(response => {
        retryResponses.set(response.id, response)
      })

      // Update the height map with retry results
      failedChainIds.forEach((ucid, index) => {
        const requestId = 1000 + index
        const response = retryResponses.get(requestId)

        if (response && "result" in response) {
          heightMap.set(ucid, Option.some(response.result))
        }
        // If still failed, keep as Option.none() (already set above)
      })
    }

    const successCount = Array.from(heightMap.values()).filter(Option.isSome).length
    const failureCount = heightMap.size - successCount

    yield* Effect.log("Completed fetching finalized heights").pipe(
      Effect.annotateLogs({
        total: heightMap.size,
        successful: successCount,
        failed: failureCount,
      }),
    )

    return heightMap
  }).pipe(
    Effect.withLogSpan("fetchFinalizedHeights"),
    Effect.provide(FetchHttpClient.layer),
  )
