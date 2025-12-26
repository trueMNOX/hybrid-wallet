use crate::error::WalletError;

pub struct Account {
    pub id: u64,
    pub balance: u64,
}

impl Account {
    pub fn new(id: u64) -> Self {
        Self { id, balance: 0 }
    }

    pub fn deposit(&mut self, amount: u64) -> Result<(), WalletError> {
        if amount == 0 {
            return Err(WalletError::InvalidAmount);
        }
        self.balance += amount;
        Ok(())
    }

    pub fn withdraw(&mut self, amount: u64) -> Result<(), WalletError> {
        if amount == 0 {
            return Err(WalletError::InvalidAmount);
        }
        if amount > self.balance {
            return Err(WalletError::InsufficientBalance);
        }
        self.balance -= amount;
        Ok(())
    }
}

mod tests {
    use super::*;

    #[test]
    fn new_account_has_zero_balance() {
        let account = Account::new(1);
        assert_eq!(account.balance, 0)
    }

    #[test]
    fn deposit_increases_balance() {
        let mut account = Account::new(1);
        let result = account.deposit(1000);
        assert!(result.is_ok());
        assert_eq!(account.balance, 1000);
    }

    #[test]
    fn deposit_zero_should_fail() {
        let mut account = Account::new(1);
        let result = account.deposit(0);
        assert!(result.is_err())
    }

    #[test]
    fn withdraw_decreases_balance() {
        let mut account = Account::new(1);
        account.deposit(200).unwrap();
        let result = account.withdraw(100);

        assert!(result.is_ok());
        assert_eq!(account.balance, 100);
    }
    #[test]
    fn withdraw_more_than_balance_should_fail() {
        let mut account = Account::new(1);
        account.deposit(50).unwrap();

        let result = account.withdraw(100);
        assert!(matches!(result, Err(WalletError::InsufficientBalance)));
    }
}
