/// BlockRequest requests a block for a specific height
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct BlockRequest {
    #[prost(int64, tag = "1")]
    pub height: i64,
}
/// BlockResponse returns block to the requested
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct BlockResponse {
    #[prost(message, optional, tag = "1")]
    pub block: ::core::option::Option<super::super::types::v1beta1::Block>,
}
/// Message is an abstract blocksync message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct Message {
    /// Sum of all possible messages.
    #[prost(oneof = "message::Sum", tags = "1, 2, 3, 4, 5")]
    pub sum: ::core::option::Option<message::Sum>,
}
/// Nested message and enum types in `Message`.
pub mod message {
    /// Sum of all possible messages.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, :: prost :: Oneof)]
    pub enum Sum {
        #[prost(message, tag = "1")]
        BlockRequest(super::BlockRequest),
        #[prost(message, tag = "2")]
        NoBlockResponse(super::NoBlockResponse),
        #[prost(message, tag = "3")]
        BlockResponse(super::BlockResponse),
        #[prost(message, tag = "4")]
        StatusRequest(super::StatusRequest),
        #[prost(message, tag = "5")]
        StatusResponse(super::StatusResponse),
    }
}
/// NoBlockResponse informs the node that the peer does not have block at the requested height
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct NoBlockResponse {
    #[prost(int64, tag = "1")]
    pub height: i64,
}
/// StatusRequest requests the status of a peer.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct StatusRequest {}
/// StatusResponse is a peer response to inform their status.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct StatusResponse {
    #[prost(int64, tag = "1")]
    pub height: i64,
    #[prost(int64, tag = "2")]
    pub base: i64,
}
impl ::prost::Name for BlockRequest {
    const NAME: &'static str = "BlockRequest";
    const PACKAGE: &'static str = "cometbft.blocksync.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cometbft.blocksync.v1beta1.{}", Self::NAME)
    }
}
impl ::prost::Name for BlockResponse {
    const NAME: &'static str = "BlockResponse";
    const PACKAGE: &'static str = "cometbft.blocksync.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cometbft.blocksync.v1beta1.{}", Self::NAME)
    }
}
impl ::prost::Name for Message {
    const NAME: &'static str = "Message";
    const PACKAGE: &'static str = "cometbft.blocksync.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cometbft.blocksync.v1beta1.{}", Self::NAME)
    }
}
impl ::prost::Name for NoBlockResponse {
    const NAME: &'static str = "NoBlockResponse";
    const PACKAGE: &'static str = "cometbft.blocksync.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cometbft.blocksync.v1beta1.{}", Self::NAME)
    }
}
impl ::prost::Name for StatusRequest {
    const NAME: &'static str = "StatusRequest";
    const PACKAGE: &'static str = "cometbft.blocksync.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cometbft.blocksync.v1beta1.{}", Self::NAME)
    }
}
impl ::prost::Name for StatusResponse {
    const NAME: &'static str = "StatusResponse";
    const PACKAGE: &'static str = "cometbft.blocksync.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cometbft.blocksync.v1beta1.{}", Self::NAME)
    }
}
