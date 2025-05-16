use serde::{Deserialize, Serialize};
use serde_aux::prelude::{
    deserialize_number_from_string as number,
    deserialize_option_number_from_string as option_number,
};

use crate::{
    AutoAddMargin, CancelType, Category, ContractType, CopyTrading, CreateType, CurAuctionPhase,
    Innovation, Interval, OcoTriggerBy, OrderStatus, OrderType, Pair, PlaceType, PositionIdx,
    PositionStatus, RejectReason, Side, SmpType, Status, StopOrderType, TimeInForce, TpslMode,
    TradeMode, TriggerBy, TriggerDirection,
};

#[derive(Debug, Deserialize, PartialEq)]
pub struct Response<T> {
    #[serde(rename = "retCode")]
    pub ret_code: i32,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: T,
    pub time: i64,
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: RetExtInfo,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct RetExtInfo {}

#[derive(Serialize)]
pub struct GetKLinesParams {
    pub category: Category,
    pub symbol: String,
    pub interval: Interval,
    pub start: Option<u64>,
    pub end: Option<u64>,
    pub limit: Option<u64>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "category")]
pub enum KLine {
    #[serde(rename = "inverse")]
    Inverse { symbol: String, list: Vec<KLineRow> },
    #[serde(rename = "linear")]
    Linear { symbol: String, list: Vec<KLineRow> },
    #[serde(rename = "option")]
    Option { symbol: String, list: Vec<KLineRow> },
    #[serde(rename = "spot")]
    Spot { symbol: String, list: Vec<KLineRow> },
}

#[derive(Debug, Deserialize)]
pub struct KLineRow {
    #[serde(rename = "startTime", deserialize_with = "number")]
    pub start_time: u64, // Start time of the candle (ms)
    #[serde(rename = "openPrice", deserialize_with = "number")]
    pub open_price: f64, // Open price
    #[serde(rename = "highPrice", deserialize_with = "number")]
    pub high_price: f64, // Highest price
    #[serde(rename = "lowPrice", deserialize_with = "number")]
    pub low_price: f64, // Lowest price
    #[serde(rename = "closePrice", deserialize_with = "number")]
    pub close_price: f64, // Close price. Is the last traded price when the candle is not closed
    #[serde(rename = "volume", deserialize_with = "number")]
    pub volume: f64, // Trade volume. Unit of contract: pieces of contract. Unit of spot: quantity of coins
    #[serde(rename = "turnover", deserialize_with = "number")]
    pub turnover: f64, // Turnover. Unit of figure: quantity of quota coin
}

#[derive(Serialize)]
pub struct GetInstrumentsInfoParams {
    pub category: Category,
    pub symbol: Option<String>,
    pub status: Option<Status>,
    pub base_coin: Option<String>,
    pub limit: Option<i64>,
    pub cursor: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(tag = "category")]
