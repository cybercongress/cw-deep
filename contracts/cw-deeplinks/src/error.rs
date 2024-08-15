use cosmwasm_std::{StdError, Uint64};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("Deleted Deeplink: {id}")]
    DeletedDeeplink { id: Uint64 },

    #[error("Particular links is not allowed id: {id}, from: {from}, to: {to}, type: {type_}")]
    InvalidDeeplink {id: Uint64, from: String, to: String, type_: String},

    #[error("Type not exists: {type_}")]
    TypeNotExists { type_: String },

    #[error("From not exists: {from}")]
    FromNotExists { from: String },

    #[error("To not exists: {to}")]
    ToNotExists { to: String },

    #[error("Type conflict \
        link ( id: {id}, type: {type_}, from: {from}, to: {to} ),\
        expected type: ( type: {expected_type}, from: {expected_from}, to: {expected_to} ),\
        received_from: ( type: {received_type}, from: {received_from}, to: {received_to} )")]
    TypeConflict {
        id: String, type_: String, from: String, to: String,
        expected_type: String, expected_from: String, expected_to: String,
        received_type: String, received_from: String, received_to: String
    },

    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Cannot migrate from different contract type: {previous_contract}")]
    CannotMigrate { previous_contract: String },

    #[error("Cannot migrate from unsupported version: {previous_version}")]
    CannotMigrateVersion { previous_version: String },

    #[error("Got a submessage reply with unknown id: {id}")]
    UnknownReplyId { id: u64 },

    #[error("Semver parsing error: {0}")]
    SemVer(String),
}

impl From<semver::Error> for ContractError {
    fn from(err: semver::Error) -> Self {
        Self::SemVer(err.to_string())
    }
}
