# **client-rust-fix**
## Rust Client for trading on [power.trade](https://power.trade) crypto exchange

[![Rust](https://github.com/laisee/client-rust-fix/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/laisee/client-rust-fix/actions/workflows/rust.yml)
[![CI](https://github.com/laisee/client-rust-fix/actions/workflows/ci.yml/badge.svg)](https://github.com/laisee/client-rust-fix/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT) 
[![Clippy](https://github.com/laisee/client-rust-fix/actions/workflows/clippy.yml/badge.svg?branch=main)](https://github.com/laisee/client-rust-fix/actions/workflows/clippy.yml)
![MSRV](https://img.shields.io/badge/MSRV-1.86.0-orange)

Rust client for power.trade Fix protocol.
Implements authentication and basic order management (add single order, cancel single order) and RFQ (Request For Quote) functionality.

## Features
* Establishes a TLS FIX session and performs logon to the Power.Trade exchange.
* Signs JWTs with your API keys to authenticate FIX connections.
* Provides utilities for building FIX messages including NewOrderSingle, OrderCancelRequest, and RFQ messages.
* Scenario-driven main executable that can place or cancel orders or send RFQ messages based on environment variables.
* Integration test validates the structure of generated FIX messages without connecting to the exchange.

See [list of issues](https://github.com/laisee/client-rust-fix/issues) for the planned set of enhancements and features.  

See [here](https://power-trade.github.io/api-docs-source/fix_order_entry.html) for Power.Trade Single Leg Order Fix message specification (Fix MsgType='D')

See [here](https://power-trade.github.io/api-docs-source/fix_order_entry.html#_introduction) for Power.Trade Drop Copy Fix message specification (Fix MsgType='8')

Power.Trade API home page can be found [here](https://support.power.trade/api/api-overview)

## Getting Started
1. Install Rust on device where client will be running. 

   See [Rust installation instructions](https://www.rust-lang.org/tools/install) for instructions on installation using Rustup.
   
   See [alternative installation methods](https://forge.rust-lang.org/infra/other-installation-methods.html) for other installation methods.

2. Check that Rustup has installed and configured **1.86** as the default version by typing the following command in a console/terminal window
  ```
  rustc --version
  ```
    the version displayed should be: "rustc 1.86.0 (default)"
3. Copy the sample env file (".env.example") to create a file for the Test environment
   ```
   cp .env.example .env.test
   ```
4. Open the new `.env.test` file and update the settings for your Test API environment

   Required environment variables include:

   - `PT_API_KEY` - Power.Trade API key
   - `PT_WS_API_KEY` - WebSocket API key
   - `PT_WS_API_SECRET` - WebSocket API secret in PEM format
   - `PT_SERVER` - FIX server hostname
   - `PT_WS_SERVER` - WebSocket endpoint
   - `PT_PEM_FILE` - path to your client PEM file
   - `PT_PUBKEY_FILE` - path to your public certificate
   - `PT_SCENARIO` - scenario to run (`ORDER`, `ORDERS`, `RFQ_QUOTE`, `RFQ_LISTEN`)
   - `PT_LISTEN_EPOCH` - number of epochs to listen for responses (for RFQ_LISTEN)
   - `PT_PUBLISH_EPOCH` - number of epochs to listen after publishing (for RFQ_QUOTE)
   - `PT_HEARTBEAT_COUNT` - number of heartbeats to send
   - `PT_HEARTBEAT_INTERVAL` - seconds between heartbeats
   - `PT_SYMBOL` - trading symbol (e.g., "BTC-USD")
   - `PT_PRICE` - order price
   - `PT_QUANTITY` - order quantity
   - `PT_SIDE` - order side ("BUY" or "SELL")
   - `PT_ORDER_TYPE` - order type ("LIMIT" or "MARKET")
   - `PT_CANCEL_ORDER` - whether to cancel the order after placing it ("true" or "false")
   - `PT_LOG_FILE` - path to log file
   - `PT_TARGET_COMP_ID` - target company ID for FIX messages

5. Build the project

   ```
   cargo build
   ```

6. Run tests to ensure everything is configured correctly

   ```
   cargo test
   ```

7. Save the file and run the client on the Test environment (same process can be followed to create production env file '.env.prod')
    
   n.b. Rust client runtime environment is set on command line as a parameter for the --env flag with value of 'development', 'test', 'production' 
   ```
   cargo run -- --env test
   ```
8. Review console output and log files (see `app.log` in the same folder) to view client activity

## Available Scenarios

The client supports several scenarios that can be configured using the `PT_SCENARIO` environment variable:

- `ORDER` - Send a single order based on the configured parameters
- `ORDERS` - Send multiple orders (currently sends a single order, but can be extended)
- `RFQ_QUOTE` - Send an RFQ quote request and listen for responses
- `RFQ_LISTEN` - Subscribe to RFQ updates and listen for incoming quotes

## Command Line Arguments

The client supports the following command line arguments:

- `--env <ENVIRONMENT>` - Set the runtime environment (development, test, production)

Example:
```
cargo run -- --env test
```

## Logging

The client logs information to both the console and a log file specified by the `PT_LOG_FILE` environment variable. The log file contains detailed information about the client's operation, including sent and received FIX messages.

## Development

To contribute to the project, please follow these steps:

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests and linting: `cargo test && cargo clippy`
5. Submit a pull request

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.
