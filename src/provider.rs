use ethers::prelude::*;
use ethers::providers::{Provider, Http};
use std::convert::TryFrom;
use std::sync::Arc;
use crate::errors::AppError;

pub fn setup_provider(rpc_url: &str) -> Result<Arc<Provider<Http>>, AppError> {
    let provider = Provider::<Http>::try_from(rpc_url).map_err(AppError::ParseError)?;
    Ok(Arc::new(provider))
}
