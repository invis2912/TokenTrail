# TokenTrail

TokenTrail is a command-line tool for tracking ERC-20 token transfers on various EVM compatible blockchains.

## Features

- Fetch token transfers for specified multiple addresses
- Fetch transactions between two specific addresses
- Support for multiple EVM compatible chains
- Support for any ERC20 Token Address
- Automatically fetches Token Name and Ticker
- Automatically fetches Token Decimals for human-readable token amount

## Installation

1. Ensure you have Rust and Cargo installed.
2. Clone the repository:
   ```
   git clone https://github.com/invis2912/TokenTrail.git
   cd TokenTrail
   ```
3. Run the project:
   ```
   cargo run
   ```
   It will return output based on constants defined in src/constants.rs
```bash
   // Define global constants for default values
pub const DEFAULT_CHAIN: Chain = Chain::Chiliz;
pub const DEFAULT_TOKEN_ADDRESS: &str = "0xFD3C73b3B09D418841dd6Aff341b2d6e3abA433b";
pub const DEFAULT_BLOCKS_BACK: u64 = 110000;
pub const DEFAULT_FROM_ADDRESS: &str = "0x87917D879ba83CE3Ada6e02d49A10c1eC1988062";
pub const DEFAULT_TO_ADDRESS: &str = "0x6C22326A4DE45b089Ea35446eeA23D3966d7e16A";
 ```

4. Build the project for release version:
   ```
   cargo build --release
   ```

   This command compiles your project with optimizations, creating a release-ready binary.

3. Once the build process is complete, you can find the compiled binary in the `target/release` directory.

4. The binary will be named `tokentrail`. You can run it directly from the `target/release` directory:

   ```bash
   ./target/release/tokentrail --chain chiliz --token-address 0xFD3C73b3B09D418841dd6Aff341b2d6e3abA433b fetch-transfers --blocks-back 10000 --addresses 0x87917d879ba83ce3ada6e02d49a10c1ec1988062,0x6c22326a4de45b089ea35446eea23d3966d7e16a,0xcc782d8b7863acf80e3a4fafc0e04d445f5bf711
   ```
   I above command, I have selected chiliz chain for Bar Token to get list of transactions for 3 addresses. 

## Usage

tokentrail [OPTIONS] --chain <CHAIN> --token-address <TOKEN_ADDRESS> <COMMAND> --blocks-back <VALUE> --addresses <WALLET ADDRESS>


### Options

- `--chain <CHAIN>`: Specify the blockchain (e.g. Chiliz, Ethereum, Polygon)
- `--token-address <TOKEN_ADDRESS>`: The address of the ERC-20 token

### Commands

- `fetch-transfers`: Fetch token transfers for specified addresses
- `fetch-transactions-between`: Fetch transactions between two addresses
- `--blocks-back <VALUE>`: Number of blocks backward from current blocks
- `--addresses <WALLET ADDRESS>`: User address 

## Usage Examples

### Fetch Token Transfers for Multiple Addresses

To fetch token transfers for multiple addresses on the Chiliz chain:

   ```bash
   ./target/release/tokentrail --chain chiliz --token-address 0xFD3C73b3B09D418841dd6Aff341b2d6e3abA433b fetch-transfers --blocks-back 10000 --addresses 0x87917d879ba83ce3ada6e02d49a10c1ec1988062,0x6c22326a4de45b089ea35446eea23d3966d7e16a,0xcc782d8b7863acf80e3a4fafc0e04d445f5bf711
   ```

This command:
- Uses the Chiliz chain
- Tracks the token at address `0xFD3C73b3B09D418841dd6Aff341b2d6e3abA433b`
- Fetches transfers from the last 10,000 blocks
- Monitors transfers for three addresses:
  - 0x87917d879ba83ce3ada6e02d49a10c1ec1988062
  - 0x6c22326a4de45b089ea35446eea23d3966d7e16a
  - 0xcc782d8b7863acf80e3a4fafc0e04d445f5bf711


## Demo

The output for fetch-transactions-between two addresses functionality looks like following:

