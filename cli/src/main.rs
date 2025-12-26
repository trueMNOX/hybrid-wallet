use core::{error::WalletError, ledger::Ledger, transaction::Transaction};
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_usage();
        return;
    }
    let mut ledger = Ledger::new();
    match args[1].as_str() {
        "create-account" => {
            if args.len() != 3 {
                eprintln!("Usage: create-account <account_id>");
                return;
            }
            let account_id = parse_u64(&args[2]);
            ledger.create_account(account_id);
            println!("Account {} created", account_id);
        }
        "deposit" => {
            if args.len() != 4 {
                eprintln!("Usage: deposit <account_id> <amount>");
                return;
            }
            let account_id = parse_u64(&args[2]);
            let amount = parse_u64(&args[3]);
            let result = ledger.apply(Transaction::Deposit { account_id, amount });
            handle_result(result, "Deposit successful");
        }
        "withdraw" => {
            if args.len() != 4 {
                eprintln!("Usage: withdraw <account_id> <amount>");
                return;
            }
            let account_id = parse_u64(&args[2]);
            let amount = parse_u64(&args[3]);
            let result = ledger.apply(Transaction::Withdraw { account_id, amount });
            handle_result(result, "Withdraw successful");
        }
        _ => {
            eprintln!("Unknown command");
            print_usage();
        }
    }
}
fn print_usage() {
    println!("Usage:");
    println!("  create-account <account_id>");
    println!("  deposit <account_id> <amount>");
    println!("  withdraw <account_id> <amount>");
}
fn parse_u64(input: &str) -> u64 {
    input.parse::<u64>().unwrap_or_else(|_| {
        eprintln!("Invalid number: {}", input);
        std::process::exit(1)
    })
}
fn handle_result(result: Result<(), WalletError>, success_msg: &str) {
    match result {
        Ok(_) => println!("{}", success_msg),
        Err(e) => eprintln!("Error: {}", e),
    }
}
