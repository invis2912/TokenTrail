use ethers::prelude::*;
use std::sync::Arc;
use chrono::{DateTime, Utc};
use crate::errors::AppError;

// Print the transactions to the console
pub async fn print_transactions(
    provider: Arc<Provider<Http>>,
    transfers: Vec<Log>,
    token_decimals: u8,
) -> Result<(), AppError> {
    println!("Transfers fetched successfully. Count: {}", transfers.len());
    for transfer in transfers {
        let event = "Transfer";
        let block_number = transfer.block_number.unwrap_or_default();
        let block = provider.get_block(block_number).await.map_err(AppError::ProviderError)?;
        let timestamp = block.map(|b| b.timestamp.as_u64()).unwrap_or_default();
        
        let date: DateTime<Utc> = DateTime::from_timestamp(timestamp as i64, 0)
            .unwrap_or_else(|| Utc::now());

        let from = Address::from_slice(&transfer.topics[1].as_bytes()[12..]);
        let to = Address::from_slice(&transfer.topics[2].as_bytes()[12..]);
        let amount_raw = U256::from_big_endian(&transfer.data.0);
        let amount = amount_raw.as_u128() as f64 / 10f64.powi(token_decimals as i32);
        let tx_hash = transfer.transaction_hash.unwrap_or_default();
        let block_hash = transfer.block_hash.unwrap_or_default();

        println!("Event: {}", event);
        println!("Date: {}", date);
        println!("From: {:?}", from);
        println!("To: {:?}", to);
        println!("Amount: {}", amount); 
        println!("Transaction Hash: {:?}", tx_hash);
        println!("Block Hash: {:?}", block_hash);
        println!("Block Number: {:?}", block_number);
        println!("--------------------");
    }

    Ok(())
}
