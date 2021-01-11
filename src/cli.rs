use clap::Clap;
use std::path::PathBuf;

#[derive(Clap)]
pub struct Opts {
    /// Point to where the currency data is stored
    #[clap(short, long, default_value = "./data/currency.json")]
    pub currency_path: PathBuf,
    #[clap(short, long, default_value = "USD")]
    pub base_currency: String,
}

pub fn load() -> Opts {
    Opts::parse()
}
