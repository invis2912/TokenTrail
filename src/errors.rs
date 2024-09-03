use std::fmt;
use std::error::Error;
use ethers::prelude::*;
use url::ParseError;

// Define the AppError enum
#[derive(Debug)]
pub enum AppError {
    EnvVarError(std::env::VarError),
    ProviderError(ethers::providers::ProviderError),
    ParseError(ParseError),
    AddressParseError(String),
    ContractError(ethers::contract::ContractError<Provider<Http>>),  // Use this for contract-related errors
    NoTransactionsFound,
}
// Display the error message
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::EnvVarError(e) => write!(f, "Environment variable error: {}", e),
            AppError::ProviderError(e) => write!(f, "Provider error: {}", e),
            AppError::ParseError(e) => write!(f, "Parse error: {}", e),
            AppError::AddressParseError(addr) => write!(f, "Failed to parse address: {}", addr),
            AppError::ContractError(e) => write!(f, "Contract error: {}", e),
            AppError::NoTransactionsFound => write!(f, "No transactions found"),
        }
    }
}

// Implement the Error trait for AppError
impl Error for AppError {}

// Convert the error to an AppError
impl From<std::env::VarError> for AppError {
    fn from(err: std::env::VarError) -> Self {
        AppError::EnvVarError(err)
    }
}
// Convert the error to an AppError
impl From<ethers::providers::ProviderError> for AppError {
    fn from(err: ethers::providers::ProviderError) -> Self {
        AppError::ProviderError(err)
    }
}
// Convert the error to an AppError
impl From<ParseError> for AppError {
    fn from(err: ParseError) -> Self {
        AppError::ParseError(err)
    }
}
// Convert the error to an AppError
impl From<ethers::contract::ContractError<Provider<Http>>> for AppError {
    fn from(err: ethers::contract::ContractError<Provider<Http>>) -> Self {
        AppError::ContractError(err)
    }
}