```bash
ajeet@Ajeets-MBP TokenTrail % ./target/release/tokentrail --chain chiliz --token-address 0xFD3C73b3B09D418841dd6Aff341b2d6e3abA433b fetch-transactions-between --from 0x87917D879ba83CE3Ada6e02d49A10c1eC1988062 --to 0x6C22326A4DE45b089Ea35446eeA23D3966d7e16A --blocks-back 50000
 __        __         _                                       _               _____           _                     _____                  _   _ 
 \ \      / /   ___  | |   ___    ___    _ __ ___     ___    | |_    ___     |_   _|   ___   | | __   ___   _ __   |_   _|  _ __    __ _  (_) | |
  \ \ /\ / /   / _ \ | |  / __|  / _ \  | '_ ` _ \   / _ \   | __|  / _ \      | |    / _ \  | |/ /  / _ \ | '_ \    | |   | '__|  / _` | | | | |
   \ V  V /   |  __/ | | | (__  | (_) | | | | | | | |  __/   | |_  | (_) |     | |   | (_) | |   <  |  __/ | | | |   | |   | |    | (_| | | | | |
    \_/\_/     \___| |_|  \___|  \___/  |_| |_| |_|  \___|    \__|  \___/      |_|    \___/  |_|\_\  \___| |_| |_|   |_|   |_|     \__,_| |_| |_|
                                                                                                                                                 

Starting program...
Selected Chain: 
   ____   _       _   _   _       
  / ___| | |__   (_) | | (_)  ____
 | |     | '_ \  | | | | | | |_  /
 | |___  | | | | | | | | | |  / / 
  \____| |_| |_| |_| |_| |_| /___|
                                  

RPC URL for Chiliz: https://chiliz-chain.rpc.thirdweb.com/
Provider created successfully.
Parsing token address...
Token address parsed successfully: 0xfd3c73b3b09d418841dd6aff341b2d6e3aba433b
Token Name:
  _____    ____     ____                                 _                         
 |  ___|  / ___|   | __ )    __ _   _ __    ___    ___  | |   ___    _ __     __ _ 
 | |_    | |       |  _ \   / _` | | '__|  / __|  / _ \ | |  / _ \  | '_ \   / _` |
 |  _|   | |___    | |_) | | (_| | | |    | (__  |  __/ | | | (_) | | | | | | (_| |
 |_|      \____|   |____/   \__,_| |_|     \___|  \___| |_|  \___/  |_| |_|  \__,_|
                                                                                   

Token Ticker:
  ____       _      ____  
 | __ )     / \    |  _ \ 
 |  _ \    / _ \   | |_) |
 | |_) |  / ___ \  |  _ < 
 |____/  /_/   \_\ |_| \_\
                          

Token decimals: 0
Fetching transactions...
Transfers fetched successfully. Count: 4
Event: Transfer
Date: 2024-09-01 16:53:37 UTC
From: 0x87917d879ba83ce3ada6e02d49a10c1ec1988062
To: 0x6c22326a4de45b089ea35446eea23d3966d7e16a
Amount: 1010
Transaction Hash: 0x9e618a34216e232aae2880a12de1a257615178810f3741a67894dbb418880072
Block Hash: 0x1a2302930a529e251a8de8f15959f0875a74db898962e973929cdd979b575dde
Block Number: 16169027
--------------------
Event: Transfer
Date: 2024-09-01 17:09:38 UTC
From: 0x87917d879ba83ce3ada6e02d49a10c1ec1988062
To: 0x6c22326a4de45b089ea35446eea23d3966d7e16a
Amount: 473
Transaction Hash: 0xd37a42b211018cbc6af66c866d8326f659e200a5e2e329c81988878be7f0356d
Block Hash: 0x6c464817c6e444b5beecc5e47327b573ba52a4e89868e364f40e49ecb37129bd
Block Number: 16169329
--------------------
Event: Transfer
Date: 2024-09-01 17:41:38 UTC
From: 0x87917d879ba83ce3ada6e02d49a10c1ec1988062
To: 0x6c22326a4de45b089ea35446eea23d3966d7e16a
Amount: 298
Transaction Hash: 0xd80156a5cfb5881e3d351b6313c439f7e39ade047b0d0b7182318981298c5d86
Block Hash: 0x4c28bf48b7ad7160b2c0bdb3b71ff44ebce6cfda4561d30394ef47c1bf66cd89
Block Number: 16169934
--------------------
Event: Transfer
Date: 2024-09-01 17:44:27 UTC
From: 0x87917d879ba83ce3ada6e02d49a10c1ec1988062
To: 0x6c22326a4de45b089ea35446eea23d3966d7e16a
Amount: 296
Transaction Hash: 0x3ba4839a65263aeab3a89a8b6b12b599c06b924c886efd3f40a9e09ace26d196
Block Hash: 0x720ce0321d17353199dc69be753758b66a052be764040867f2c889852351b8c5
Block Number: 16169985
--------------------
Thank you for using TokenTrail.
```
The output for fetch-transfers for multiple addresses functionality looks like following:
```bash
ajeet@Ajeets-MBP TokenTrail % ./target/release/tokentrail --chain chiliz --token-address 0xFD3C73b3B09D418841dd6Aff341b2d6e3abA433b fetch-transfers --blocks-back 10000 --addresses 0x87917d879ba83ce3ada6e02d49a10c1ec1988062,0x6c22326a4de45b089ea35446eea23d3966d7e16a,0xcc782d8b7863acf80e3a4fafc0e04d445f5bf711
 __        __         _                                       _               _____           _                     _____                  _   _ 
 \ \      / /   ___  | |   ___    ___    _ __ ___     ___    | |_    ___     |_   _|   ___   | | __   ___   _ __   |_   _|  _ __    __ _  (_) | |
  \ \ /\ / /   / _ \ | |  / __|  / _ \  | '_ ` _ \   / _ \   | __|  / _ \      | |    / _ \  | |/ /  / _ \ | '_ \    | |   | '__|  / _` | | | | |
   \ V  V /   |  __/ | | | (__  | (_) | | | | | | | |  __/   | |_  | (_) |     | |   | (_) | |   <  |  __/ | | | |   | |   | |    | (_| | | | | |
    \_/\_/     \___| |_|  \___|  \___/  |_| |_| |_|  \___|    \__|  \___/      |_|    \___/  |_|\_\  \___| |_| |_|   |_|   |_|     \__,_| |_| |_|
                                                                                                                                                 

Starting program...
Selected Chain: 
   ____   _       _   _   _       
  / ___| | |__   (_) | | (_)  ____
 | |     | '_ \  | | | | | | |_  /
 | |___  | | | | | | | | | |  / / 
  \____| |_| |_| |_| |_| |_| /___|
                                  

RPC URL for Chiliz: https://chiliz-chain.rpc.thirdweb.com/
Provider created successfully.
Parsing token address...
Token address parsed successfully: 0xfd3c73b3b09d418841dd6aff341b2d6e3aba433b
Token Name:
  _____    ____     ____                                 _                         
 |  ___|  / ___|   | __ )    __ _   _ __    ___    ___  | |   ___    _ __     __ _ 
 | |_    | |       |  _ \   / _` | | '__|  / __|  / _ \ | |  / _ \  | '_ \   / _` |
 |  _|   | |___    | |_) | | (_| | | |    | (__  |  __/ | | | (_) | | | | | | (_| |
 |_|      \____|   |____/   \__,_| |_|     \___|  \___| |_|  \___/  |_| |_|  \__,_|
                                                                                   

Token Ticker:
  ____       _      ____  
 | __ )     / \    |  _ \ 
 |  _ \    / _ \   | |_) |
 | |_) |  / ___ \  |  _ < 
 |____/  /_/   \_\ |_| \_\
                          

Token decimals: 0
Address 0 parsed successfully: 0x87917d879ba83ce3ada6e02d49a10c1ec1988062
Address 1 parsed successfully: 0x6c22326a4de45b089ea35446eea23d3966d7e16a
Address 2 parsed successfully: 0xcc782d8b7863acf80e3a4fafc0e04d445f5bf711
Address list parsed successfully. Total addresses: 3
Fetching transfers...
Transfers fetched successfully. Count: 3
Event: Transfer
Date: 2024-09-03 01:03:06 UTC
From: 0x87917d879ba83ce3ada6e02d49a10c1ec1988062
To: 0xe7969e270667cd43bfe8640fa0187029e71535f2
Amount: 285
Transaction Hash: 0x8ee467848fe35d87cd0947777a58d615d7b73ec1bb07269ee844e61209c9569a
Block Hash: 0x7204dfd267c01ab32b66e698b83dcb244678560e0fde9b312cae9142f50c7adb
Block Number: 16207015
--------------------
Event: Transfer
Date: 2024-09-03 03:20:03 UTC
From: 0xe505579e56d316cb2be9cef81f2cdfa6dc5c9e14
To: 0x87917d879ba83ce3ada6e02d49a10c1ec1988062
Amount: 106
Transaction Hash: 0xb65a6c5b5e5f6af58d703cf97c3f8673e0ed23fc887c51dbaf7d811991ca1794
Block Hash: 0x490bb2f6c49be03ed7bc6d8f996b4ccf48339018c20cb77f53a525260ddfe220
Block Number: 16209754
--------------------
Event: Transfer
Date: 2024-09-03 08:40:03 UTC
From: 0x42e6cc9c13611fc8e98b5504f85d0fb1f62c433a
To: 0x87917d879ba83ce3ada6e02d49a10c1ec1988062
Amount: 3039
Transaction Hash: 0x4e4be438d66520643dfdf214e3fd7ebbaca6c173cdbec730e044d1bb6a616ea3
Block Hash: 0x5790becfba819515c1af30a39bee80ed8f5bdd4d822f9068c053ce5e5dbde6d0
Block Number: 16216154
--------------------
Thank you for using TokenTrail.
```
The following results are for --chain ethereum and PEPE token address -

