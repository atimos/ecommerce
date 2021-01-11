use chrono::{serde::ts_seconds, DateTime, Utc};
use rust_decimal::Decimal;
use serde_json::from_reader;
use std::{
    collections::HashMap,
    fs::File,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub enum Error {
    OpenFile(std::io::Error, PathBuf),
    Parse(serde_json::Error),
}

pub struct Rate {
    base_currency: CurrencyName,
    rate: Decimal,
}

pub enum CurrencyName {
}

#[derive(Debug)]
pub struct Rates {
    path: PathBuf,
    inner: Inner,
}

#[derive(serde::Deserialize, Debug)]
struct Inner {
    #[serde(rename = "timestamp")]
    #[serde(with = "ts_seconds")]
    time: DateTime<Utc>,
    rates: HashMap<String, Decimal>,
    base: String,
}

impl Rates {
    pub async fn load(path: PathBuf, base: &str) -> Result<Self, Error> {
        fetch(&base, &path)?;

        Ok(Self {
            inner: File::open(&path)
                .map_err(|err| Error::OpenFile(err, path.clone()))
                .and_then(|file| from_reader(file).map_err(Error::Parse))?,
            path,
        })
    }

    pub async fn reload(&mut self) -> Result<(), Error> {
        *self = Self::load(self.path.clone(), &self.inner.base).await?;
        Ok(())
    }

    pub fn rate(&self, currency: &str) -> Option<&Decimal> {
        self.inner.rates.get(currency)
    }

    pub fn convert_to(&self, currency: &str, value: &Decimal) -> Option<Decimal> {
        self.inner.rates.get(currency).map(|rate| value * rate)
    }

    pub fn convert_from(&self, currency: &str, value: &Decimal) -> Option<Decimal> {
        self.inner.rates.get(currency).map(|rate| value / rate)
    }
}

fn fetch(base: &str, path: &Path) -> Result<(), Error> {
    Ok(())
}
