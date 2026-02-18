use solana_sdk::pubkey::Pubkey;

use crate::pkg::account::{Account, AccountType, Summarizable};
use crate::pkg::errors::LedgerError;
use crate::pkg::ledger::Ledger;

pub mod pkg;

fn main() {
    let mut ledger = Ledger::new();
    let wallet_1 = Account::new(AccountType::Wallet { balance: 10000 });
    let wallet_2 = Account::new(AccountType::Wallet { balance: 50000000 });
    let program = Account::new(AccountType::Program {
        executable: true,
        program_data: b"program data".to_vec(),
    });
    let token_account = Account::new(AccountType::TokenAccount {
        mint: Pubkey::new_unique().to_string(),
        token_balance: 200_000_000_000,
        delegate: Some(String::from("a wild string")),
    });
    let stake_account = Account::new(AccountType::Stake {
        validator: Pubkey::new_unique().to_string(),
        staked_amount: 40_000_000_000,
    });

    handle_error(ledger.add_account(wallet_1.clone()).err());
    handle_error(ledger.add_account(program.clone()).err());
    handle_error(ledger.add_account(token_account.clone()).err());
    handle_error(ledger.add_account(stake_account.clone()).err());

    println!();
    println!("accounts in ledger summary");
    print_summary(ledger.accounts_by_type("all"));
    println!();
    println!("invalid transfer");
    handle_error(ledger.transfer(&wallet_1.pubkey, &program.pubkey, 10).err());

    println!();
    handle_error(ledger.add_account(wallet_2.clone()).err());
    println!("wallets before transfer");
    print_summary(ledger.accounts_by_type("wallet"));
    println!();

    println!("valid transfer");
    handle_error(
        ledger
            .transfer(&wallet_1.pubkey, &wallet_2.pubkey, 100)
            .err(),
    );

    println!();
    println!("wallets after transfer");
    print_summary(ledger.accounts_by_type("wallet"));

    println!();
    println!("duplicated account");
    handle_error(ledger.add_account(wallet_1.clone()).err());

    handle_error(ledger.save_ledger("./temp/ledger/ledger.bin").err());
    println!("original ledger supply: {:?}", ledger.total_supply());
    if let Ok(ledger_2) = Ledger::load_ledger("./temp/ledger/ledger.bin") {
        println!("loaded ledger supply: {:?}", ledger_2.total_supply());
    }
}

fn handle_error(res: Option<LedgerError>) {
    if let Some(err) = res {
        println!("an error happens: {}", err);
    }
}

fn print_summary(accounts: Vec<&Account>) {
    for account in accounts {
        let start_end_str = String::from("-").repeat(account.summary().len());
        println!("{start_end_str}");
        println!("{}", account.summary());
        println!("{start_end_str}");
    }
}
