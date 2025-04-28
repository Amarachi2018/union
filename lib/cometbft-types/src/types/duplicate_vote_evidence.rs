use serde::{Deserialize, Serialize};
use unionlabs::google::protobuf::timestamp::Timestamp;

use crate::types::vote::Vote;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DuplicateVoteEvidence {
    pub vote_a: Vote,
    pub vote_b: Vote,
    #[serde(rename = "TotalVotingPower", with = "::serde_utils::string")]
    pub total_voting_power: i64,
    #[serde(rename = "ValidatorPower", with = "::serde_utils::string")]
    pub validator_power: i64,
    #[serde(rename = "Timestamp")]
    pub timestamp: Timestamp,
}

#[cfg(feature = "proto")]
pub mod proto {
    use unionlabs::{
        errors::MissingField, google::protobuf::timestamp::TryFromTimestampError, required,
    };

    use crate::types::{duplicate_vote_evidence::DuplicateVoteEvidence, vote};

    impl From<DuplicateVoteEvidence> for protos::cometbft::types::v1::DuplicateVoteEvidence {
        fn from(value: DuplicateVoteEvidence) -> Self {
            Self {
                vote_a: Some(value.vote_a.into()),
                vote_b: Some(value.vote_b.into()),
                total_voting_power: value.total_voting_power,
                validator_power: value.validator_power,
                timestamp: Some(value.timestamp.into()),
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, thiserror::Error)]
    pub enum Error {
        #[error(transparent)]
        MissingField(#[from] MissingField),
        #[error("invalid vote a")]
        VoteA(#[source] vote::proto::Error),
        #[error("invalid vote b")]
        VoteB(#[source] vote::proto::Error),
        #[error("invalid timestamp")]
        Timestamp(#[from] TryFromTimestampError),
    }

    impl TryFrom<protos::cometbft::types::v1::DuplicateVoteEvidence> for DuplicateVoteEvidence {
        type Error = Error;

        fn try_from(
            value: protos::cometbft::types::v1::DuplicateVoteEvidence,
        ) -> Result<Self, Self::Error> {
            Ok(Self {
                vote_a: required!(value.vote_a)?.try_into().map_err(Error::VoteA)?,
                vote_b: required!(value.vote_b)?.try_into().map_err(Error::VoteB)?,
                total_voting_power: value.total_voting_power,
                validator_power: value.validator_power,
                timestamp: required!(value.timestamp)?.try_into()?,
            })
        }
    }
}
