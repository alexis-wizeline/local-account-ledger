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
}

fn handle_error(res: Option<LedgerError>) {
    if let Some(err) = res {
        println!("an error happens: {}", err);
    }
}
