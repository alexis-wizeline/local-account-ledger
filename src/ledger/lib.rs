use borsh::{BorshDeserialize, BorshSerialize, to_vec};
use std::{
    collections::HashMap,
    fs::{self, File},
    io::{Read, Write},
};

pub enum LedgerErrors {
    AccountAlreadyExist(String),
    AccountNotFound(String),
    AccountInsuficientFunds,
}

impl std::fmt::Display for LedgerErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LedgerErrors::AccountAlreadyExist(message) => {
                write!(f, "The account: {} already exists", message)
            }
            LedgerErrors::AccountNotFound(message) => {
                write!(f, "account: {} not found", message)
            }
            LedgerErrors::AccountInsuficientFunds => {
                write!(f, "account does not have enough funds")
            }
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct Account {
    balance: u32,
    owner: String,
}

impl Account {
    fn new() -> Account {
        Account {
            balance: 0,
            owner: "system".to_owned(),
        }
    }
}

#[derive(Debug)]
pub struct Ledger {
    accounts: HashMap<String, Account>,
}

impl Ledger {
    pub fn save_to_file(&self) -> std::io::Result<()> {
        let path = "./temp/ledger.bin";
        if let Some(parent) = std::path::Path::new(path).parent() {
            fs::create_dir(parent)?
        }

        let contet: Vec<u8> = to_vec(&self.accounts).unwrap();
        let mut file = File::create(path).unwrap();
        file.write_all(&contet)
    }

    pub fn load_from_file() -> Self {
        let path = "./temp/ledger.bin";
        if let Ok(file) = File::open(path).as_mut() {
            let mut buff:Vec<u8> = Vec::new();
            file.read_to_end(&mut buff).unwrap();

            let accounts: HashMap<String, Account> = HashMap::try_from_slice(&buff).unwrap();
            return Self { accounts: accounts };
        }
        Self {
            accounts: HashMap::new(),
        }
    }

    pub fn new_account(&mut self, account_name: &String) -> Result<(), LedgerErrors> {
        let key = account_name.clone().to_lowercase();
        if self.accounts.contains_key(&key) {
            return Err(LedgerErrors::AccountAlreadyExist(key));
        }
        let acc = Account::new();
        self.accounts.insert(key, acc);

        Ok(())
    }

    pub fn add_balance_account(
        &mut self,
        account_name: &String,
        amount: u32,
    ) -> Result<(), LedgerErrors> {
        match self.accounts.get_mut(account_name) {
            Some(acc) => {
                acc.balance += amount;
                Ok(())
            }
            None => Err(LedgerErrors::AccountNotFound(account_name.to_owned())),
        }
    }

    pub fn transfer(
        &mut self,
        account_name_from: &String,
        account_name_to: &String,
        amount: u32,
    ) -> Result<(), LedgerErrors> {
        if !self.accounts.contains_key(account_name_from) {
            return Err(LedgerErrors::AccountNotFound(account_name_from.to_owned()));
        }

        if !self.accounts.contains_key(account_name_to) {
            return Err(LedgerErrors::AccountNotFound(account_name_to.to_owned()));
        }

        let acc_from = self.accounts.get_mut(account_name_from).unwrap();
        if acc_from.balance < amount {
            return Err(LedgerErrors::AccountInsuficientFunds);
        }
        acc_from.balance -= amount;

        let acc_to = self.accounts.get_mut(account_name_to).unwrap();
        acc_to.balance += amount;

        Ok(())
    }
}
