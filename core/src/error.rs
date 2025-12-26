use std::fmt;

#[derive(Debug)]
pub enum WalletError {
    InsufficientBalance,
    InvalidAmount,
    AccountNotFound,
}
impl fmt::Display for WalletError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WalletError::InsufficientBalance => write!(f, "Insufficient balance"),
            WalletError::InvalidAmount => write!(f, "Invalid amount"),
            WalletError::AccountNotFound => write!(f, "Account not found"),
        }
    }
}
