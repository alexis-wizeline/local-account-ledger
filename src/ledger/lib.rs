use std::collections::HashMap;

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

#[derive(Debug)]
struct Account {
    // name: String,
    balance: u32,
}

impl Account {
    fn new() -> Account {
        Account { balance: 0 }
    }
}

#[derive(Debug)]
pub struct Ledger {
    accounts: HashMap<String, Account>,
}

impl Ledger {
    pub fn new() -> Self {
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