pub enum InstrumentsInfo {
    #[serde(rename = "inverse", rename_all = "camelCase")]
    Inverse {
        next_page_cursor: String,
        list: Vec<AllCategoriesInstrumentsInfo>, // TODO: Rename AllCategoriesInstrumentsInfo to InverseLinearInstrumentsInfo.
    },
    #[serde(rename = "linear", rename_all = "camelCase")]
    Linear {
        next_page_cursor: String,
        list: Vec<AllCategoriesInstrumentsInfo>,
    },
    #[serde(rename = "option", rename_all = "camelCase")]
    Option {
        next_page_cursor: String,
        list: Vec<AllCategoriesInstrumentsInfo>, // TODO: Rewrite this.
    },
    #[serde(rename = "spot", rename_all = "camelCase")]
    Spot {
        next_page_cursor: Option<String>,
        list: Vec<SpotInstrumentsInfo>,
    },
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AllCategoriesInstrumentsInfo {
    pub symbol: String,
    pub contract_type: ContractType,
    pub status: Status,
    pub base_coin: String,
    pub quote_coin: String,
    #[serde(deserialize_with = "number")]
    pub launch_time: i64,
    #[serde(deserialize_with = "number")]
    pub delivery_time: i64,
    #[serde(deserialize_with = "option_number")]
    pub delivery_fee_rate: Option<f64>,
    #[serde(deserialize_with = "number")]
    pub price_scale: i64,
    pub leverage_filter: LeverageFilter,
    pub price_filter: PriceFilter,
    pub lot_size_filter: LotSizeFilter,
    pub unified_margin_trade: bool,
    pub funding_interval: i64,
    pub settle_coin: String,
    pub copy_trading: CopyTrading,
    #[serde(deserialize_with = "number")]
    pub upper_funding_rate: f64,
    #[serde(deserialize_with = "number")]
    pub lower_funding_rate: f64,
    pub risk_parameters: RiskParameters,
    pub is_pre_listing: bool,
    pub pre_listing_info: Option<PreListingInfo>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SpotInstrumentsInfo {
    pub symbol: String,         // Symbol name
    pub base_coin: String,      // Base coin
    pub quote_coin: String,     // Quote coin
    pub innovation: Innovation, // Whether or not this is an innovation zone token. 0: false, 1: true
    pub status: Status,         // Instrument status
    pub margin_trading: String, // Margin trade symbol or not
    // This is to identify if the symbol support margin trading under different account modes
    // You may find some symbols not supporting margin buy or margin sell, so you need to go to Collateral Info (UTA) to check if that coin is borrowable
    pub st_tag: String, // Whether or not it has an special treatment label. 0: false, 1: true
    pub lot_size_filter: SpotLotSizeFilter, // Size attributes
    pub price_filter: SpotPriceFilter, // Price attributes
    pub risk_parameters: RiskParameters, // Risk parameters for limit order price, refer to announcement
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LeverageFilter {
    #[serde(deserialize_with = "number")]
    pub min_leverage: f64,
    #[serde(deserialize_with = "number")]
    pub max_leverage: f64,
    #[serde(deserialize_with = "number")]
    pub leverage_step: f64,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PriceFilter {
    #[serde(deserialize_with = "number")]
    pub min_price: f64,
    #[serde(deserialize_with = "number")]
    pub max_price: f64,
    #[serde(deserialize_with = "number")]
    pub tick_size: f64,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SpotPriceFilter {
    #[serde(deserialize_with = "number")]
    pub tick_size: f64, // The step to increase/reduce order price
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LotSizeFilter {
    #[serde(deserialize_with = "number")]
    pub min_notional_value: f64,
    #[serde(deserialize_with = "number")]
    pub max_order_qty: f64,
    #[serde(deserialize_with = "number")]
    pub max_mkt_order_qty: f64,
    #[serde(deserialize_with = "number")]
    pub min_order_qty: f64,
    #[serde(deserialize_with = "number")]
    pub qty_step: f64,
    #[serde(deserialize_with = "number")]
    pub post_only_max_order_qty: f64,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SpotLotSizeFilter {
    #[serde(deserialize_with = "number")]
    pub base_precision: f64, // The precision of base coin
    #[serde(deserialize_with = "number")]
    pub quote_precision: f64, // The precision of quote coin
    #[serde(deserialize_with = "number")]
    pub min_order_qty: f64, // Minimum order quantity
    #[serde(deserialize_with = "number")]
    pub max_order_qty: f64, // Maximum order quantity
    #[serde(deserialize_with = "number")]
    pub min_order_amt: f64, // Minimum order amount
    #[serde(deserialize_with = "number")]
    pub max_order_amt: f64, // Maximum order amount
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RiskParameters {
    #[serde(deserialize_with = "number")]
    pub price_limit_ratio_x: f64,
    #[serde(deserialize_with = "number")]
    pub price_limit_ratio_y: f64,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PreListingInfo {
    pub cur_auction_phase: CurAuctionPhase,
    pub phases: Vec<Phase>,
    pub auction_fee_info: AuctionFeeInfo,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Phase {
    pub phase: CurAuctionPhase,
    #[serde(deserialize_with = "option_number")]
    pub start_time: Option<i64>,
    #[serde(deserialize_with = "option_number")]
    pub end_time: Option<i64>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AuctionFeeInfo {
    #[serde(deserialize_with = "number")]
    pub auction_fee_rate: f64,
    #[serde(deserialize_with = "number")]
    pub taker_fee_rate: f64,
    #[serde(deserialize_with = "number")]
    pub maker_fee_rate: f64,
}

#[derive(Serialize)]
pub struct GetTickersParams {
    pub category: Category,
    pub symbol: Option<String>,
    pub base_coin: Option<String>,
    pub exp_date: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "category")]
pub enum Ticker {
    #[serde(rename = "inverse")]
    Inverse { list: Vec<LinearInverseTicker> },
    #[serde(rename = "linear")]
    Linear { list: Vec<LinearInverseTicker> },
    #[serde(rename = "option")]
    Option { list: Vec<OptionTicker> },
    #[serde(rename = "spot")]
    Spot { list: Vec<SpotTicker> },
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinearInverseTicker {
    pub symbol: String, // Symbol name
    #[serde(deserialize_with = "number")]
    pub last_price: f64, // Last price
    #[serde(deserialize_with = "number")]
    pub mark_price: f64, // Mark price
    #[serde(deserialize_with = "number")]
    pub index_price: f64, // Index price
    #[serde(deserialize_with = "number")]
    pub prev_price24h: f64, // Market price 24 hours ago
    #[serde(deserialize_with = "number")]
    pub price24h_pcnt: f64, // Percentage change of market price in the last 24 hours
    #[serde(deserialize_with = "number")]
    pub high_price24h: f64, // The highest price in the last 24 hours
    #[serde(deserialize_with = "number")]
    pub low_price24h: f64, // The lowest price in the last 24 hours
    #[serde(deserialize_with = "number")]
    pub prev_price1h: f64, // Market price an hour ago
    #[serde(deserialize_with = "number")]
    pub open_interest: f64, // Open interest size
    #[serde(deserialize_with = "number")]
    pub open_interest_value: f64, // Open interest value
    #[serde(deserialize_with = "number")]
    pub turnover24h: f64, // Turnover for 24h
    #[serde(deserialize_with = "number")]
    pub volume24h: f64, // Volume for 24h
    #[serde(deserialize_with = "number")]
    pub funding_rate: f64, // Funding rate
    #[serde(deserialize_with = "number")]
    pub next_funding_time: u64, // Next funding timestamp (ms)
    pub predicted_delivery_price: String, // Predicated delivery price. It has value when 30 min before delivery
    pub basis_rate: String, // Basis rate. Unique field for inverse futures & USDC futures
    pub basis: String,      // Basis. Unique field for inverse futures & USDC futures
    pub delivery_fee_rate: String, // Delivery fee rate. Unique field for inverse futures & USDC futures
    pub delivery_time: String, // Delivery date time (UTC+0). Unique field for inverse futures & USDC futures
    #[serde(deserialize_with = "number")]
    pub bid1_price: f64, // Best bid price
    #[serde(deserialize_with = "number")]
    pub bid1_size: f64, // Best bid size
    #[serde(deserialize_with = "number")]
    pub ask1_price: f64, // Best ask price
    #[serde(deserialize_with = "number")]
    pub ask1_size: f64, // Best ask size
    pub pre_open_price: String, // Estimated pre-market contract open price. The value is meaningless when entering continuous trading phase.
    pub pre_qty: String, // Estimated pre-market contract open qty. The value is meaningless when entering continuous trading phase.
    pub cur_pre_listing_phase: String, // Enum: NotStarted, Finished, CallAuction, CallAuctionNoCancel, CrossMatching, ContinuousTrading.
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionTicker {
    pub symbol: String, // Symbol name
    #[serde(deserialize_with = "number")]
    pub bid1_price: f64, // Best bid price
    #[serde(deserialize_with = "number")]
    pub bid1_size: f64, // Best bid size
    #[serde(deserialize_with = "number")]
    pub bid1_iv: f64, // Best bid iv
    #[serde(deserialize_with = "number")]
    pub ask1_price: f64, // Best ask price
    #[serde(deserialize_with = "number")]
    pub ask1_size: f64, // Best ask size
    #[serde(deserialize_with = "number")]
    pub ask1_iv: f64, // Best ask iv
    #[serde(deserialize_with = "number")]
    pub last_price: f64, // Last price
    #[serde(deserialize_with = "number")]
    pub high_price24h: f64, // The highest price in the last 24 hours
    #[serde(deserialize_with = "number")]
    pub low_price24h: f64, // The lowest price in the last 24 hours
    #[serde(deserialize_with = "number")]
    pub mark_price: f64, // Mark price
    #[serde(deserialize_with = "number")]
    pub index_price: f64, // Index price
    #[serde(deserialize_with = "number")]
    pub mark_iv: f64, // Mark price iv
    #[serde(deserialize_with = "number")]
    pub underlying_price: f64, // Underlying price
    #[serde(deserialize_with = "number")]
    pub open_interest: f64, // Open interest size
    #[serde(deserialize_with = "number")]
    pub turnover24h: f64, // Turnover for 24h
    #[serde(deserialize_with = "number")]
    pub volume24h: f64, // Volume for 24h
    #[serde(deserialize_with = "number")]
    pub total_volume: f64, // Total volume
    #[serde(deserialize_with = "number")]
    pub total_turnover: f64, // Total turnover
    #[serde(deserialize_with = "number")]
    pub delta: f64, // Delta
    #[serde(deserialize_with = "number")]
    pub gamma: f64, // Gamma
    #[serde(deserialize_with = "number")]
    pub vega: f64, // Vega
    #[serde(deserialize_with = "number")]
    pub theta: f64, // Theta
    #[serde(deserialize_with = "number")]
    pub predicted_delivery_price: f64, // Predicated delivery price. It has value when 30 min before delivery
    #[serde(deserialize_with = "number")]
    pub change24h: f64, // The change in the last 24 hous
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpotTicker {
    pub symbol: String, // Symbol name
    #[serde(deserialize_with = "number")]
    pub bid1_price: f64, // Best bid price
    #[serde(deserialize_with = "number")]
    pub bid1_size: f64, // Best bid size
    #[serde(deserialize_with = "number")]
    pub ask1_price: f64, // Best ask price
    #[serde(deserialize_with = "number")]
    pub ask1_size: f64, // Best ask size
    #[serde(deserialize_with = "number")]
    pub last_price: f64, // Last price
    #[serde(deserialize_with = "number")]
    pub prev_price24h: f64, // Market price 24 hours ago
    #[serde(deserialize_with = "number")]
    pub price24h_pcnt: f64, // Percentage change of market price in the last 24 hours
    #[serde(deserialize_with = "number")]
    pub high_price24h: f64, // The highest price in the last 24 hours
    #[serde(deserialize_with = "number")]
    pub low_price24h: f64, // The lowest price in the last 24 hours
    #[serde(deserialize_with = "number")]
    pub turnover24h: f64, // Turnover for 24h
    #[serde(deserialize_with = "number")]
    pub volume24h: f64, // Volume for 24h
    // USD index price
    // - used to calculate USD value of the assets in Unified account
    // - non-collateral margin coin returns ""
    // - Only those trading pairs like "XXX/USDT" or "XXX/USDC" have the value
    #[serde(deserialize_with = "number")]
    pub usd_index_price: f64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTradesParams {
    pub category: Category,
    // required for spot/linear/inverse
    // optional for option
    pub symbol: Option<String>,
    // Apply to option only
    // If the field is not passed, return BTC data by default
    pub base_coin: Option<String>,
    // optionType	false	string	Option type. Call or Put. Apply to option only
    pub option_type: Option<u64>,
    // spot: [1,60], default: 60
    // others: [1,1000], default: 500
    pub limit: Option<u64>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "category")]
pub enum Trade {
    #[serde(rename = "inverse")]
    Inverse { list: Vec<InverseLinearSpotTrade> },
    #[serde(rename = "linear")]
    Linear { list: Vec<InverseLinearSpotTrade> },
    #[serde(rename = "option")]
    Option { list: Vec<OptionTrade> },
    #[serde(rename = "spot")]
    Spot { list: Vec<InverseLinearSpotTrade> },
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InverseLinearSpotTrade {
    pub exec_id: String,
    pub symbol: String,
    #[serde(deserialize_with = "number")]
    pub price: f64,
    #[serde(deserialize_with = "number")]
    pub size: f64,
    pub side: Side,
    #[serde(deserialize_with = "number")]
    pub time: u64,
    pub is_block_trade: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionTrade {
    pub exec_id: String,
    pub symbol: String,
    #[serde(deserialize_with = "number")]
    pub price: f64,
    #[serde(deserialize_with = "number")]
    pub size: f64,
    pub side: Side,
    #[serde(deserialize_with = "number")]
    pub time: u64,
    pub is_block_trade: bool,
    #[serde(rename = "mP", deserialize_with = "number")]
    pub mark_price: f64,
    #[serde(rename = "iP", deserialize_with = "number")]
    pub index_price: f64,
    #[serde(rename = "mIv", deserialize_with = "number")]
    pub mark_iv: f64,
    #[serde(rename = "iv", deserialize_with = "number")]
    pub iv: f64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOpenClosedOrdersParams {
    /// Product type
    /// UTA2.0, UTA1.0: linear, inverse, spot, option
    /// classic account: linear, inverse, spot
    category: String,
    /// Symbol name, like BTCUSDT, uppercase only. For linear, either symbol, baseCoin, settleCoin is required
    symbol: Option<String>,
    /// Base coin, uppercase only
    /// Supports linear, inverse & option
    /// option: it returns all option open orders by default
    base_coin: Option<String>,
    /// Settle coin, uppercase only
    /// linear: either symbol, baseCoin or settleCoin is required
    /// spot: not supported
    /// option: USDT or USDC
    settle_coin: Option<String>,
    /// Order ID
    order_id: Option<String>,
    /// User customised order ID
    order_link_id: Option<String>,
    /// 0(default): UTA2.0, UTA1.0, classic account query open status orders (e.g., New, PartiallyFilled) only
    /// 1: UTA2.0, UTA1.0(except inverse)
    /// 2: UTA1.0(inverse), classic account
    /// Query a maximum of recent 500 closed status records are kept under each account each category (e.g., Cancelled, Rejected, Filled orders).
    /// If the Bybit service is restarted due to an update, this part of the data will be cleared and accumulated again, but the order records will still be queried in order history
    /// openOnly param will be ignored when query by orderId or orderLinkId
    /// Classic spot: not supported
    open_only: Option<i64>,
    /// Order: active order, StopOrder: conditional order for Futures and Spot, tpslOrder: spot TP/SL order, OcoOrder: Spot oco order, BidirectionalTpslOrder: Spot bidirectional TPSL order
    /// classic account spot: return Order active order by default
    /// Others: all kinds of orders by default
    order_filter: Option<String>,
    /// Limit for data size per page. [1, 50]. Default: 20
    limit: Option<i64>,
    /// Cursor. Use the nextPageCursor token from the response to retrieve the next page of the result set
    cursor: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    /// Order ID
    pub order_id: String,
    /// User customised order ID
    pub order_link_id: String,
    /// Paradigm block trade ID
    pub block_trade_id: String,
    /// Symbol name
    pub symbol: String,
    /// Order price
    #[serde(deserialize_with = "number")]
    pub price: f64,
    /// Order qty
    #[serde(deserialize_with = "number")]
    pub qty: f64,
    /// Side. Buy,Sell
    pub side: Side,
    /// Whether to borrow. Unified spot only. 0: false, 1: true. Classic spot is not supported, always 0
    pub is_leverage: bool,
    /// Position index. Used to identify positions in different position modes.
    pub position_idx: PositionIdx,
    /// Order status
    pub order_status: OrderStatus,
    /// Order create type
    /// Only for category=linear or inverse
    /// Spot, Option do not have this key
    pub create_type: Option<CreateType>,
    /// Cancel type
    pub cancel_type: CancelType,
    /// Reject reason. Classic spot is not supported
    pub reject_reason: RejectReason,
    /// Average filled price
    /// UTA: returns "" for those orders without avg price
    /// classic account: returns "0" for those orders without avg price, and also for those orders have partilly filled but cancelled at the end
    pub avg_price: String,
    /// The remaining qty not executed. Classic spot is not supported
    #[serde(deserialize_with = "number")]
    pub leaves_qty: f64,
    /// The estimated value not executed. Classic spot is not supported
    #[serde(deserialize_with = "number")]
    pub leaves_value: f64,
    /// Cumulative executed order qty
    #[serde(deserialize_with = "number")]
    pub cum_exec_qty: f64,
    /// Cumulative executed order value. Classic spot is not supported
    #[serde(deserialize_with = "number")]
    pub cum_exec_value: f64,
    /// Cumulative executed trading fee. Classic spot is not supported
    #[serde(deserialize_with = "number")]
    pub cum_exec_fee: f64,
    /// Time in force
    pub time_in_force: TimeInForce,
    /// Order type. Market,Limit. For TP/SL order, it means the order type after triggered
    pub order_type: OrderType,
    /// Stop order type
    pub stop_order_type: StopOrderType,
    /// Implied volatility
    pub order_iv: String,
    /// The unit for qty when create Spot market orders for UTA account. baseCoin, quoteCoin
    pub market_unit: String,
    /// Trigger price. If stopOrderType=TrailingStop, it is activate price. Otherwise, it is trigger price
    #[serde(deserialize_with = "number")]
    pub trigger_price: f64,
    /// Take profit price
    #[serde(deserialize_with = "number")]
    pub take_profit: f64,
    /// Stop loss price
    #[serde(deserialize_with = "number")]
    pub stop_loss: f64,
    /// TP/SL mode, Full: entire position for TP/SL. Partial: partial position tp/sl. Spot does not have this field, and Option returns always ""
    pub tpsl_mode: Option<TpslMode>,
    /// The trigger type of Spot OCO order.OcoTriggerByUnknown, OcoTriggerByTp, OcoTriggerByBySl. Classic spot is not supported
    pub oco_trigger_by: OcoTriggerBy,
    /// The limit order price when take profit price is triggered
    #[serde(deserialize_with = "number")]
    pub tp_limit_price: f64,
    /// The limit order price when stop loss price is triggered
    #[serde(deserialize_with = "number")]
    pub sl_limit_price: f64,
    /// The price type to trigger take profit
    pub tp_trigger_by: TriggerBy,
    /// The price type to trigger stop loss
    pub sl_trigger_by: TriggerBy,
    /// Trigger direction. 1: rise, 2: fall
    pub trigger_direction: TriggerDirection,
    /// The price type of trigger price
    pub trigger_by: TriggerBy,
    /// Last price when place the order, Spot is not applicable
    #[serde(deserialize_with = "number")]
    pub last_price_on_created: f64,
    /// Last price when place the order, Spot has this field only
    #[serde(deserialize_with = "number")]
    pub base_price: f64,
    /// Reduce only. true means reduce position size
    pub reduce_only: bool,
    /// Close on trigger.
    pub close_on_trigger: bool,
    /// Place type, option used. iv, price
    pub place_type: PlaceType,
    /// SMP execution type
    pub smp_type: SmpType,
    /// Smp group ID. If the UID has no group, it is 0 by default
    #[serde(deserialize_with = "number")]
    pub smp_group: i64,
    /// The counterparty's orderID which triggers this SMP execution
    pub smp_order_id: String,
    /// Order created timestamp (ms)
    pub created_time: String,
    /// Order updated timestamp (ms)
    pub updated_time: String,
}

impl Order {
    pub fn is_open_status(&self) -> bool {
        self.order_status.is_open()
    }
    pub fn is_closed_status(&self) -> bool {
        self.order_status.is_closed()
    }
}

pub fn spot_fee_currency(side: Side, is_maker_order: bool, maker_fee_rate: f64) -> Pair {
    if maker_fee_rate >= 0.0 {
        match side {
            Side::Buy => Pair::Base,
            Side::Sell => Pair::Quote,
        }
    } else if is_maker_order {
        match side {
            Side::Buy => Pair::Quote,
            Side::Sell => Pair::Base,
        }
    } else {
        match side {
            Side::Buy => Pair::Base,
            Side::Sell => Pair::Quote,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPositionInfo {
    /// Product type
    /// UTA2.0, UTA1.0: linear, inverse, option
    /// Classic account: linear, inverse
    pub category: Category,
    /// Symbol name, like BTCUSDT, uppercase only
    /// If symbol passed, it returns data regardless of having position or not.
    /// If symbol=null and settleCoin specified, it returns position size greater than zero.
    pub symbol: Option<String>,
    /// Base coin, uppercase only. option only. Return all option positions if not passed
    pub base_coin: Option<String>,
    /// Settle coin
    /// linear: either symbol or settleCoin is required. symbol has a higher priority
    pub settle_coin: Option<String>,
    /// Limit for data size per page. [1, 200]. Default: 20
    pub limit: Option<i64>,
    /// Cursor. Use the nextPageCursor token from the response to retrieve the next page of the result set
    pub cursor: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PositionInfo {
    /// Product type
    pub category: Category,
    /// Refer to the cursor request parameter
    pub next_page_cursor: String,
    pub list: Vec<Position>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    /// Position idx, used to identify positions in different position modes
    /// 0: One-Way Mode
    /// 1: Buy side of both side mode
    /// 2: Sell side of both side mode
    /// riskId:Snteger, /// Risk tier ID
    /// for portfolio margin mode, this field returns 0, which means risk limit rules are invalid
    pub position_idx: PositionIdx,
    /// Risk limit value
    /// for portfolio margin mode, this field returns 0, which means risk limit rules are invalid
    #[serde(deserialize_with = "number")]
    pub risk_limit_value: f64,
    /// Symbol name
    pub symbol: String,
    /// Position side. Buy: long, Sell: short
    /// one-way mode: classic & UTA1.0(inverse), an empty position returns None.
    /// UTA2.0(linear, inverse) & UTA1.0(linear): either one-way or hedge mode returns an empty string "" for an empty position.
    pub side: Side,
    /// Position size, always positive
    #[serde(deserialize_with = "number")]
    pub size: f64,
    /// Average entry price
    /// For USDC Perp & Futures, it indicates average entry price, and it will not be changed with 8-hour session settlement
    #[serde(deserialize_with = "number")]
    pub avg_price: f64,
    /// Position value
    #[serde(deserialize_with = "number")]
    pub position_value: f64,
    /// Trade mode
    /// Classic & UTA1.0(inverse): 0: cross-margin, 1: isolated margin
    /// UTA2.0, UTA1.0(execpt inverse): deprecated, always 0, check Get Account Info to know the margin mode
    pub trade_mode: TradeMode,
    /// Whether to add margin automatically when using isolated margin mode
    /// 0: false
    /// 1: true
    pub auto_add_margin: AutoAddMargin,
    /// Position status. Normal, Liq, Adl
    pub position_status: PositionStatus,
    /// Position leverage
    /// for portfolio margin mode, this field returns "", which means leverage rules are invalid
    #[serde(deserialize_with = "option_number")]
    pub leverage: Option<f64>,
    /// Mark price
    #[serde(deserialize_with = "number")]
    pub mark_price: f64,
    /// Position liquidation price
    /// UTA2.0(isolated margin), UTA1.0(isolated margin), UTA1.0(inverse), Classic account:
    /// it is the real price for isolated and cross positions, and keeps "" when liqPrice <= minPrice or liqPrice >= maxPrice
    /// UTA2.0(Cross margin), UTA1.0(Cross margin):
    /// it is an estimated price for cross positions(because the unified mode controls the risk rate according to the account), and keeps "" when liqPrice <= minPrice or liqPrice >= maxPrice
    /// this field is empty for Portfolio Margin Mode, and no liquidation price will be provided
    #[serde(deserialize_with = "option_number")]
    pub liq_price: Option<f64>,
    /// Bankruptcy price
    #[serde(deserialize_with = "number")]
    pub bust_price: f64,
    /// Initial margin
    /// Classic & UTA1.0(inverse): ignore this field
    /// UTA portfolio margin mode, it returns ""
    #[serde(deserialize_with = "number")]
    pub position_i_m: f64,
    /// Maintenance margin
    /// Classic & UTA1.0(inverse): ignore this field
    /// UTA portfolio margin mode, it returns ""
    #[serde(deserialize_with = "number")]
    pub position_m_m: f64,
    /// Position margin
    /// Classic & UTA1.0(inverse) can refer to this field to get the position initial margin plus position closing fee
    #[serde(deserialize_with = "number")]
    pub position_balance: f64,
    /// Take profit price
    #[serde(deserialize_with = "number")]
    pub take_profit: f64,
    /// Stop loss price
    #[serde(deserialize_with = "number")]
    pub stop_loss: f64,
    /// Trailing stop (The distance from market price)
    #[serde(deserialize_with = "number")]
    pub trailing_stop: f64,
    /// USDC contract session avg price, it is the same figure as avg entry price shown in the web UI
    #[serde(deserialize_with = "option_number")]
    pub session_avg_price: Option<f64>,
    /// Delta
    pub delta: Option<String>,
    /// Gamma
    pub gamma: Option<String>,
    /// Vega
    pub vega: Option<String>,
    /// Theta
    pub theta: Option<String>,
    /// Unrealised PnL
    #[serde(deserialize_with = "number")]
    pub unrealised_pnl: f64,
    /// The realised PnL for the current holding position
    #[serde(deserialize_with = "number")]
    pub cur_realised_pnl: f64,
    /// Cumulative realised pnl
    /// Futures & Perps: it is the all time cumulative realised P&L
    /// Option: always "", meaningless
    #[serde(deserialize_with = "number")]
    pub cum_realised_pnl: f64,
    /// Auto-deleverage rank indicator. What is Auto-Deleveraging?
    pub adl_rank_indicator: i64,
    /// Timestamp of the first time a position was created on this symbol (ms)
    #[serde(deserialize_with = "number")]
    pub created_time: i64,
    /// Position updated timestamp (ms)
    #[serde(deserialize_with = "number")]
    pub updated_time: i64,
    /// Cross sequence, used to associate each fill and each position update
    /// Different symbols may have the same seq, please use seq + symbol to check unique
    /// Returns "-1" if the symbol has never been traded
    /// Returns the seq updated by the last transaction when there are settings like leverage, risk limit
    pub seq: i64,
    /// Useful when Bybit lower the risk limit
    /// true: Only allowed to reduce the position. You can consider a series of measures, e.g., lower the risk limit, decrease leverage or reduce the position, add margin, or cancel orders, after these operations, you can call confirm new risk limit endpoint to check if your position can be removed the reduceOnly mark
    /// false: There is no restriction, and it means your position is under the risk when the risk limit is systematically adjusted
    /// Only meaningful for isolated margin & cross margin of USDT Perp, USDC Perp, USDC Futures, Inverse Perp and Inverse Futures, meaningless for others
    pub is_reduce_only: bool,
    /// Useful when Bybit lower the risk limit
    /// When isReduceOnly=true: the timestamp (ms) when the MMR will be forcibly adjusted by the system
    /// When isReduceOnly=false: the timestamp when the MMR had been adjusted by system
    /// It returns the timestamp when the system operates, and if you manually operate, there is no timestamp
    /// Keeps "" by default, if there was a lower risk limit system adjustment previously, it shows that system operation timestamp
    /// Only meaningful for isolated margin & cross margin of USDT Perp, USDC Perp, USDC Futures, Inverse Perp and Inverse Futures, meaningless for others
    pub mmr_sys_updated_time: Option<String>,
    /// Useful when Bybit lower the risk limit
    /// When isReduceOnly=true: the timestamp (ms) when the leverage will be forcibly adjusted by the system
    /// When isReduceOnly=false: the timestamp when the leverage had been adjusted by system
    /// It returns the timestamp when the system operates, and if you manually operate, there is no timestamp
    /// Keeps "" by default, if there was a lower risk limit system adjustment previously, it shows that system operation timestamp
    /// Only meaningful for isolated margin & cross margin of USDT Perp, USDC Perp, USDC Futures, Inverse Perp and Inverse Futures, meaningless for others
    pub leverage_sys_updated_time: Option<String>,
    /// deprecated, always "Full"
    pub tpsl_mode: String,
}

#[cfg(test)]
mod tests {
    use crate::common::deserialize_slice;

    use super::*;

    #[test]
    fn deserialize_response_instruments_info_trading_linear() {
        // Official USDT Perpetual instrument structure
        let json = r#"{
            "category": "linear",
            "list": [
                {
                    "symbol": "BTCUSDT",
                    "contractType": "LinearPerpetual",
                    "status": "Trading",
                    "baseCoin": "BTC",
                    "quoteCoin": "USDT",
                    "launchTime": "1585526400000",
                    "deliveryTime": "0",
                    "deliveryFeeRate": "",
                    "priceScale": "2",
                    "leverageFilter": {
                        "minLeverage": "1",
                        "maxLeverage": "100.00",
                        "leverageStep": "0.01"
                    },
                    "priceFilter": {
                        "minPrice": "0.10",
                        "maxPrice": "1999999.80",
                        "tickSize": "0.10"
                    },
                    "lotSizeFilter": {
                        "maxOrderQty": "1190.000",
                        "minOrderQty": "0.001",
                        "qtyStep": "0.001",
                        "postOnlyMaxOrderQty": "1190.000",
                        "maxMktOrderQty": "500.000",
                        "minNotionalValue": "5"
                    },
                    "unifiedMarginTrade": true,
                    "fundingInterval": 480,
                    "settleCoin": "USDT",
                    "copyTrading": "both",
                    "upperFundingRate": "0.00375",
                    "lowerFundingRate": "-0.00375",
                    "isPreListing": false,
                    "preListingInfo": null,
                    "riskParameters": {
                        "priceLimitRatioX": "0.01",
                        "priceLimitRatioY": "0.02"
                    }
                }
            ],
            "nextPageCursor": ""
        }"#;
        let message: InstrumentsInfo = deserialize_slice(json.as_bytes()).unwrap();
        let expected = InstrumentsInfo::Linear {
            next_page_cursor: String::from(""),
            list: vec![AllCategoriesInstrumentsInfo {
                symbol: String::from("BTCUSDT"),
                contract_type: ContractType::LinearPerpetual,
                status: Status::Trading,
                base_coin: String::from("BTC"),
                quote_coin: String::from("USDT"),
                launch_time: 1585526400000,
                delivery_time: 0,
                delivery_fee_rate: None,
                price_scale: 2,
                leverage_filter: LeverageFilter {
                    min_leverage: 1.0,
                    max_leverage: 100.00,
                    leverage_step: 0.01,
                },
                price_filter: PriceFilter {
                    min_price: 0.10,
                    max_price: 1999999.80,
                    tick_size: 0.10,
                },
                lot_size_filter: LotSizeFilter {
                    min_notional_value: 5.0,
                    max_order_qty: 1190.000,
                    max_mkt_order_qty: 500.000,
                    min_order_qty: 0.001,
                    qty_step: 0.001,
                    post_only_max_order_qty: 1190.000,
                },
                unified_margin_trade: true,
                funding_interval: 480,
                settle_coin: String::from("USDT"),
                copy_trading: CopyTrading::Both,
                upper_funding_rate: 0.00375,
                lower_funding_rate: -0.00375,
                risk_parameters: RiskParameters {
                    price_limit_ratio_x: 0.01,
                    price_limit_ratio_y: 0.02,
                },
                is_pre_listing: false,
                pre_listing_info: None,
            }],
        };
        assert_eq!(message, expected);
    }

    #[test]
    fn deserialize_response_instruments_info_trading_spot() {
        // Spot
        let json = r#"{
            "retCode": 0,
            "retMsg": "OK",
            "result": {
                "category": "spot",
                "list": [
                {
                    "symbol": "BTCUSDT",
                    "baseCoin": "BTC",
                    "quoteCoin": "USDT",
                    "innovation": "0",
                    "status": "Trading",
                    "marginTrading": "utaOnly",
                    "stTag": "0",
                    "lotSizeFilter": {
                        "basePrecision": "0.000001",
                        "quotePrecision": "0.0000001",
                        "minOrderQty": "0.000011",
                        "maxOrderQty": "83",
                        "minOrderAmt": "5",
                        "maxOrderAmt": "8000000"
                    },
                    "priceFilter": {
                        "tickSize": "0.1"
                    },
                    "riskParameters": {
                        "priceLimitRatioX": "0.01",
                        "priceLimitRatioY": "0.02"
                    }
                }
                ]
            },
            "retExtInfo": {},
            "time": 1746213108077
        }"#;
        let message: Response<InstrumentsInfo> = deserialize_slice(json.as_bytes()).unwrap();
        let expected = Response {
            ret_code: 0,
            ret_msg: String::from("OK"),
            result: InstrumentsInfo::Spot {
                next_page_cursor: None,
                list: vec![SpotInstrumentsInfo {
                    symbol: String::from("BTCUSDT"),
                    status: Status::Trading,
                    base_coin: String::from("BTC"),
                    quote_coin: String::from("USDT"),
                    risk_parameters: RiskParameters {
                        price_limit_ratio_x: 0.01,
                        price_limit_ratio_y: 0.02,
                    },
                    innovation: Innovation::False,
                    margin_trading: String::from("utaOnly"), // TODO: Rewrite.
                    st_tag: String::from("0"),               // TODO: Rewrite.
                    lot_size_filter: SpotLotSizeFilter {
                        base_precision: 0.000001,
                        quote_precision: 0.0000001,
                        min_order_qty: 0.000011,
                        max_order_qty: 83.0,
                        min_order_amt: 5.0,
                        max_order_amt: 8000000.0,
                    },
                    price_filter: SpotPriceFilter { tick_size: 0.1 },
                }],
            },
            time: 1746213108077,
            ret_ext_info: RetExtInfo {},
        };
        assert_eq!(message, expected);
    }

    #[test]
    fn deserialize_response_instruments_info_pre_launch() {
        // Pre-market Perpetual instrument structure
        let json = r#"{
        "category": "linear",
        "list": [
            {
                "symbol": "BIOUSDT",
                "contractType": "LinearPerpetual",
                "status": "PreLaunch",
                "baseCoin": "BIO",
                "quoteCoin": "USDT",
                "launchTime": "1735032510000",
                "deliveryTime": "0",
                "deliveryFeeRate": "",
                "priceScale": "4",
                "leverageFilter": {
                    "minLeverage": "1",
                    "maxLeverage": "5.00",
                    "leverageStep": "0.01"
                },
                "priceFilter": {
                    "minPrice": "0.0001",
                    "maxPrice": "1999.9998",
                    "tickSize": "0.0001"
                },
                "lotSizeFilter": {
                    "maxOrderQty": "70000",
                    "minOrderQty": "1",
                    "qtyStep": "1",
                    "postOnlyMaxOrderQty": "70000",
                    "maxMktOrderQty": "14000",
                    "minNotionalValue": "5"
                },
                "unifiedMarginTrade": true,
                "fundingInterval": 480,
                "settleCoin": "USDT",
                "copyTrading": "none",
                "upperFundingRate": "0.05",
                "lowerFundingRate": "-0.05",
                "isPreListing": true,
                "preListingInfo": {
                    "curAuctionPhase": "ContinuousTrading",
                    "phases": [
                        {
                            "phase": "CallAuction",
                            "startTime": "1735113600000",
                            "endTime": "1735116600000"
                        },
                        {
                            "phase": "CallAuctionNoCancel",
                            "startTime": "1735116600000",
                            "endTime": "1735116900000"
                        },
                        {
                            "phase": "CrossMatching",
                            "startTime": "1735116900000",
                            "endTime": "1735117200000"
                        },
                        {
                            "phase": "ContinuousTrading",
                            "startTime": "1735117200000",
                            "endTime": ""
                        }
                    ],
                    "auctionFeeInfo": {
                        "auctionFeeRate": "0",
                        "takerFeeRate": "0.001",
                        "makerFeeRate": "0.0004"
                    }
                },
                "riskParameters": {
                    "priceLimitRatioX": "0.05",
                    "priceLimitRatioY": "0.1"
                }
            }
        ],
        "nextPageCursor": "first%3DBIOUSDT%26last%3DBIOUSDT"
        }"#;
        let message: InstrumentsInfo = deserialize_slice(json.as_bytes()).unwrap();
        let expected = InstrumentsInfo::Linear {
            next_page_cursor: String::from("first%3DBIOUSDT%26last%3DBIOUSDT"),
            list: vec![AllCategoriesInstrumentsInfo {
                symbol: String::from("BIOUSDT"),
                contract_type: ContractType::LinearPerpetual,
                status: Status::PreLaunch,
                base_coin: String::from("BIO"),
                quote_coin: String::from("USDT"),
                launch_time: 1735032510000,
                delivery_time: 0,
                delivery_fee_rate: None,
                price_scale: 4,
                leverage_filter: LeverageFilter {
                    min_leverage: 1.0,
                    max_leverage: 5.00,
                    leverage_step: 0.01,
                },
                price_filter: PriceFilter {
                    min_price: 0.0001,
                    max_price: 1999.9998,
                    tick_size: 0.0001,
                },
                lot_size_filter: LotSizeFilter {
                    min_notional_value: 5.0,
                    max_order_qty: 70000.0,
                    max_mkt_order_qty: 14000.0,
                    min_order_qty: 1.0,
                    qty_step: 1.0,
                    post_only_max_order_qty: 70000.0,
                },
                unified_margin_trade: true,
                funding_interval: 480,
                settle_coin: String::from("USDT"),
                copy_trading: CopyTrading::None,
                upper_funding_rate: 0.05,
                lower_funding_rate: -0.05,
                risk_parameters: RiskParameters {
                    price_limit_ratio_x: 0.05,
                    price_limit_ratio_y: 0.1,
                },
                is_pre_listing: true,
                pre_listing_info: Some(PreListingInfo {
                    cur_auction_phase: CurAuctionPhase::ContinuousTrading,
                    phases: vec![
                        Phase {
                            phase: CurAuctionPhase::CallAuction,
                            start_time: Some(1735113600000),
                            end_time: Some(1735116600000),
                        },
                        Phase {
                            phase: CurAuctionPhase::CallAuctionNoCancel,
                            start_time: Some(1735116600000),
                            end_time: Some(1735116900000),
                        },
                        Phase {
                            phase: CurAuctionPhase::CrossMatching,
                            start_time: Some(1735116900000),
                            end_time: Some(1735117200000),
                        },
                        Phase {
                            phase: CurAuctionPhase::ContinuousTrading,
                            start_time: Some(1735117200000),
                            end_time: None,
                        },
                    ],
                    auction_fee_info: AuctionFeeInfo {
                        auction_fee_rate: 0.0,
                        taker_fee_rate: 0.001,
                        maker_fee_rate: 0.0004,
                    },
                }),
            }],
        };
        assert_eq!(message, expected);
    }

    #[test]
    fn deserialize_response_position_info() {
        let json = r#"{
            "retCode": 0,
            "retMsg": "OK",
            "result": {
                "list": [
                    {
                        "positionIdx": 0,
                        "riskId": 1,
                        "riskLimitValue": "150",
                        "symbol": "BTCUSD",
                        "side": "Sell",
                        "size": "300",
                        "avgPrice": "27464.50441675",
                        "positionValue": "0.01092319",
                        "tradeMode": 0,
                        "positionStatus": "Normal",
                        "autoAddMargin": 1,
                        "adlRankIndicator": 2,
                        "leverage": "10",
                        "positionBalance": "0.00139186",
                        "markPrice": "28224.50",
                        "liqPrice": "",
                        "bustPrice": "999999.00",
                        "positionMM": "0.0000015",
                        "positionIM": "0.00010923",
                        "tpslMode": "Full",
                        "takeProfit": "0.00",
                        "stopLoss": "0.00",
                        "trailingStop": "0.00",
                        "unrealisedPnl": "-0.00029413",
                        "curRealisedPnl": "0.00013123",
                        "cumRealisedPnl": "-0.00096902",
                        "seq": 5723621632,
                        "isReduceOnly": false,
                        "mmrSysUpdateTime": "",
                        "leverageSysUpdatedTime": "",
                        "sessionAvgPrice": "",
                        "createdTime": "1676538056258",
                        "updatedTime": "1697673600012"
                    }
                ],
                "nextPageCursor": "",
                "category": "inverse"
            },
            "retExtInfo": {},
            "time": 1697684980172
        }"#;
        let message: Response<PositionInfo> = deserialize_slice(json.as_bytes()).unwrap();
        let expected = Response {
            ret_code: 0,
            ret_msg: String::from("OK"),
            result: PositionInfo {
                category: Category::Inverse,
                next_page_cursor: String::from(""),
                list: vec![Position {
                    position_idx: PositionIdx::OneWay,
                    risk_limit_value: 150.0,
                    symbol: String::from("BTCUSD"),
                    side: Side::Sell,
                    size: 300.0,
                    avg_price: 27464.50441675,
                    position_value: 0.01092319,
                    trade_mode: TradeMode::CrossMargin,
                    auto_add_margin: AutoAddMargin::True,
                    position_status: PositionStatus::Normal,
                    leverage: Some(10.0),
                    mark_price: 28224.5,
                    liq_price: None,
                    bust_price: 999999.0,
                    position_i_m: 0.00010923,
                    position_m_m: 0.0000015,
                    position_balance: 0.00139186,
                    take_profit: 0.0,
                    stop_loss: 0.0,
                    trailing_stop: 0.0,
                    session_avg_price: None,
                    delta: None,
                    gamma: None,
                    vega: None,
                    theta: None,
                    unrealised_pnl: -0.00029413,
                    cur_realised_pnl: 0.00013123,
                    cum_realised_pnl: -0.00096902,
                    adl_rank_indicator: 2,
                    created_time: 1676538056258,
                    updated_time: 1697673600012,
                    seq: 5723621632,
                    is_reduce_only: false,
                    mmr_sys_updated_time: None,
                    leverage_sys_updated_time: Some(String::new()),
                    tpsl_mode: String::from("Full"),
                }],
            },
            time: 1697684980172,
            ret_ext_info: RetExtInfo {},
        };
        assert_eq!(message, expected);
    }
}