```bash
ajeet@Ajeets-MBP TokenTrail % ./target/release/tokentrail --chain ethereum --token-address 0x6982508145454Ce325dDbE47a25d4ec3d2311933 fetch-transactions-between --from 0x8d10c7E20fDd521fb300172Defa8440547544128 --to 0x45326B06216DC69F8498eF211b39F4abF8047184 --blocks-back 30000
 __        __         _                                       _               _____           _                     _____                  _   _ 
 \ \      / /   ___  | |   ___    ___    _ __ ___     ___    | |_    ___     |_   _|   ___   | | __   ___   _ __   |_   _|  _ __    __ _  (_) | |
  \ \ /\ / /   / _ \ | |  / __|  / _ \  | '_ ` _ \   / _ \   | __|  / _ \      | |    / _ \  | |/ /  / _ \ | '_ \    | |   | '__|  / _` | | | | |
   \ V  V /   |  __/ | | | (__  | (_) | | | | | | | |  __/   | |_  | (_) |     | |   | (_) | |   <  |  __/ | | | |   | |   | |    | (_| | | | | |
    \_/\_/     \___| |_|  \___|  \___/  |_| |_| |_|  \___|    \__|  \___/      |_|    \___/  |_|\_\  \___| |_| |_|   |_|   |_|     \__,_| |_| |_|
                                                                                                                                                 

Starting program...
Selected Chain: 
  _____   _     _                                              
 | ____| | |_  | |__     ___   _ __    ___   _   _   _ __ ___  
 |  _|   | __| | '_ \   / _ \ | '__|  / _ \ | | | | | '_ ` _ \ 
 | |___  | |_  | | | | |  __/ | |    |  __/ | |_| | | | | | | |
 |_____|  \__| |_| |_|  \___| |_|     \___|  \__,_| |_| |_| |_|
                                                               

