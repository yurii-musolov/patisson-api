#![allow(dead_code)]

// Mainnet.
pub const URL_BASE_API_MAINNET_1: &str = "https://api.bybit.com";
pub const URL_BASE_API_MAINNET_2: &str = "https://api.bytick.com";
/// For Netherland users.
pub const URL_BASE_API_MAINNET_3: &str = "https://api.bybit.nl";
/// For Hong Kong users.
pub const URL_BASE_API_MAINNET_4: &str = "https://api.byhkbit.com";
/// For Turkey users.
pub const URL_BASE_API_MAINNET_5: &str = "wss://api.bybit-tr.com";
/// For Kazakhstan users.
pub const URL_BASE_API_MAINNET_6: &str = "wss://api.bybit.kz";

pub const URL_BASE_STREAM_MAINNET_1: &str = "wss://stream.bybit.com";
/// For Turkey users.
pub const URL_BASE_STREAM_MAINNET_2: &str = "wss://stream.bybit-tr.com";
/// For Kazakhstan users.
pub const URL_BASE_STREAM_MAINNET_3: &str = "wss://stream.bybit.kz";

// Testnet.
pub const URL_BASE_API_TESTNET: &str = "https://api-testnet.bybit.com";
pub const URL_BASE_STREAM_TESTNET: &str = "wss://stream-testnet.bybit.com";

// Demo trading.
pub const URL_BASE_API_DEMO_TRADING: &str = "https://api-demo.bybit.com";
pub const URL_BASE_STREAM_DEMO_TRADING: &str = "wss://stream-demo.bybit.com";

// The following HTTP header keys must be used for authentication:
/// API key.
pub const HEADER_X_BAPI_API_KEY: &str = "X-BAPI-API-KEY";
/// UTC timestamp in milliseconds.
pub const HEADER_X_BAPI_TIMESTAMP: &str = "X-BAPI-TIMESTAMP";
/// A signature derived from the request's parameters.
pub const HEADER_X_BAPI_SIGN: &str = "X-BAPI-SIGN";
/// The header for broker users only.
pub const HEADER_X_REFERER: &str = "X-Referer";
/// The header for broker users only.
pub const HEADER_REFERER: &str = "Referer";
/// The header for specify how long an HTTP request is valid (unit in millisecond and default value is 5,000). It is also used to prevent replay attacks..
pub const HEADER_X_BAPI_RECV_WINDOW: &str = "X-BAPI-RECV-WINDOW";
/// Your remaining requests for current endpoint.
pub const HEADER_X_BAPI_LIMIT: &str = "X-Bapi-Limit";
/// Your current limit for current endpoint.
pub const HEADER_X_BAPI_LIMIT_STATUS: &str = "X-Bapi-Limit-Status";
/// The timestamp indicating when your request limit resets if you have exceeded your rate_limit. Otherwise, this is just the current timestamp (it may not exactly match timeNow).
pub const HEADER_X_BAPI_LIMIT_RESET_TIMESTAMP: &str = "X-Bapi-Limit-Reset-Timestamp";
///  To assist in diagnosing advanced network problems. Its value should be unique for each request.
pub const HEADER_CDN_REQUEST_ID: &str = "cdn-request-id";

// Candlestick, orderbook, ticker, platform transaction data, underlying financial rules, risk control rules
pub const PATH_MARKET_KLINE: &str = "/v5/market/kline";
pub const PATH_MARKET_MARK_PRICE_KLINE: &str = "/v5/market/mark-price-kline";
pub const PATH_MARKET_INDEX_PRICE_KLINE: &str = "/v5/market/index-price-kline";
pub const PATH_MARKET_PREMIUM_INDEX_PRICE_KLINE: &str = "/v5/market/premium-index-price-kline";
pub const PATH_MARKET_ORDERBOOK: &str = "/v5/market/orderbook";
pub const PATH_MARKET_TICKERS: &str = "/v5/market/tickers";
pub const PATH_MARKET_FUNDING_HISTORY: &str = "/v5/market/funding/history";
pub const PATH_MARKET_RECENT_TRADE: &str = "/v5/market/recent-trade";
pub const PATH_MARKET_OPEN_INTEREST: &str = "/v5/market/open-interest";
pub const PATH_MARKET_HISTORICAL_VOLATILITY: &str = "/v5/market/historical-volatility";
pub const PATH_MARKET_INSURANCE: &str = "/v5/market/insurance";
pub const PATH_MARKET_INSTRUMENTS_INFO: &str = "/v5/market/instruments-info";
pub const PATH_MARKET_RISK_LIMIT: &str = "/v5/market/risk-limit";
pub const PATH_MARKET_DELIVERY_PRICE: &str = "/v5/market/delivery-price";

