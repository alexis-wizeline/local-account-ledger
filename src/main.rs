use crate::pkg::errors::LedgerError;
// use crate::pkg::lib::{ LedgerErrors, Ledger };
use crate::pkg::account::{Account, AccountType, Summarizable};
use crate::pkg::ledger::Ledger;

pub mod pkg;

fn main() {
    //     let name1 = String::from("Alexis");
    //     let name2 = String::from("Aaron");

    //     let key1 = name1.clone().to_lowercase();
    //     let key2 = name2.clone().to_lowercase();

    //     handle_error(ledger.new_account(&name1));
    //     handle_error(ledger.new_account(&name1));
    //     handle_error(ledger.new_account(&name2));
    //     handle_error(ledger.add_balance_account(&key1, 100));
    //     handle_error(ledger.add_balance_account(&name1, 100));
    //     handle_error(ledger.transfer(&key1, &key2, 10));
    //     handle_error(ledger.transfer(&key1, &key2, 100000));

    //    let _res =  ledger.save_to_file();
    let mut ledger = Ledger::new();
    let wallet_1 = Account::new(AccountType::Wallet { balance: 10000 });

    handle_error(ledger.add_account(wallet_1.clone()).err());
    handle_error(ledger.add_account(wallet_1.clone()).err());

    let wallest = ledger.accounts_by_type("blah");
    println!("wallets found: {:?}", wallest);

    println!("{:?}", ledger);
    println!("{:?} - {:?}", wallet_1.summary(), wallet_1);
}

fn handle_error(res: Option<LedgerError>) {
    if let Some(err) = res {
        println!("an error happens: {}", err);
    }
}
