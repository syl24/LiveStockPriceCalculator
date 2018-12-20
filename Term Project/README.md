# CS311-Project

Repo for CPSC 311 Term Project

Requirements:
To install Rust refer to Rust’s official installation [documentations](https://www.rust-lang.org/en-US/install.html). Our source code currently has only been tested on Windows, other platforms may behave unexpectedly.

To run the application, go to the base directory of the project and run the command `cargo run <equity>` where equity is a stock symbol from the list of exchanges NASDAQ, NYSE, AMEX. You can find equities following this [link](https://www.nasdaq.com/screening/company-list.aspx).

Refer to [reqwest](https://docs.rs/reqwest/0.9.5/reqwest/) and [reqwest github](https://github.com/seanmonstar/reqwest) for HTTP Rust Client. Most of the code/setup for the fetch function is borrowed from there. 
The Stock Time Series Data API from [Alpha Vantage](https://www.alphavantage.co/documentation/) offers real time stock series data derived from the current trading day. We will be using the intraday time series option with one minute intervals to gather the last 90 data points of a certain equity.