// Order management
pub const PATH_ORDER_CREATE: &str = "/v5/order/create";
pub const PATH_ORDER_AMEND: &str = "/v5/order/amend";
pub const PATH_ORDER_CANCEL: &str = "/v5/order/cancel";
pub const PATH_ORDER_REALTIME: &str = "/v5/order/realtime";
pub const PATH_ORDER_CANCEL_ALL: &str = "/v5/order/cancel-all";
pub const PATH_ORDER_HISTORY: &str = "/v5/order/history";
pub const PATH_ORDER_CREATE_BATCH: &str = "/v5/order/create-batch";
pub const PATH_ORDER_AMEND_BATCH: &str = "/v5/order/amend-batch";
pub const PATH_ORDER_CANCEL_BATCH: &str = "/v5/order/cancel-batch";
pub const PATH_ORDER_SPOT_BORROW_CHECK: &str = "/v5/order/spot-borrow-check";

// Position management
pub const PATH_POSITION_LIST: &str = "/v5/position/list";
pub const PATH_POSITION_SET_LEVERAGE: &str = "/v5/position/set-leverage";
pub const PATH_POSITION_SET_RISK_LIMIT: &str = "/v5/position/set-risk-limit";
pub const PATH_POSITION_TRADING_STOP: &str = "/v5/position/trading-stop";
pub const PATH_POSITION_SWITCH_ISOLATED: &str = "/v5/position/switch-isolated";
pub const PATH_POSITION_SWITCH_MODE: &str = "/v5/position/switch-mode";
pub const PATH_POSITION_SET_AUTO_ADD_MARGIN: &str = "/v5/position/set-auto-add-margin";
pub const PATH_POSITION_CLOSED_PNL: &str = "/v5/position/closed-pnl";
pub const PATH_EXECUTION_LIST: &str = "/v5/execution/list";

// Single account operations only – unified funding account, rates, etc.
pub const PATH_ACCOUNT_WALLET_BALANCE: &str = "/v5/account/wallet-balance";
pub const PATH_ACCOUNT_UPGRADE_TO_UTA: &str = "/v5/account/upgrade-to-uta";
pub const PATH_ACCOUNT_BORROW_HISTORY: &str = "/v5/account/borrow-history";
pub const PATH_ACCOUNT_COLLATERAL_INFO: &str = "/v5/account/collateral-info";
pub const PATH_ASSET_COIN_GREEKS: &str = "/v5/asset/coin-greeks";
pub const PATH_ACCOUNT_INFO: &str = "/v5/account/info";
pub const PATH_ACCOUNT_TRANSACTION_LOG: &str = "/v5/account/transaction-log";
pub const PATH_ACCOUNT_SET_MARGIN_MODE: &str = "/v5/account/set-margin-mode";
pub const PATH_ACCOUNT_SET_MARGIN_MODE_DEMO_APPLY_MONEY: &str = "/v5/account/demo-apply-money";

// Operations across multiple accounts – asset management, fund management, etc.
pub const PATH_ASSET_DELIVERY_RECORD: &str = "/v5/asset/delivery-record";
pub const PATH_ASSET_SETTLEMENT_RECORD: &str = "/v5/asset/settlement-record";
pub const PATH_ASSET_TRANSFER_INTER_TRANSFER: &str = "/v5/asset/transfer/inter-transfer";
pub const PATH_ASSET_TRANSFER_QUERY_INTER_TRANSFER_LIST: &str =
    "/v5/asset/transfer/query-inter-transfer-list";
