use alloy::transports::{RpcError, TransportErrorKind};
use color_eyre::eyre::Report;

use crate::indexer::api::IndexerError;

mod abi;
mod block_handle;
pub mod config;
mod context;
mod fetcher_client;
mod log_parser;
mod mapping;
mod postgres;
mod provider;

impl From<RpcError<TransportErrorKind>> for IndexerError {
    fn from(error: RpcError<TransportErrorKind>) -> Self {
        Self::ProviderError(Report::from(error))
    }
}