RPC URL for Ethereum: https://ethereum.rpc.thirdweb.com/
Provider created successfully.
Parsing token address...
Token address parsed successfully: 0x6982508145454ce325ddbe47a25d4ec3d2311933
Token Name:
  ____                        
 |  _ \    ___   _ __     ___ 
 | |_) |  / _ \ | '_ \   / _ \
 |  __/  |  __/ | |_) | |  __/
 |_|      \___| | .__/   \___|
                |_|           

Token Ticker:
  ____    _____   ____    _____ 
 |  _ \  | ____| |  _ \  | ____|
 | |_) | |  _|   | |_) | |  _|  
 |  __/  | |___  |  __/  | |___ 
 |_|     |_____| |_|     |_____|
                                

Token decimals: 18
Fetching transactions...
Transfers fetched successfully. Count: 1
Event: Transfer
Date: 2024-09-02 14:42:35 UTC
From: 0x8d10c7e20fdd521fb300172defa8440547544128
To: 0x45326b06216dc69f8498ef211b39f4abf8047184
Amount: 39512313
Transaction Hash: 0x48430980ab5d3a726dd4b2656f1e17e2b4fabdd4636e3ca1e0fb2966f2554740
Block Hash: 0x0b0ba40bb6688ffe63e5142b2e5bfbb12eb069a5e737bc7a000da0228d6aafcd
Block Number: 20663551
--------------------
Thank you for using TokenTrail.

