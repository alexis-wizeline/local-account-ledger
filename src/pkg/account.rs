use borsh::{BorshDeserialize, BorshSerialize, to_vec};
use solana_sdk::pubkey::Pubkey;
use std::{
    fmt::Display,
    mem,
    time::{SystemTime, UNIX_EPOCH},
    u64,
};

use crate::pkg::errors::LedgerError;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
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

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub struct Account {
    pub pubkey: String,
    owner: String,
    pub lamports: u64,
    pub account_type: AccountType,
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

    pub fn is_account_type(&self, account_type: AccountType) -> bool {
        mem::discriminant(&self.account_type) == mem::discriminant(&account_type)
    }

    pub fn save_to_bytes(&self) -> Result<Vec<u8>, LedgerError> {
        let buff = to_vec(&self);
        if let Err(err) = &buff {
            return Err(LedgerError::SerializationError(err.to_string()));
        }

        Ok(buff.unwrap())
    }

    pub fn from_bytes(buff: &[u8]) -> Result<Account, LedgerError> {
        let account = Account::try_from_slice(&buff);
        if let Err(err) = &account {
            return Err(LedgerError::SerializationError(err.to_string()));
        }

        Ok(account.unwrap())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_account_type_display() {
        let wallet_type = AccountType::Wallet { balance: 0 };
        assert_eq!(wallet_type.to_string(), "Wallet");
        let program_type = AccountType::Program {
            executable: false,
            program_data: vec![],
        };
        assert_eq!(program_type.to_string(), "Program");
        let token_type = AccountType::TokenAccount {
            mint: "".to_string(),
            token_balance: 0,
            delegate: None,
        };
        assert_eq!(token_type.to_string(), "Token Account");
        let stake_type = AccountType::Stake {
            validator: "".to_string(),
            staked_amount: 0,
        };
        assert_eq!(stake_type.to_string(), "Stake");
    }

    #[test]
    fn test_account_wallet_serialized_round_trip() {
        let lamports = 5_000_000_000;
        let wallet = Account::new(AccountType::Wallet { balance: lamports });

        let clone_wallet = serialized_deserialize(wallet.clone());
        assert_eq!(wallet.pubkey, clone_wallet.pubkey);
        assert_eq!(wallet.lamports, clone_wallet.lamports);
        assert_eq!(wallet.owner, clone_wallet.owner);
        assert_eq!(wallet.created_at, clone_wallet.created_at);
        assert_eq!(wallet.summary(), clone_wallet.summary());
        if let AccountType::Wallet { balance } = clone_wallet.account_type {
            assert_eq!(balance, lamports);
        } else {
            panic!("account result is not a wallet type");
        }
    }

    #[test]
    fn test_account_program_round_trip_serialization() {
        let executable_val = true;
        let program_data_val = b"hello random program passing by".to_vec();
        let program_account = Account::new(AccountType::Program {
            executable: executable_val,
            program_data: program_data_val.clone(),
        });

        let clone_program_account = serialized_deserialize(program_account.clone());
        assert_eq!(program_account.pubkey, clone_program_account.pubkey);
        assert_eq!(program_account.created_at, clone_program_account.created_at);
        assert_eq!(program_account.owner, clone_program_account.owner);
        assert_eq!(program_account.lamports, clone_program_account.lamports);
        assert_eq!(program_account.summary(), clone_program_account.summary());
        if let AccountType::Program {
            executable,
            program_data,
        } = clone_program_account.account_type
        {
            assert_eq!(executable, executable_val);
            assert_eq!(program_data, program_data_val)
        } else {
            panic!("account type is not program type");
        }
    }

    #[test]
    fn test_account_token_round_trip_serialization() {
        let mint_data = Pubkey::new_unique().to_string();
        let token_balance_data = 200_000_000_000_000;
        let token_account = Account::new(AccountType::TokenAccount {
            mint: mint_data.clone(),
            token_balance: token_balance_data,
            delegate: None,
        });

        let clone_token_account = serialized_deserialize(token_account.clone());
        assert_eq!(token_account.pubkey, clone_token_account.pubkey);
        assert_eq!(token_account.created_at, clone_token_account.created_at);
        assert_eq!(token_account.lamports, clone_token_account.lamports);
        assert_eq!(token_account.owner, clone_token_account.owner);
        assert_eq!(token_account.summary(), clone_token_account.summary());

        if let AccountType::TokenAccount {
            mint,
            token_balance,
            delegate,
        } = clone_token_account.account_type
        {
            assert_eq!(mint_data, mint);
            assert_eq!(token_balance_data, token_balance);
            assert_eq!(token_account.lamports, token_balance);
            assert_eq!(delegate, None);
        } else {
            panic!("account is not a token account");
        }
    }

    #[test]
    fn test_account_stake_round_trip_serialization() {
        let validator_data = Pubkey::new_unique().to_string();
        let staked_amount_data = 2_000_000_000;
        let stake_account = Account::new(AccountType::Stake {
            validator: validator_data.clone(),
            staked_amount: staked_amount_data,
        });

        let clone_stake_account = serialized_deserialize(stake_account.clone());
        assert_eq!(stake_account.created_at, clone_stake_account.created_at);
        assert_eq!(stake_account.owner, clone_stake_account.owner);
        assert_eq!(stake_account.pubkey, clone_stake_account.pubkey);
        assert_eq!(stake_account.lamports, clone_stake_account.lamports);
        assert_eq!(stake_account.summary(), clone_stake_account.summary());
        if let AccountType::Stake {
            validator,
            staked_amount,
        } = clone_stake_account.account_type
        {
            assert_eq!(validator_data, validator);
            assert_eq!(staked_amount_data, staked_amount);
        } else {
            panic!("account is not a stake account");
        }
    }

    #[test]
    fn test_serialization_error() {
        let bad_data = b"hello random set of bytes passing by";
        if let Err(err) = Account::from_bytes(bad_data) {
            assert!(
                mem::discriminant(&err)
                    == mem::discriminant(&LedgerError::SerializationError("".to_string()))
            );
        }
    }

    #[test]
    fn test_is_account_type() {
        let account = Account::new(AccountType::Stake {
            validator: String::new(),
            staked_amount: 0,
        });
        assert!(account.is_account_type(AccountType::Stake {
            validator: String::new(),
            staked_amount: 0
        }));
        assert!(!account.is_account_type(AccountType::TokenAccount {
            mint: String::new(),
            token_balance: 0,
            delegate: None
        }));
    }

    #[test]
    fn test_account_summary() {
        let lamports: u64 = 20_000_000_000;
        let acc_type = AccountType::Stake {
            validator: String::new(),
            staked_amount: lamports,
        };
        let account = Account::new(acc_type);

        let pubkey = account.pubkey.clone();
        let sumary_key = pubkey.get(..8).unwrap().to_owned()
            + ".."
            + pubkey.chars().rev().collect::<String>().get(..4).unwrap();
        let sol = (lamports as f64) / 1_000_000_000.0;
        let acc_type_str = AccountType::Stake {
            validator: String::new(),
            staked_amount: 0,
        }
        .to_string();
        assert_eq!(
            account.summary(),
            format!("{sumary_key}|{acc_type_str}|{sol} SOL")
        );
    }

    fn serialized_deserialize(acc: Account) -> Account {
        let bytes = acc.save_to_bytes();
        if let Err(err) = bytes {
            panic!("{}", err);
        }
        let clone_acc_result = Account::from_bytes(&bytes.unwrap());
        if let Err(err) = clone_acc_result {
            panic!("{}", err);
        }

        clone_acc_result.unwrap()
    }
}