pub const PATH_ASSET_TRANSFER_SAVE_TRANSFER_SUB_MEMBER: &str =
    "/v5/asset/transfer/save-transfer-sub-member";
pub const PATH_ASSET_TRANSFER_UNIVERSAL_TRANSFER: &str = "/v5/asset/transfer/universal-transfer";
pub const PATH_ASSET_TRANSFER_QUERY_UNIVERSAL_TRANSFER_LIST: &str =
    "/v5/asset/transfer/query-universal-transfer-list";
pub const PATH_ASSET_TRANSFER_QUERY_TRANSFER_COIN_LIST: &str =
    "/v5/asset/transfer/query-transfer-coin-list";
pub const PATH_ASSET_TRANSFER_QUERY_SUB_MEMBER_LIST: &str =
    "/v5/asset/transfer/query-sub-member-list";
pub const PATH_ASSET_TRANSFER_QUERY_ACCOUNT_COIN_BALANCE: &str =
    "/v5/asset/transfer/query-account-coin-balance";
pub const PATH_ASSET_TRANSFER_QUERY_ASSET_INFO: &str = "/v5/asset/transfer/query-asset-info";
pub const PATH_ASSET_DEPOSIT_QUERY_ALLOWED_LIST: &str = "/v5/asset/deposit/query-allowed-list";
pub const PATH_ASSET_DEPOSIT_QUERY_RECORD: &str = "/v5/asset/deposit/query-record";
pub const PATH_ASSET_DEPOSIT_QUERY_SUB_MEMBER_RECORD: &str =
    "/v5/asset/deposit/query-sub-member-record";
pub const PATH_ASSET_WITHDRAW_QUERY_RECORD: &str = "/v5/asset/withdraw/query-record";
pub const PATH_ASSET_COIN_QUERY_INFO: &str = "/v5/asset/coin/query-info";
pub const PATH_ASSET_WITHDRAW_CREATE: &str = "/v5/asset/withdraw/create";
pub const PATH_ASSET_WITHDRAW_CANCEL: &str = "/v5/asset/withdraw/cancel";
pub const PATH_ASSET_DEPOSIT_QUERY_ADDRESS: &str = "/v5/asset/deposit/query-address";
pub const PATH_ASSET_DEPOSIT_QUERY_SUB_MEMBER_ADDRESS: &str =
    "/v5/asset/deposit/query-sub-member-address";
pub const PATH_ASSET_EXCHANGE_ORDER_RECORD: &str = "/v5/asset/exchange/order-record";

// Obtain quotes from Leveraged Tokens on Spot, and to exercise purchase and redeem functions
pub const PATH_SPOT_LEVER_TOKEN_INFO: &str = "/v5/spot-lever-token/info";
pub const PATH_SPOT_LEVER_TOKEN_REFERENCE: &str = "/v5/spot-lever-token/reference";
pub const PATH_SPOT_LEVER_TOKEN_PURCHASE: &str = "/v5/spot-lever-token/purchase";
pub const PATH_SPOT_LEVER_TOKEN_REDEEM: &str = "/v5/spot-lever-token/redeem";
pub const PATH_SPOT_LEVER_TOKEN_ORDER_RECORD: &str = "/v5/spot-lever-token/order-record";

// Manage Margin Trading on Spot
pub const PATH_SPOT_MARGIN_TRADE_SWITCH_MODE: &str = "/v5/spot-margin-trade/switch-mode";
pub const PATH_SPOT_MARGIN_TRADE_SET_LEVERAGE: &str = "/v5/spot-margin-trade/set-leverage";
pub const PATH_SPOT_MARGIN_TRADE_SET_PLEDGE_TOKEN: &str = "/v5/spot-margin-trade/set-pledge-token";

// Stream paths.
pub const PATH_PUBLIC_SPOT: &str = "/v5/public/spot";
pub const PATH_PUBLIC_LINEAR: &str = "/v5/public/linear";
pub const PATH_PUBLIC_INVERSE: &str = "/v5/public/inverse";
pub const PATH_PUBLIC_OPTION: &str = "/v5/public/option";
pub const PATH_PRIVATE: &str = "/v5/private";
pub const PATH_TRADE: &str = "/v5/trade";
