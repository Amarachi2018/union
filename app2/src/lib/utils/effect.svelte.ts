import {
  type Cause,
  type Effect,
  Either,
  Exit,
  type Fiber,
  FiberId,
  Match,
  Option,
  Runtime,
} from "effect"
import { constant, constVoid, dual, flow, identity } from "effect/Function"
import type { Simplify } from "effect/Types"

/* eslint-disable @typescript-eslint/no-explicit-any, prefer-rest-params,  */
/**
 * @see https://github.com/Effect-TS/effect/blob/4a687e8dbe57702833d162a007a9f29863e514af/packages/effect/src/internal/runtime.ts#L29
 */
const makeDual = <Args extends Array<any>, Return>(
  f: (
    runtime: Runtime.Runtime<never>,
    effect: () => Effect.Effect<any, any>,
    ...args: Args
  ) => Return,
): {
  <R>(
    runtime: Runtime.Runtime<R>,
  ): <A, E>(effect: () => Effect.Effect<A, E, R>, ...args: Args) => Return
  <R, A, E>(
    runtime: Runtime.Runtime<R>,
    effect: () => Effect.Effect<A, E, R>,
    ...args: Args
  ): Return
} =>
  function(this: any) {
    // biome-ignore lint/style/noArguments: <explanation>
    if (arguments.length === 1) {
      // biome-ignore lint/style/noArguments: <explanation>
      const runtime = arguments[0]
      return (effect: any, ...args: Args) => f(runtime, effect, ...args)
    }
    // biome-ignore lint/style/noArguments: <explanation>
    return f.apply(this, arguments as any)
  } as any
/* eslint-enable @typescript-eslint/no-explicit-any, prefer-rest-params,  */

export type RunPromiseExitResult<A, E> = {
  readonly current: Option.Option<Exit.Exit<A, E>>
  readonly either: Option.Option<Either.Either<A, Cause.Cause<E>>>
  readonly interrupt: (reason?: any) => void
}
type RunPromiseExitOptions = {
  /**
   * Adjust state behavior on interrupt:
   * - `error`: *(default)* set state with {@link Exit.Failure}
   * - `ignore`: skip set state
   * - `none`: set state to {@link Option.None}
   */
  onInterrupt?: "ignore" | "none" | "error" | undefined
  variant?: "post" | "root" | "pre" | undefined
}
/**
 * {@link Effect.runPromiseExit} but like {@link $effect}.
 */
export const runPromiseExitWithRuntime: {
  <R = never>(
    runtime: Runtime.Runtime<R>,
  ): <A, E, R>(
    effect: () => Effect.Effect<A, E, R>,
    options?: RunPromiseExitOptions | undefined,
  ) => Simplify<RunPromiseExitResult<A, E>>
  <A, E, R = never>(
    runtime: Runtime.Runtime<R>,
    effect: () => Effect.Effect<A, E, R>,
    options?: RunPromiseExitOptions | undefined,
  ): Simplify<RunPromiseExitResult<A, E>>
} = makeDual(
  <A, E, R>(
    runtime: Runtime.Runtime<R>,
    self: () => Effect.Effect<A, E, R>,
    options?: RunPromiseExitOptions | undefined,
  ): Simplify<RunPromiseExitResult<A, E>> => {
    const onInterrupt: NonNullable<RunPromiseExitOptions["onInterrupt"]> = options?.onInterrupt
      ?? "error"
    const variant: NonNullable<RunPromiseExitOptions["variant"]> = options?.variant ?? "post"
    const runPromiseExit = Runtime.runPromiseExit(runtime)
    let state = $state<Option.Option<Exit.Exit<A, E>>>(Option.none())
    let controller = new AbortController()
    const wrap = Match.type<Exit.Exit<A, E>>().pipe(
      Match.whenAnd(
        Exit.isInterrupted<A, E>,
        () => onInterrupt === "ignore",
        () => Option.none(),
      ),
      Match.whenAnd(
        Exit.isInterrupted<A, E>,
        () => onInterrupt === "none",
        () => Option.some(Option.none()),
      ),
      Match.orElse(flow(identity<Exit.Exit<A, E>>, Option.some, Option.some)),
    )

    const effect: () => void | VoidFunction = () => {
      controller = new AbortController()
      runPromiseExit(
        self(),
        { signal: controller.signal },
      ).then(flow(
        wrap,
        Option.match({
          onNone: constVoid,
          onSome: (exit) => (state = exit),
        }),
      ))
      return () => controller.abort("teardown")
    }

    switch (variant) {
      case "post":
        $effect(effect)
        break
      case "root":
        $effect.root(effect)
        break
      case "pre":
        $effect.pre(effect)
        break
    }
    // Match.value(variant).pipe(
    //   Match.when("pre", constant($effect.pre(effect))),
    //   Match.when("post", constant($effect(effect))),
    //   Match.when("root", constant($effect.root(effect))),
    //   Match.exhaustive,
    // )
    // f(() => {
    //   controller = new AbortController()
    //   runPromiseExit(
    //     self(),
    //     { signal: controller.signal },
    //   ).then(flow(
    //     wrap,
    //     Option.match({
    //       onNone: constVoid,
    //       onSome: (exit) => (state = exit),
    //     }),
    //   ))
    //   return () => controller.abort("teardown")
    // })

    return {
      get current() {
        return state
      },
      get either() {
        return Option.map(
          state,
          Exit.match({
            onFailure: Either.left,
            onSuccess: Either.right,
          }),
        )
      },
      interrupt: (reason?: any) => controller.abort(reason),
    } as const
  },
)

export type RunForkResult<A, E> = {
  readonly fiber: Fiber.RuntimeFiber<A, E>
  readonly interrupt: () => void
}
/**
 * {@link Effect.runFork} with automatic {@link Fiber} cleanup.
 */
export const runForkWithRuntime: {
  <R>(
    runtime: Runtime.Runtime<R>,
  ): <A, E>(
    effect: () => Effect.Effect<A, E, R>,
    options?: Runtime.RunForkOptions | undefined,
  ) => Simplify<RunForkResult<A, E>>
  <R, A, E>(
    runtime: Runtime.Runtime<R>,
    effect: () => Effect.Effect<A, E, R>,
    options?: Runtime.RunForkOptions | undefined,
  ): Simplify<RunForkResult<A, E>>
} = makeDual(
  <R, A, E>(
    runtime: Runtime.Runtime<R>,
    self: () => Effect.Effect<A, E, R>,
    options?: Runtime.RunForkOptions,
  ): Simplify<RunForkResult<A, E>> => {
    const runFork = Runtime.runFork(runtime)
    let state = $state<Fiber.RuntimeFiber<A, E> | null>(null)

    $effect(() => {
      state = runFork(self(), { immediate: true, ...options })
      return () => state?.unsafeInterruptAsFork(FiberId.none)
    })

    return {
      get fiber() {
        // biome-ignore lint/style/noNonNullAssertion: immediate execution
        return state!
      },
      interrupt: () => state?.unsafeInterruptAsFork(FiberId.none),
    } as const
  },
)
