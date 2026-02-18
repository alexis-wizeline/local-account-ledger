use borsh::{BorshDeserialize, to_vec};
use std::{
    fs::{File, create_dir_all},
    io::{Read, Write},
};

use crate::pkg::{
    account::{Account, AccountType},
    errors::LedgerError,
};

#[derive(Debug)]
pub struct Ledger {
    accounts: Vec<Account>,
}

impl Ledger {
    pub fn new() -> Self {
        Self {
            accounts: Vec::new(),
        }
    }

    pub fn load_ledger(path: &str) -> Result<Ledger, LedgerError> {
        let mut file = File::open(path);
        if let Err(err) = file {
            return Err(LedgerError::SerializationError(err.to_string()));
        }

        let mut buff: Vec<u8> = Vec::new();
        if let Err(err) = file.as_mut().unwrap().read_to_end(&mut buff) {
            return Err(LedgerError::SerializationError(err.to_string()));
        };

        let accounts: Result<Vec<Account>, std::io::Error> = Vec::<Account>::try_from_slice(&buff);
        if let Err(err) = accounts {
            return Err(LedgerError::SerializationError(err.to_string()));
        }

        Ok(Ledger {
            accounts: accounts.unwrap(),
        })
    }

    pub fn save_ledger(&self, path: &str) -> Result<(), LedgerError> {
        let last_index = path.rfind("/").unwrap_or(0);
        if let Err(err) = create_dir_all(path.get(0..=last_index).unwrap_or("")) {
            return Err(LedgerError::SerializationError(err.to_string()));
        }

        let buff = to_vec(&self.accounts);
        if let Err(err) = &buff {
            return Err(LedgerError::SerializationError(err.to_string()));
        }

        let mut file = File::create(path).unwrap();
        if let Err(err) = file.write_all(&buff.unwrap()) {
            return Err(LedgerError::SerializationError(err.to_string()));
        }

        Ok(())
    }

    pub fn add_account(&mut self, acc: Account) -> Result<&Account, LedgerError> {
        let pubkey = &acc.pubkey.clone();
        if self.account_exist(pubkey) {
            return Err(LedgerError::DuplicateAccount(pubkey.to_string()));
        }

        self.accounts.push(acc);
        Ok(self.accounts.iter().find(|a| &a.pubkey == pubkey).unwrap())
    }

    pub fn accounts_by_type(&self, type_name: &str) -> Vec<&Account> {
        self.accounts
            .iter()
            .filter(|acc| match type_name {
                "wallet" => acc.is_account_type(AccountType::Wallet { balance: 0 }),
                "program" => acc.is_account_type(AccountType::Program {
                    executable: false,
                    program_data: Vec::new(),
                }),
                "token_account" => acc.is_account_type(AccountType::TokenAccount {
                    mint: "".to_string(),
                    token_balance: 0,
                    delegate: None,
                }),
                "stake" => acc.is_account_type(AccountType::Stake {
                    validator: "".to_string(),
                    staked_amount: 0,
                }),
                _ => false,
            })
            .collect()
    }

    pub fn transfer(&mut self, from: &str, to: &str, amount: u64) -> Result<(), LedgerError> {
        if !self.account_exist(&from.to_string()) {
            return Err(LedgerError::AccountNotFound(from.to_string()));
        }

        if !self.account_exist(&from.to_string()) {
            return Err(LedgerError::AccountNotFound(to.to_string()));
        }

        let wallets = self.accounts_by_type("wallet");

        let from_is_wallet = wallets.iter().any(|w| w.pubkey == from);
        if !from_is_wallet {
            return Err(LedgerError::InvalidTransfer(format!(
                "key: {} is not a Wallet",
                from
            )));
        }

        let to_is_wallet = wallets.iter().any(|w| w.pubkey == to);
        if !to_is_wallet {
            return Err(LedgerError::InvalidTransfer(format!(
                "key: {} is not a Wallet",
                to
            )));
        }

        let from_wallet = self.accounts.iter_mut().find(|w| w.pubkey == from).unwrap();
        if from_wallet.lamports < amount {
            return Err(LedgerError::InsufficientFunds {
                require: amount,
                available: from_wallet.lamports,
            });
        }

        from_wallet.lamports -= amount;
        if let AccountType::Wallet { ref mut balance } = from_wallet.account_type {
            *balance -= amount;
        }

        let to_wallet = self.accounts.iter_mut().find(|w| w.pubkey == to).unwrap();
        to_wallet.lamports += amount;
        if let AccountType::Wallet { ref mut balance } = to_wallet.account_type {
            *balance += amount;
        }

        Ok(())
    }

    pub fn total_supply(&self) -> u64 {
        self.accounts
            .iter()
            .map(|acc| acc.lamports)
            .reduce(|total, value| total + value)
            .unwrap()
    }

    fn account_exist(&self, pubkey: &String) -> bool {
        self.accounts
            .iter()
            .any(|acc| acc.pubkey == pubkey.to_owned())
    }
}
