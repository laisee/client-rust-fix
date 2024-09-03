# **client-rust-fix**
## Rust Client for trading on [https://power.trade](power.trade) crypto exchange

[![Rust](https://github.com/laisee/client-rust-fix/actions/workflows/rust.yml/badge.svg)](https://github.com/laisee/client-rust-fix/actions/workflows/rust.yml) 
[![CI](https://github.com/laisee/client-rust-fix/actions/workflows/ci.yml/badge.svg)](https://github.com/laisee/client-rust-fix/actions/workflows/ci.yml) 
![Clippy](https://github.com/laisee/client-rust-fix/actions/workflows/clippy.yml/badge.svg)
[![Verify dependencies](https://github.com/laisee/client-rust-fix/actions/workflows/dependencies.yml/badge.svg)](https://github.com/laisee/client-rust-fix/actions/workflows/dependencies.yml) [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)![MSRV](https://img.shields.io/badge/MSRV-1.80.0-orange)


Rust client for power.trade Fix protocol. 
Implements authentication and basic order management(add single order, cancel single order). 

See [list of issues](https://github.com/laisee/client-rust-fix/issues) for the planned set of enhancements and features.  

See [here](https://power-trade.github.io/api-docs-source/fix_order_entry.html) for Power.Trade Single Leg Order Fix message specification(Fix MsgType='D')

See [here](https://power-trade.github.io/api-docs-source/fix_order_entry.html#_introduction) for Power.Trade Drop Copy Fix message specification(Fix MsgType='8')

Power.Trade API home page can be found [here](https://support.power.trade/api/api-overview)

## Getting Started
1. Install Rust on device where client will be running. 

   See [https://www.rust-lang.org/tools/install](here) for instructions on installation using Rustup.
   
   See [https://forge.rust-lang.org/infra/other-installation-methods.html](here) for other installation methods.

3. Check that Rustup has installed and configured 1.80 as the default version by typing following command in a console/terminal window
  ```
  rustc --version
  ```
    the version displayed should be: "rustc 1.80.0 (default)"
3. Copy the sample env file(".env.example") to create a file for Test environment
   ```
   cp .env.example .env.test
   ```
4. Open the new .env file (".env.test") and update the settings for Test API env
   
6. Save the file and run client on Test environment.
    
   n.b. Rust client runtime environment is set on command line as a parameter for the --env flag with value of 'development', 'test', 'production' 
   ```
   cargo run -- --env test
   ```
8. Review console output and log files (see 'app.log' in same folder) to view client activity
   
   
