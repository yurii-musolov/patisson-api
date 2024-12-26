use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use crate::{
    application::IApp,
    presentation::{
        from_api_exchange, from_api_schema, to_api_candle, to_api_symbol, APICandle, APIExchange,
        APISchema, APISymbol,
    },
};

pub async fn get_symbols<T>(
    State(application): State<T>,
    Path((exchange, schema)): Path<(APIExchange, APISchema)>,
) -> Result<Json<Vec<APISymbol>>, StatusCode>
where
    T: IApp,
{
    let symbols = application
        .get_symbols(from_api_exchange(exchange), from_api_schema(schema))
        .await;
    let symbols = symbols.iter().map(to_api_symbol).collect();
    Ok(Json(symbols))
}

pub async fn get_candles<T>(
    State(application): State<T>,
    Path((exchange, schema, symbol)): Path<(APIExchange, APISchema, String)>,
) -> Result<Json<Vec<APICandle>>, StatusCode>
where
    T: IApp,
{
    let candles = application
        .get_candles(from_api_exchange(exchange), from_api_schema(schema), symbol)
        .await;
    let candles = candles.iter().map(to_api_candle).collect();
    Ok(Json(candles))
}
