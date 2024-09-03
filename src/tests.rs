#[cfg(test)]
mod tests {
    use crate::commands::fetch::{fetch_transfers, fetch_transactions_between}; // Import the functions
    use ethers::prelude::*;
    use std::str::FromStr;
    use std::sync::Arc; // Import Arc

    #[tokio::test]
    async fn test_fetch_transfers() {
        // Setup mock provider and token address
        let provider = Provider::<Http>::try_from("https://ethereum.rpc.thirdweb.com/").unwrap();
        let provider = Arc::new(provider); // Wrap provider in Arc
        let token_address = Address::from_str("0x6982508145454Ce325dDbE47a25d4ec3d2311933").unwrap();
        let addresses = vec![
            Address::from_str("0x8d10c7E20fDd521fb300172Defa8440547544128").unwrap(),
            Address::from_str("0x45326B06216DC69F8498eF211b39F4abF8047184").unwrap(),
        ];
        let blocks_back = 10000;

        // Call the function
        let result = fetch_transfers(provider, token_address, addresses, blocks_back).await;

        // Assert the result
        match result {
            Ok(transfers) => {
                assert!(!transfers.is_empty(), "Transfers should not be empty");
            }
            Err(e) => panic!("Error fetching transfers: {:?}", e),
        }
    }

    #[tokio::test]
    async fn test_fetch_transactions_between() {
        // Setup mock provider and token address
        let provider = Provider::<Http>::try_from("https://ethereum.rpc.thirdweb.com/").unwrap();
        let provider = Arc::new(provider); // Wrap provider in Arc
        let token_address = Address::from_str("0x6982508145454Ce325dDbE47a25d4ec3d2311933").unwrap();
        let from_address = Address::from_str("0x8d10c7E20fDd521fb300172Defa8440547544128").unwrap();
        let to_address = Address::from_str("0x45326B06216DC69F8498eF211b39F4abF8047184").unwrap();
        let blocks_back = 10000;

        // Call the function
        let result = fetch_transactions_between(provider, token_address, from_address, to_address, blocks_back).await;

        // Assert the result
        match result {
            Ok(transfers) => {
                assert!(!transfers.is_empty(), "Transfers should not be empty");
            }
            Err(e) => panic!("Error fetching transactions: {:?}", e),
        }
    }
}