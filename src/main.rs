#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![feature(str_split_once)]
#![feature(or_patterns)]

mod config;
mod identifier;
// mod cli;
// mod currency;

#[tokio::main]
async fn main() {
    // let opts = cli::load();
    // let list = currency::Rates::load(opts.currency_path,
    // &opts.base_currency).await.unwrap();
}
