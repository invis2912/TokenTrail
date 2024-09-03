use ethers::prelude::*;
use std::sync::Arc;
use std::collections::HashSet;
use crate::errors::AppError;

// Fetch BAR transfers from the given contract address
pub async fn fetch_transfers(
    provider: Arc<Provider<Http>>,
    contract_address: Address,
    address_list: Vec<Address>,
    blocks_back: u64,
) -> Result<Vec<Log>, AppError> {
    let latest_block = provider.get_block_number().await.map_err(AppError::ProviderError)?;
    let from_block = BlockNumber::Number((latest_block.as_u64() - blocks_back).into());

    let filter = Filter::new()
        .address(contract_address)
        .event("Transfer(address,address,uint256)")
        .from_block(from_block)
        .to_block(BlockNumber::Latest);
        
    let logs: Vec<Log> = provider.get_logs(&filter).await.map_err(AppError::ProviderError)?;

    let address_set: HashSet<Address> = address_list.into_iter().collect();

    let filtered_logs: Vec<Log> = logs
        .into_iter()
        .filter(|log| {
            if log.topics.len() < 3 {
                return false;
            }
            let from = Address::from_slice(&log.topics[1].as_bytes()[12..]);
            let to = Address::from_slice(&log.topics[2].as_bytes()[12..]);
            address_set.contains(&from) || address_set.contains(&to)
        })
        .collect();

    if filtered_logs.is_empty() {
        Err(AppError::NoTransactionsFound)
    } else {
        Ok(filtered_logs)
    }
}

// Fetch transactions between two addresses
pub async fn fetch_transactions_between(
    provider: Arc<Provider<Http>>,
    contract_address: Address,
    from_address: Address,
    to_address: Address,
    blocks_back: u64,
) -> Result<Vec<Log>, AppError> {
    let latest_block = provider.get_block_number().await.map_err(AppError::ProviderError)?;
    let from_block = BlockNumber::Number((latest_block.as_u64() - blocks_back).into());

    let filter = Filter::new()
        .address(contract_address)
        .event("Transfer(address,address,uint256)")
        .from_block(from_block)
        .to_block(BlockNumber::Latest);
        
    let logs: Vec<Log> = provider.get_logs(&filter).await.map_err(AppError::ProviderError)?;

    let filtered_logs: Vec<Log> = logs
        .into_iter()
        .filter(|log| {
            if log.topics.len() < 3 {
                return false;
            }
            let log_from = Address::from_slice(&log.topics[1].as_bytes()[12..]);
            let log_to = Address::from_slice(&log.topics[2].as_bytes()[12..]);
            (log_from == from_address && log_to == to_address) || 
            (log_from == to_address && log_to == from_address)
        })
        .collect();

    if filtered_logs.is_empty() {
        Err(AppError::NoTransactionsFound)
    } else {
        Ok(filtered_logs)
    }
}
