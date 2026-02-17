use crate::pkg::account::{Account, AccountType, Summarizable};
use crate::pkg::errors::LedgerError;
use crate::pkg::ledger::Ledger;

pub mod pkg;

fn main() {
    let mut ledger = Ledger::new();
    let wallet_1 = Account::new(AccountType::Wallet { balance: 10000 });
    let wallet_2 = Account::new(AccountType::Wallet { balance: 50000000 });

    handle_error(ledger.add_account(wallet_1.clone()).err());
    handle_error(ledger.add_account(wallet_1.clone()).err());
    handle_error(ledger.add_account(wallet_2.clone()).err());

    let wallest = ledger.accounts_by_type("wallet");
    println!("wallets found: {:?}", wallest);

    handle_error(
        ledger
            .transfer(&wallet_2.pubkey, &wallet_1.pubkey, 100)
            .err(),
    );

    println!("{:?}", ledger);
    println!("{}", ledger.total_supply());
    println!("{:?} - {:?}", wallet_1.summary(), wallet_1);

    println!();
    let buff = wallet_1.clone().save_to_bytes().unwrap();
    println!("{:?}", buff);

    let wallet_1_bytes = Account::from_bytes(&buff);
    println!("{:?}", wallet_1_bytes.unwrap());

    handle_error(ledger.save_ledger("./temp/ledger/ledger.bin").err());

    println!("old: {:?}", ledger);
    let ledger_2 = Ledger::load_ledger("./temp/ledger/ledger.bin");
    println!("new: {:?}", ledger_2);

    handle_error(ledger_2.err());
}

fn handle_error(res: Option<LedgerError>) {
    if let Some(err) = res {
        println!("an error happens: {}", err);
    }
}
