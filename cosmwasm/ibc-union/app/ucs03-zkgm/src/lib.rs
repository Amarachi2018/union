pub mod com;
pub mod contract;
pub mod msg;
mod state;
use alloy::primitives::ruint::ParseError;
use cosmwasm_std::StdError;
use thiserror::Error;
use unionlabs::primitives::Bytes;
use unionlabs_cosmwasm_upgradable::UpgradeError;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error(transparent)]
    Std(#[from] StdError),
    #[error("migration error")]
    Migrate(#[from] UpgradeError),
    #[error("invalid ibc version, got {version}")]
    InvalidIbcVersion { version: String },
    #[error("invalid operation, sender must be ibc host")]
    OnlyIBCHost,
    #[error("invalid operation, sender must be self")]
    OnlySelf,
    #[error(transparent)]
    Alloy(#[from] alloy::sol_types::Error),
    #[error("invalid zkgm instruction version: {version}")]
    UnsupportedVersion { version: u8 },
    #[error("unknown zkgm instruction opcode: {opcode}")]
    UnknownOpcode { opcode: u8 },
    #[error("unknown reply id: {id}")]
    UnknownReply { id: u64 },
    #[error("invalid operation, can only be executed by a market maker")]
    OnlyMaker,
    #[error("packet execution reentrancy not allowed")]
    AlreadyExecuting,
    #[error("order amount must be u128")]
    AmountOverflow,
    #[error("the quote token must be a valid utf8 denom")]
    InvalidQuoteToken,
    #[error("the base token must be a valid utf8 denom")]
    InvalidBaseToken,
    #[error("invalid channel balance, counterparty has been taken over?")]
    InvalidChannelBalance,
    #[error("amount must be non zero")]
    InvalidAmount,
    #[error("transfer require funds to be submitted along the transaction")]
    MissingFunds,
    #[error("receiver must be a valid address")]
    InvalidReceiver,
    #[error("receiver must be a valid address")]
    InvalidSender,
    #[error(
        "the receiver can't be validated, make sure the bech prefix matches the current chain"
    )]
    UnableToValidateReceiver,
    #[error(
        "the receiver can't be validated, make sure the bech prefix matches the current chain"
    )]
    UnableToValidateMarketMaker,
    #[error("the sender can't be validated, make sure the bech prefix matches the current chain")]
    UnableToValidateSender,
    #[error("multiplex contract address must be a valid address")]
    InvalidContractAddress,
    #[error(
        "the multiplex target contract address can't be validated, make sure the bech prefix matches the current chain"
    )]
    UnableToValidateMultiplexTarget,
    #[error("feature is not yet implemented")]
    Unimplemented,
    #[error("contract creation event not found during handling `reply`")]
    ContractCreationEventNotFound,
    #[error("{0:?}")]
    InvalidPath(ParseError),
    #[error(
        "forward previousDestinationChannelId mistmatch, actual: {actual}, expted: {expected}"
    )]
    InvalidForwardDestinationChannelId { actual: u32, expected: u32 },
    #[error("forward (sent) packet is missing from the reply")]
    ForwardedPacketMissingInReply,
    #[error("could not deserialize sent packet on reply, data: {sent_packet_data}")]
    CouldNotDeserializeSentPacket {
        error: serde_json_wasm::de::Error,
        sent_packet_data: Bytes,
    },
    #[error("asynchronous multiplexing is not supported")]
    AsyncMultiplexUnsupported,
}
