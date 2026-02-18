use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum LedgerError {
    AccountNotFound(String),
    InsufficientFunds { require: u64, available: u64 },
    DuplicateAccount(String),
    InvalidTransfer(String),
    SerializationError(String),
}

impl Display for LedgerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AccountNotFound(pubkey) => write!(f, "{} was not fount", pubkey),
            Self::InsufficientFunds { require, available } => write!(
                f,
                "Insuficient funds to make the trasnfer: requires: {}, account has: {}",
                require, available
            ),
            Self::DuplicateAccount(pubkey) => write!(f, "account {} already exists", pubkey),
            Self::InvalidTransfer(message) => write!(f, "invalid transfer for: {}", message),
            Self::SerializationError(message) => write!(f, "{}", message),
        }
    }
}

impl Error for LedgerError {}