```

## Environment Variables
Private or public RPC-URLS for chains can be updated in .env file.

## Quick Setup and Run 
Considering all other installations are complete.
Run following on terminal - 

For fetch-transactions-between on chiliz for BAR Token:
```bash
cargo run -- --chain chiliz --token-address 0xFD3C73b3B09D418841dd6Aff341b2d6e3abA433b fetch-transactions-between --from 0x87917D879ba83CE3Ada6e02d49A10c1eC1988062 --to 0x6C22326A4DE45b089Ea35446eeA23D3966d7e16A --blocks-back 30000
```

For fetch-transfers on chiliz for BAR Token:
```bash
cargo run -- --chain chiliz --token-address 0xFD3C73b3B09D418841dd6Aff341b2d6e3abA433b fetch-transfers --blocks-back 10000 --addresses 0x87917d879ba83ce3ada6e02d49a10c1ec1988062,0x6c22326a4de45b089ea35446eea23d3966d7e16a,0xcc782d8b7863acf80e3a4fafc0e04d445f5bf711
```

For fetch-transactions-between on ethereum for PEPE token:
```bash
cargo run -- --chain ethereum --token-address 0x6982508145454Ce325dDbE47a25d4ec3d2311933 fetch-transactions-between --from 0x8d10c7E20fDd521fb300172Defa8440547544128 --to 0x45326B06216DC69F8498eF211b39F4abF8047184 --blocks-back 30000
```


## Test Case
There are Test Cases for two functions - test_fetch_transactions_between and test_fetch_transfers.
Run following command in terminal for running Test Case.

```bash
cargo test
```
Output will be as following:
```bash
ajeet@Ajeets-MBP TokenTrail % cargo test
   Compiling TokenTrail v0.1.0 (/Users/ajeet/Documents/Work/TokenTransfers/TokenTrail)

warning: `TokenTrail` (bin "TokenTrail" test) generated 1 warning
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.78s
     Running unittests src/main.rs (target/debug/deps/TokenTrail-4ae03f12af7a8faf)

running 2 tests
test tests::tests::test_fetch_transactions_between ... ok
test tests::tests::test_fetch_transfers ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.18s
```
More Test cases needs to be developed to make sure code is running as expected considering different inputs, scenarios and conditions.

## Future Code Improvements
1. Features:
- Add More EVM and Non-Evm chains
- Add More RPC URLS for Local, Private and Public Endpoints
- Front-End Development for Easy to Use Interaction and Visualization
- Add range of `From` and `To` Block functionality to get specific blocks Transaction Detail
2. Security:
- Environment Variables: Ensure sensitive data like RPC URLs and API keys are securely managed using environment variables and limit API calls. Restrict API calls only to required Resources.
- Error Handling: Improve error handling to avoid exposing sensitive information. Ensure all errors are properly logged.
- Dependency Updates: Regularly update dependencies to patch known vulnerabilities.
3. Scalability: 
- Asynchronous Operations: Ensure all I/O-bound operations are asynchronous to improve scalability. This is already partially implemented using `tokio`.
- Caching: Implement caching for frequently accessed data, such as token metadata, to reduce the load on the provider. This is partially implemented for `token decimal`parsing by caching data locally using HashMap.

4. Maintainability:
- Modularization: Further modularize the code to separate concerns. For example, separate the CLI parsing, business logic, and data access layers.
- Documentation: Add comprehensive documentation and comments to make the codebase easier to understand and maintain.
- Testing: Increase test coverage, including unit tests and integration tests, to ensure code reliability. This is partially implemented for `test_fetch_transactions_between` and `test_fetch_transfers` functions.
5. Speed:
- Parallel Processing: Use parallel processing where possible, such as fetching multiple token transfers concurrently.
- Efficient Data Structures: Use efficient data structures and algorithms to handle large datasets.
- Batched Requests: Implement batched requests if possible to reduce overhead and latency.
6. Use of 3rd party tools:
- Logging: Integrate a robust logging framework like `log` or `slog` to capture and manage logs effectively.
- Monitoring: Use monitoring tools like Prometheus and Grafana to monitor the application's performance and health.
- Indexing Protocol: Use an indexing protocol like The Graph or Covalent APIs for efficient data querying.
7. Modern approach:
- Web3 Tools: Use Tools and Libraries like Foundry, Mocha, Chai, etc for smart contract interaction and testing.
- Blockhcain Query: Use GraphQL for querying blockchain data more flexibly and efficiently.



## License

[MIT License](LICENSE)



## Appendix

1. List of Public RPC URLS : https://www.comparenodes.com/library/public-endpoints
2. Chiliz Blockchain Explorer : https://chiliscan.com
3. Ethereum Blockchain Explorer: https://etherscan.io
