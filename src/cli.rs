use clap::{Parser, Subcommand, ValueEnum};

// Define the Chain enum
#[derive(ValueEnum, Clone, Debug)]
pub enum Chain {
    Ethereum,
    BinanceSmartChain,
    Polygon,
    Chiliz,
    Arbitrum,
    // More chains can be added here
}
// Implement the Chain enum
impl Chain {
    pub fn rpc_env_var(&self) -> &'static str {
        match self {
            Chain::Ethereum => "ETH_RPC_URL",
            Chain::BinanceSmartChain => "BSC_RPC_URL",
            Chain::Polygon => "POLYGON_RPC_URL",
            Chain::Chiliz => "CHILIZ_RPC_URL",
            Chain::Arbitrum => "ARBITRUM_RPC_URL",
            // Add more chains here
        }
    }
}

// Define the Cli struct
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, value_enum, default_value = "Ethereum")]
    pub chain: Chain,

    #[arg(short, long)]
    pub token_address: String, // Add token address as an argument

    #[command(subcommand)]
    pub command: Option<Commands>,
}

// Define the Commands enum
#[derive(Subcommand)]
pub enum Commands {
    FetchTransfers {
        #[arg(short, long)]
        blocks_back: u64,
        #[arg(short, long, num_args = 1.., value_delimiter = ',')]
        addresses: Vec<String>,
    },
    FetchTransactionsBetween {
        #[arg(short, long)]
        from: String,
        #[arg(short, long)]
        to: String,
        #[arg(short, long)]
        blocks_back: u64,
    },
}
