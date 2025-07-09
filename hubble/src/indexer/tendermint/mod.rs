use color_eyre::eyre::Report;
use cometbft_rpc::JsonRpcError;

use crate::indexer::api::IndexerError;

mod block_handle;
pub mod config;
mod context;
mod fetcher_client;
mod mapping;
mod postgres;
mod provider;

impl From<JsonRpcError> for IndexerError {
    fn from(error: JsonRpcError) -> Self {
        Self::ProviderError(Box::new(Report::from(error)))
    }
}
