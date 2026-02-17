use crate::pkg::{account::{Account, AccountType}, errors::LedgerError};

#[derive(Debug)]
pub struct Ledger {
    accounts: Vec<Account>
}

impl Ledger {
    pub fn new() -> Self {
        Self { accounts: Vec::new() }
    }

    pub fn add_account(&mut self, acc: Account) -> Result<&Account, LedgerError> {
        let pubkey = &acc.pubkey.clone();
        if self.account_exist(pubkey){
            return Err(LedgerError::DuplicateAccount(pubkey.to_string()))
        }
        
        self.accounts.push(acc);
        Ok(self.accounts.iter().find(|a| { &a.pubkey == pubkey }).unwrap())
    }


    pub fn accounts_by_type(&self, type_name: &str) -> Vec<&Account> {
        self.accounts.iter().filter(|acc| { match type_name {
            "wallet" => acc.is_account_type(AccountType::Wallet { balance: 0 }.to_string()),
            "program" => acc.is_account_type(AccountType::Program { executable: false, program_data: Vec::new() }.to_string()),
            "token_account" => acc.is_account_type(AccountType::TokenAccount { mint: "".to_string(), token_balance: 0, delegate: None }.to_string()),
            "stake" => acc.is_account_type(AccountType::Stake { validator: "".to_string(), staked_amount: 0 }.to_string()),
            _ => false
        } }).collect()
    }

    fn account_exist(&self, pubkey: &String) -> bool {
        self.accounts.iter().any(|acc|{ acc.pubkey == pubkey.to_owned()})
    }
}