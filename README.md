# Basic Ledger

A simple Rust-based digital ledger system for managing accounts and transactions, inspired by blockchain concepts and using Solana SDK primitives.

## Features
- Manage multiple account types: Wallets, Programs, Token Accounts, Stakes
- Add accounts and prevent duplicates
- Transfer funds between wallet accounts
- Query accounts by type
- Serialize and deserialize ledger/account data using [Borsh](https://github.com/near/borsh)
- Save and load ledger state to/from disk
- Uses Solana public keys for account identification

## Project Structure
- `src/main.rs`: Example usage and entry point
- `src/pkg/ledger.rs`: Ledger logic (accounts, transfers, persistence)
- `src/pkg/account.rs`: Account types and serialization
- `src/pkg/errors.rs`: Custom error types

## Example
```rust
let mut ledger = Ledger::new();
let wallet_1 = Account::new(AccountType::Wallet { balance: 10000 });
let wallet_2 = Account::new(AccountType::Wallet { balance: 50000000 });
ledger.add_account(wallet_1.clone());
ledger.add_account(wallet_2.clone());
ledger.transfer(&wallet_2.pubkey, &wallet_1.pubkey, 100);
ledger.save_ledger("./temp/ledger/ledger.bin");
```

## Dependencies
- [borsh](https://crates.io/crates/borsh)
- [solana-sdk](https://crates.io/crates/solana-sdk)
