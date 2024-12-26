use bybit_sdk::{
    Category, Client, GetKLinesParams, GetTickersParams, Interval, URL_BASE_API_MAINNET_1,
};
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let symbol = String::from("BTCUSDT");
    let linear_client = Client::new(URL_BASE_API_MAINNET_1);

    let response = linear_client
        .get_kline(GetKLinesParams {
            category: Category::Linear,
            symbol: symbol.clone(),
            interval: Interval::Minute1,
            start: None,
            end: None,
            limit: Some(2),
        })
        .await?;
    println!("Response kline linear {symbol}: {response:#?}");

    let response = linear_client
        .get_tickers(GetTickersParams {
            category: Category::Linear,
            symbol: Some(symbol.clone()),
            base_coin: None,
            exp_date: None,
        })
        .await?;
    println!("Response ticker linear {symbol}: {response:#?}");

    Ok(())
}
