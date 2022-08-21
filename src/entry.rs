#[derive(Serialize, Deserialize, Debug)]
struct FullEntry {
    data: DataObject
}

#[derive(Serialize, Deserialize, Debug)]
struct DataObject {
    id: u32,
    name: String,
    slug: String,
    numMarketPairs: u32,
    marketPairs: Vec<MarketPair>
}

#[derive(Serialize, Deserialize, Debug)]
struct MarketPair {
    exchangeId: u32,
    exchangeName: String,
    exchangeSlug: String,
    outlierDetected: u32,
    priceExcluded: u32,
    volumeExcluded: u32,
    marketId: 40454,
    marketPair: String,
    category: String,
    marketUrl: String,
    marketScore: u32,
    marketReputation: u32,
    baseSymbol: String,
    baseCurrencyId: u32,
    baseCurrencyName: String,
    baseCurrencySlug: String,
    quoteSymbol: String,
    quoteCurrencyId: u32,
    price: f64,
    volumeUsd: f64,
    effectiveLiquidity: f64,
    lastUpdated: String,
    volumeBase: f64,
    volumeQuote: f64,
    feeType: String,
    depthUsdNegativeTwo: f64,
    depthUsdPositiveTwo: f64,
}