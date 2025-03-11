use bybit_sdk::{
    Category, Client, GetInstrumentsInfoParams, GetKLinesParams, GetTickersParams, Interval,
    URL_BASE_API_MAINNET_1,
};
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let category = Category::Linear;
    let symbol = String::from("BTCUSDT");
    let linear_client = Client::new(URL_BASE_API_MAINNET_1);

    let response = linear_client
        .get_kline(GetKLinesParams {
            category: category.clone(),
            symbol: symbol.clone(),
            interval: Interval::Minute1,
            start: None,
            end: None,
            limit: Some(2),
        })
        .await?;
    println!("{response:#?}");

    let response = linear_client
        .get_tickers(GetTickersParams {
            category: category.clone(),
            symbol: Some(symbol.clone()),
            base_coin: None,
            exp_date: None,
        })
        .await?;
    println!("{response:#?}");

    let response = linear_client
        .get_instruments_info(GetInstrumentsInfoParams {
            category,
            symbol: Some(symbol),
            status: None,
            base_coin: None,
            limit: None,
            cursor: None,
        })
        .await?;
    println!("{response:#?}");

    Ok(())
}
