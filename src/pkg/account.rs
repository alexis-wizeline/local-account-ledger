use solana_sdk::pubkey::Pubkey;
use std::{
    fmt::Display,
    time::{SystemTime, UNIX_EPOCH},
    u64,
};

#[derive(Debug, Clone)]
pub enum AccountType {
    Wallet {
        balance: u64,
    },
    Program {
        executable: bool,
        program_data: Vec<u8>,
    },
    TokenAccount {
        mint: String,
        token_balance: u64,
        delegate: Option<String>,
    },
    Stake {
        validator: String,
        staked_amount: u64,
    },
}

impl AccountType {
    fn owner(&self) -> String {
        match self {
            AccountType::Wallet { balance: _ } => String::from("system"),
            _ => String::from(""),
        }
    }

    fn balance(&self) -> u64 {
        match self {
            Self::Wallet { balance } => *balance,
            Self::TokenAccount {
                mint: _,
                token_balance,
                delegate: _,
            } => *token_balance,
            Self::Stake {
                validator: _,
                staked_amount,
            } => *staked_amount,
            _ => 1,
        }
    }
}

impl Display for AccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AccountType::Wallet { balance: _ } => write!(f, "Wallet"),
            AccountType::Program {
                executable: _,
                program_data: _,
            } => write!(f, "Program"),
            AccountType::TokenAccount {
                mint: _,
                token_balance: _,
                delegate: _,
            } => write!(f, "Token Account"),
            AccountType::Stake {
                validator: _,
                staked_amount: _,
            } => write!(f, "Stake"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Account {
    pubkey: String,
    owner: String,
    lamports: u64,
    account_type: AccountType,
    created_at: u64,
}

impl Account {
    pub fn new(account_type: AccountType) -> Self {
        let pubkey = Pubkey::new_unique().to_string();

        Self {
            pubkey: pubkey,
            owner: account_type.owner(),
            lamports: account_type.balance(),
            account_type: account_type,
            created_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("SystemTime set to a time before UNIX EPOCH")
                .as_secs(),
        }
    }
}

pub trait Summarizable {
    fn summary(&self) -> String;
}

impl Summarizable for Account {
    fn summary(&self) -> String {
        let clone_key = self.pubkey.clone();

        let summarized_key = clone_key.get(..8).unwrap().to_owned()
            + ".."
            + clone_key
                .chars()
                .rev()
                .collect::<String>()
                .get(..4)
                .unwrap();
        let account_type = &self.account_type;
        let sol = (self.lamports as f64) / 1_000_000_000.0;

        format!("{summarized_key}|{account_type}|{sol} SOL")
    }
}
