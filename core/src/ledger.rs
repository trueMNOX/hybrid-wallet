use crate::{account::Account, error::WalletError, transaction::Transaction};

use std::collections::HashMap;

pub struct Ledger {
    accounts: HashMap<u64, Account>,
}

impl Ledger {
    pub fn new() -> Self {
        Self {
            accounts: HashMap::new(),
        }
    }
    pub fn create_account(&mut self, id: u64) {
        let account = Account::new(id);
        self.accounts.insert(id, account);
    }
    pub fn apply(&mut self, tx: Transaction) -> Result<(), WalletError> {
        match tx {
            Transaction::Deposit { account_id, amount } => {
                let account = self
                    .accounts
                    .get_mut(&account_id)
                    .ok_or(WalletError::AccountNotFound)?;
                account.deposit(amount)
            }
            Transaction::Withdraw { account_id, amount } => {
                let account = self
                    .accounts
                    .get_mut(&account_id)
                    .ok_or(WalletError::AccountNotFound)?;

                account.withdraw(amount)
            }
        }
    }
}

mod tests {
    use crate::ledger;

    use super::*;

    #[test]
    fn create_account_adds_accounts() {
        let mut ledger = Ledger::new();
        ledger.create_account(1);
        let result = ledger.apply(Transaction::Deposit {
            account_id: 1,
            amount: 100,
        });
        assert!(result.is_ok())
    }

    #[test]
    fn ledger_deposit_works() {
        let mut ledger = Ledger::new();
        ledger.create_account(1);

        ledger
            .apply(Transaction::Deposit {
                account_id: 1,
                amount: 200,
            })
            .unwrap();
        let result = ledger.apply(Transaction::Withdraw {
            account_id: 1,
            amount: 100,
        });
        assert!(result.is_ok())
    }
    #[test]
    fn applying_tx_to_missing_account_fails() {
        let mut ledger = Ledger::new();

        let result = ledger.apply(Transaction::Deposit {
            account_id: 99,
            amount: 200,
        });
        assert!(matches!(result, Err(WalletError::AccountNotFound)))
    }
}
