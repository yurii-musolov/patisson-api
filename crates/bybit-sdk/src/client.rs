use reqwest::{self, Client as RClient, Method};

use crate::{
    api::{GetKLinesParams, GetTickersParams, KLine, Response, Ticker},
    url::{PATH_MARKET_KLINE, PATH_MARKET_RECENT_TRADE, PATH_MARKET_TICKERS},
    GetTradesParams, Trade,
};

#[derive(Clone)]
pub struct Client<'a> {
    base_url: &'a str,
}

impl<'a> Client<'a> {
    pub fn new(base_url: &'a str) -> Self {
        Self { base_url }
    }

    pub async fn get_kline(
        &self,
        params: GetKLinesParams,
    ) -> Result<Response<KLine>, Box<dyn std::error::Error>> {
        let url = format!("{}{}", self.base_url, PATH_MARKET_KLINE);

        let client = RClient::builder().build()?;
        let request_builder = client.request(Method::GET, url).query(&params);

        let response = request_builder.send().await?;
        let res = response.json().await?;

        Ok(res)
    }

    pub async fn get_tickers(
        &self,
        params: GetTickersParams,
    ) -> Result<Response<Ticker>, Box<dyn std::error::Error>> {
        let url = format!("{}{}", self.base_url, PATH_MARKET_TICKERS);

        let client = RClient::builder().build()?;
        let request_builder = client.request(Method::GET, url).query(&params);

        let response = request_builder.send().await?;
        let res = response.json().await?;

        Ok(res)
    }

    pub async fn get_public_recent_trading_history(
        &self,
        params: GetTradesParams,
    ) -> Result<Response<Trade>, Box<dyn std::error::Error>> {
        let url = format!("{}{}", self.base_url, PATH_MARKET_RECENT_TRADE);

        let client = RClient::builder().build()?;
        let request_builder = client.request(Method::GET, url).query(&params);

        let response = request_builder.send().await?;
        let res = response.json().await?;

        Ok(res)
    }
}
