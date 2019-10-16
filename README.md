# The Algos of Wall Street: CS311-Project

**Documentation**:

* [Proposal](docs/project-proposal.pdf)

* [Background Report](docs/project-background-report.pdf)

* [Plan-Proof](docs/project-plan-proof.pdf)

* [Final](docs/project-final.pdf)

**Intro**:

A stock trading application that retrieves real time stock prices of an equity to determine a buy or sell signal. The program uses [AlphaVantage Intraday API](https://www.alphavantage.co/documentation/) to retrieve live stock information and utilizes the [mean reversion model](https://medium.com/the-ocean-trade/algorithmic-trading-101-lesson-2-data-strategy-design-and-mean-reversion-25c19a003328) to determine the signal type. This program is valuable as it will benefit trading firms by strengthening their decision to either buy or sell a stock. A demo of the program can be seen below.

[![algos gif](docs/demo.svg)](https://asciinema.org/a/pG0yXK0V9XEdHym1s7VowsquF?autoplay=1)

**Requirements**:

To install Rust refer to Rust’s official installation [documentations](https://www.rust-lang.org/en-US/install.html). Our source code currently has only been tested on Windows, other platforms may behave unexpectedly.

**Running the application**:

To run the application, go to the base directory of the project and run the command `cargo run <equity>` where equity is a stock symbol from the list of exchanges NASDAQ, NYSE, AMEX. You can find equities following this [link](https://www.nasdaq.com/screening/company-list.aspx).

Refer to [reqwest](https://docs.rs/reqwest/0.9.5/reqwest/) and [reqwest github](https://github.com/seanmonstar/reqwest) for HTTP Rust Client. Most of the code for the fetch function is based on their implementation. 

The Stock Time Series Data API from [Alpha Vantage](https://www.alphavantage.co/documentation/) offers real time stock series data derived from the current trading day. We will be using the intraday time series option with one minute intervals to gather the last 90 data points of an equity. **For demonstration purposes we are using 90 minute and 30 minute averages instead of days**.
