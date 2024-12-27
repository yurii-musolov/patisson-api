use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};

use crate::{
    application::{GetCandlesParams, IApp},
    presentation::{
        from_api_exchange, from_api_interval, from_api_schema, to_api_candle, to_api_symbol,
        APICandle, APIExchange, APIInterval, APISchema, APISymbol, GetCandlesQuery,
        GetSymbolsQuery,
    },
};

pub async fn get_symbols<T>(
    State(application): State<T>,
    Path((exchange, schema)): Path<(APIExchange, APISchema)>,
    Query(params): Query<GetSymbolsQuery>,
) -> Result<Json<Vec<APISymbol>>, StatusCode>
where
    T: IApp,
{
    if !schema.is_valid_with(&exchange) {
        return Err(StatusCode::BAD_REQUEST);
    }

    let symbols = application
        .get_symbols(
            from_api_exchange(&exchange),
            from_api_schema(&schema),
            params.symbol,
        )
        .await;
    let symbols = symbols.iter().map(to_api_symbol).collect();
    Ok(Json(symbols))
}

pub async fn get_candles<T>(
    Path((exchange, schema, symbol, interval)): Path<(APIExchange, APISchema, String, String)>,
    Query(query): Query<GetCandlesQuery>,
    State(application): State<T>,
) -> Result<Json<Vec<APICandle>>, StatusCode>
where
    T: IApp,
{
    let interval = APIInterval::from_str(&interval);
    if !schema.is_valid_with(&exchange) || interval.is_none() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let candles = application
        .get_candles(
            from_api_exchange(&exchange),
            from_api_schema(&schema),
            GetCandlesParams {
                symbol,
                interval: from_api_interval(&interval.unwrap()),
                start: query.start,
                end: query.end,
                limit: query.limit,
            },
        )
        .await;
    let candles = candles.iter().map(to_api_candle).collect();
    Ok(Json(candles))
}
