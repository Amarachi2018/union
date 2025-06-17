use color_eyre::eyre::Report;
use sqlx::PgPool;
use url::Url;

use crate::indexer::{
    api::{BlockHeight, IndexerId},
    ethereum::{context::EthContext, fetcher_client::EthFetcherClient},
    FinalizerConfig, Indexer, PublisherConfig,
};

const DEFAULT_CHUNK_SIZE: usize = 200;

#[derive(Clone, Debug, serde::Deserialize)]
pub struct Config {
    pub indexer_id: IndexerId,
    pub start_height: BlockHeight,
    pub chunk_size: Option<usize>,
    pub rpc_urls: Vec<Url>,
    #[serde(default)]
    pub finalizer: FinalizerConfig,
    pub publisher: PublisherConfig,
}

impl Config {
    pub async fn build(
        self,
        pg_pool: PgPool,
        nats: Option<async_nats::jetstream::context::Context>,
    ) -> Result<Indexer<EthFetcherClient>, Report> {
        Ok(Indexer::new(
            pg_pool,
            nats,
            self.indexer_id,
            self.start_height,
            self.chunk_size.unwrap_or(DEFAULT_CHUNK_SIZE),
            self.finalizer,
            self.publisher,
            EthContext {
                rpc_urls: self.rpc_urls,
            },
        ))
    }
}
