use crate::ledger::lib::{ LedgerErrors, Ledger };

pub mod ledger;



fn main() {

    let mut ledger = Ledger::new();

    let name1 = String::from("Alexis");
    let name2 = String::from("Aaron");

    let key1 = name1.clone().to_lowercase();
    let key2 = name2.clone().to_lowercase();
    handle_error(ledger.new_account(&name1));
    handle_error(ledger.new_account(&name1));
    handle_error(ledger.new_account(&name2));
    handle_error(ledger.add_balance_account(&key1, 100));
    handle_error(ledger.add_balance_account(&name1, 100));
    handle_error(ledger.transfer(&key1, &key2, 10));
    handle_error(ledger.transfer(&key1, &key2, 100000));

    println!("{:?}", ledger);
}


fn handle_error(res: Result<(), LedgerErrors>) {
    if let Err(err) = res {
        println!("the action fails cause: {}", err);
    }
}