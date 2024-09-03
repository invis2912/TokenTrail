mod cli;
mod errors;
mod provider;
mod token;
mod commands;
mod tests;
mod constants;  // Import the constants module

use cli::{Cli, Commands};
use dotenv::dotenv;
use figlet_rs::FIGfont;
use ethers::prelude::*;
use std::str::FromStr;
use clap::Parser;
use errors::AppError;
use provider::setup_provider;
use token::get_token_contract;
use commands::fetch::*;
use commands::print::*;
use constants::*;  // Use the constants

// Main function
#[tokio::main]
async fn main() -> Result<(), Box<AppError>> {
    dotenv().ok();  // Load the .env file
    
    // Parse the CLI arguments
    let mut cli = Cli::parse();

    // Set default values if not provided
    if cli.command.is_none() {
        cli.command = Some(Commands::FetchTransactionsBetween {
            from: DEFAULT_FROM_ADDRESS.to_string(),
            to: DEFAULT_TO_ADDRESS.to_string(),
            blocks_back: DEFAULT_BLOCKS_BACK,
        });
    }

    // Load standard font and print "TokenTrail" as ASCII art
    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert("Welcome to TokenTrail");
    println!("{}", figure.unwrap());

    println!("Starting program...");

    println!("Selected Chain: ");

    // Convert and print the selected chain as ASCII art
    let chain_name_art = standard_font.convert(&format!("{:?}", cli.chain));
    println!("{}", chain_name_art.unwrap());

    // Get the RPC URL for the selected chain
    let rpc_env_var = cli.chain.rpc_env_var();
    let rpc_url = std::env::var(rpc_env_var).map_err(AppError::EnvVarError)?;
    println!("RPC URL for {:?}: {}", cli.chain, rpc_url);

    // Create the provider
    let provider = setup_provider(&rpc_url)?;
    println!("Provider created successfully.");

    // Parse the token address
    println!("Parsing token address...");
    let token_address = Address::from_str(&cli.token_address)
        .map_err(|_| AppError::AddressParseError(cli.token_address.clone()))?;
    println!("Token address parsed successfully: {:?}", token_address);

    // Instantiate the ERC-20 contract
    let token_contract = get_token_contract(provider.clone(), token_address);

    // Fetch and print the token name
    let token_name = token_contract.name().call().await.map_err(AppError::ContractError)?;
    println!("Token Name:");
      
    // Convert token_name to ASCII art
    let token_name_art = standard_font.convert(&token_name);
    println!("{}", token_name_art.unwrap());
    
    // Fetch and print the token ticker symbol
    let token_symbol = token_contract.symbol().call().await.map_err(AppError::ContractError)?;
    println!("Token Ticker:");
    // Convert token_symbol to ASCII art
    let token_symbol_art = standard_font.convert(&token_symbol);
    println!("{}", token_symbol_art.unwrap());

    // Fetch the token decimals
    let token_decimals = token_contract.decimals().call().await.map_err(AppError::ContractError)?;
    println!("Token decimals: {:?}", token_decimals);

    // Execute the command based on the CLI arguments
    match &cli.command {
        Some(Commands::FetchTransfers { blocks_back, addresses }) => {
            // Parse the addresses
            let mut address_list = Vec::new();
            for (index, addr_str) in addresses.iter().enumerate() {
                let trimmed_addr = addr_str.trim();
                let address = Address::from_str(trimmed_addr)
                    .map_err(|_| AppError::AddressParseError(trimmed_addr.to_string()))?;

                address_list.push(address);
                println!("Address {} parsed successfully: {:?}", index, address);
            }
            println!("Address list parsed successfully. Total addresses: {}", address_list.len());

            println!("Fetching transfers...");

            // Fetch the transfers
            match fetch_transfers(provider.clone(), token_address, address_list, *blocks_back).await {  // Added dereference
                Ok(transfers) => print_transactions(provider.clone(), transfers, token_decimals).await?,
                Err(AppError::NoTransactionsFound) => println!("No transactions found for the given addresses and block range."),
                Err(e) => return Err(Box::new(e)),
            }
        },
        Some(Commands::FetchTransactionsBetween { from, to, blocks_back }) => {
            let from_address = Address::from_str(from)
               .map_err(|_| AppError::AddressParseError(from.to_string()))?;
            let to_address = Address::from_str(to)
               .map_err(|_| AppError::AddressParseError(to.to_string()))?;

            println!("Fetching transactions...");

            // Fetch the transactions between the given addresses
            match fetch_transactions_between(provider.clone(), token_address, from_address, to_address, *blocks_back).await {  
                Ok(transfers) => print_transactions(provider.clone(), transfers, token_decimals).await?,
                Err(AppError::NoTransactionsFound) => println!("No transactions found between the given addresses in the specified block range."),
                Err(e) => return Err(Box::new(e)),
            }
        },
        None => {
            println!("No command specified. Use --help for usage information.");
        }
    }

    println!("Thank you for using TokenTrail.");
    Ok::<(), Box<AppError>>(())
}
