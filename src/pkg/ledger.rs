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
                "all" => true,
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
            .unwrap_or_default()
    }

    fn account_exist(&self, pubkey: &String) -> bool {
        self.accounts
            .iter()
            .any(|acc| acc.pubkey == pubkey.to_owned())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ledger_test_add_account() {
        let mut ledger = Ledger::new();
        assert!(ledger.accounts.is_empty());

        let account = Account::new(AccountType::Wallet { balance: 0 });
        let acc = ledger.add_account(account.clone()).unwrap();
        assert!(acc.pubkey == account.pubkey);
        assert!(ledger.accounts.len() == 1);

        let err = ledger.add_account(account.clone()).unwrap_err();
        let expected_err = LedgerError::DuplicateAccount(account.pubkey.clone());
        assert_eq!(err.to_string(), expected_err.to_string());
        assert!(ledger.accounts.len() == 1);

        let new_account = Account::new(AccountType::Program {
            executable: true,
            program_data: vec![],
        });
        let ref_new_account = ledger.add_account(new_account.clone()).unwrap();
        assert!(new_account.pubkey == ref_new_account.pubkey);
        assert!(ledger.accounts.len() == 2);
    }

    #[test]
    fn ledger_test_accounts_by_type() {
        let wallet_1 = Account::new(AccountType::Wallet { balance: 0 });
        let program_1 = Account::new(AccountType::Program {
            executable: false,
            program_data: vec![],
        });

        let mut ledger = Ledger::new();

        handle_add_account(&mut ledger, wallet_1.clone());
        handle_add_account(&mut ledger, program_1.clone());

        let wallets = ledger.accounts_by_type("wallet");
        assert!(wallets.len() == 1);
        assert_eq!(wallets.first().unwrap().pubkey, wallet_1.pubkey);

        let toke_accounts = ledger.accounts_by_type("toke_account");
        assert!(toke_accounts.is_empty());

        let no_valid_type = ledger.accounts_by_type("hfsdbhfsdbhfds");
        assert!(no_valid_type.is_empty());

        let programs = ledger.accounts_by_type("program");
        assert!(programs.len() == 1);
        assert_eq!(programs.first().unwrap().pubkey, program_1.pubkey);
    }

    #[test]
    fn ledger_test_total_suply() {
        let mut ledger = Ledger::new();
        assert!(ledger.total_supply() == 0);

        let program_acc = Account::new(AccountType::Program {
            executable: false,
            program_data: vec![],
        });
        handle_add_account(&mut ledger, program_acc);
        assert!(ledger.total_supply() == 1); // program does not have balance but they need a minimun of lamports to be rent excempt;

        let stacked_coins: u64 = 200_000_000_000_000;
        let stake_acc = Account::new(AccountType::TokenAccount {
            mint: String::new(),
            token_balance: stacked_coins,
            delegate: None,
        });
        handle_add_account(&mut ledger, stake_acc);
        assert!(ledger.total_supply() == stacked_coins + 1); // we use the amouint of stacked coins as lamports for the account

        let balance_coins: u64 = 40_000_000_000;
        let wallet_acc = Account::new(AccountType::Wallet {
            balance: balance_coins,
        });
        handle_add_account(&mut ledger, wallet_acc);

        assert!(ledger.total_supply() == stacked_coins + balance_coins + 1);
    }

    #[test]
    fn ledger_test_transfer() {
        let wallet_1 = Account::new(AccountType::Wallet { balance: 10 });
        let wallet_2 = Account::new(AccountType::Wallet { balance: 2 });

        let program_1 = Account::new(AccountType::Program {
            executable: false,
            program_data: vec![],
        });

        let mut ledger = Ledger::new();
        handle_add_account(&mut ledger, wallet_1.clone());
        handle_add_account(&mut ledger, wallet_2.clone());
        handle_add_account(&mut ledger, program_1.clone());

        if let Err(err) = ledger.transfer(&wallet_1.pubkey, &wallet_2.pubkey, 15) {
            let expected_err = LedgerError::InsufficientFunds {
                require: 15,
                available: 10,
            };
            assert_eq!(err.to_string(), expected_err.to_string());
        }

        if let Err(err) = ledger.transfer(&wallet_1.pubkey, &wallet_2.pubkey, 3) {
            panic!("{}", err.to_string());
        }

        if let Err(err) = ledger.transfer(&wallet_1.pubkey, &program_1.pubkey, 1) {
            let expected_err =
                LedgerError::InvalidTransfer(format!("key: {} is not a Wallet", program_1.pubkey));
            assert_eq!(err.to_string(), expected_err.to_string());
        }
    }

    fn handle_add_account(l: &mut Ledger, acc: Account) {
        if let Err(err) = l.add_account(acc) {
            panic!("{}", err.to_string());
        }
    }
}
