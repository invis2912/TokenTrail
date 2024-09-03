use ethers::contract::abigen;
use ethers::prelude::*;
use ethers::providers::Provider;
use std::sync::Arc;

// Define the ERC20Token struct
abigen!(
    ERC20Token,
    r#"[ 
        function name() view returns (string)
        function symbol() view returns (string)
        function decimals() view returns (uint8)
    ]"#,
);

// Get the token contract
pub fn get_token_contract(provider: Arc<Provider<Http>>, address: Address) -> ERC20Token<Provider<Http>> {
    ERC20Token::new(address, provider)
}
