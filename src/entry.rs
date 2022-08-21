use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct FullEntry {
    pub data: DataObject
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DataObject {
    pub id: u32,
    pub name: String,
    pub slug: String,
    pub numMarketPairs: u32,
    pub marketPairs: Vec<MarketPair>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MarketPair {
    pub exchangeId: u32,
    pub exchangeName: String,
    pub exchangeSlug: String,
    pub outlierDetected: u32,
    pub priceExcluded: u32,
    pub volumeExcluded: u32,
    pub marketId: u32,
    pub marketPair: String,
    pub category: String,
    pub marketUrl: String,
    pub marketScore: String,
    pub marketReputation: u32,
    pub baseSymbol: String,
    pub baseCurrencyId: u32,
    pub baseCurrencyName: String,
    pub baseCurrencySlug: String,
    pub quoteSymbol: String,
    pub quoteCurrencyId: u32,
    pub price: f64,
    pub volumeUsd: f64,
    pub effectiveLiquidity: f64,
    pub lastUpdated: String,
    pub volumeBase: f64,
    pub volumeQuote: f64,
    pub feeType: String,
    pub depthUsdNegativeTwo: f64,
    pub depthUsdPositiveTwo: f64,
}