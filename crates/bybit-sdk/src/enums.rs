use serde::{Deserialize, Serialize};
use serde_repr::*;
use std::fmt;

#[derive(Debug, Deserialize, Serialize)]
pub enum Locale {
    #[serde(rename = "de-DE")]
    DeDe,
    #[serde(rename = "en-US")]
    EnUs,
    #[serde(rename = "es-AR")]
    EsAr,
    #[serde(rename = "es-ES")]
    EsEs,
    #[serde(rename = "es-MX")]
    EsMx,
    #[serde(rename = "fr-FR")]
    FrFr,
    #[serde(rename = "kk-KZ")]
    KkKz,
    #[serde(rename = "id-ID")]
    IdId,
    #[serde(rename = "uk-UA")]
    UkUa,
    #[serde(rename = "ja-JP")]
    JaJp,
    #[serde(rename = "ru-RU")]
    RuRu,
    #[serde(rename = "th-TH")]
    ThTh,
    #[serde(rename = "pt-BR")]
    PtBr,
    #[serde(rename = "tr-TR")]
    TrTr,
    #[serde(rename = "vi-VN")]
    ViVn,
    #[serde(rename = "zh-TW")]
    ZhTw,
    #[serde(rename = "ar-SA")]
    ArSa,
    #[serde(rename = "hi-IN")]
    HiIn,
    #[serde(rename = "fil-PH")]
    FilPh,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum AnnouncementType {
    #[serde(rename = "new_crypto")]
    NewCrypto,
    #[serde(rename = "latest_bybit_news")]
    LatestBybitNews,
    #[serde(rename = "delistings")]
    Delistings,
    #[serde(rename = "latest_activities")]
    LatestActivities,
    #[serde(rename = "product_updates")]
    ProductUpdates,
    #[serde(rename = "maintenance_updates")]
    MaintenanceUpdates,
    #[serde(rename = "new_fiat_listings")]
    NewFiatListings,
    #[serde(rename = "other")]
    Other,
}

// Unified Account: spot | linear | inverse | option
// Classic Account: linear | inverse | spot
#[derive(PartialEq, Debug, Deserialize, Serialize, Clone)]
pub enum Category {
    #[serde(rename = "inverse")]
    Inverse, // Inverse contract, including Inverse perp, Inverse futures.
    #[serde(rename = "linear")]
    Linear, // USDT perpetual, and USDC contract, including USDC perp, USDC futures.
    #[serde(rename = "option")]
    Option,
    #[serde(rename = "spot")]
    Spot,
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            Self::Inverse => "inverse",
            Self::Linear => "linear",
            Self::Option => "option",
            Self::Spot => "spot",
        };
        write!(f, "{value}")
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum OrderStatus {
    // open status
    New, // order has been placed successfully
    PartiallyFilled,
    Untriggered, // Conditional orders are created
    // closed status
    Rejected,
    PartiallyFilledCanceled, // Only spot has this order status
    Filled,
    Cancelled,   // In derivatives, orders with this status may have an executed qty
    Triggered,   // instantaneous state for conditional orders from Untriggered to New
    Deactivated, // UTA: Spot tp/sl order, conditional order, OCO order are cancelled before they are triggered
}

impl OrderStatus {
    pub fn is_open(&self) -> bool {
        matches!(self, Self::New | Self::PartiallyFilled | Self::Untriggered)
    }
    pub fn is_closed(&self) -> bool {
        matches!(
            self,
            Self::Rejected
                | Self::PartiallyFilledCanceled
                | Self::Filled
                | Self::Cancelled
                | Self::Triggered
                | Self::Deactivated
        )
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum TimeInForce {
    GTC, // GoodTillCancel
    IOC, // ImmediateOrCancel
    FOK, // FillOrKill
    PostOnly,
    RPI, // features:
         // Exclusive Matching: Only match non-algorithmic users; no execution against orders from Open API.
         // Post-Only Mechanism: Act as maker orders, adding liquidity
         // Lower Priority: Execute after non-RPI orders at the same price level.
         // Limited Access: Initially for select market makers across multiple pairs.
         // Order Book Updates: Excluded from API but displayed on the GUI.
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum CreateType {
    CreateByUser,
    CreateByFutureSpread, // Spread order
    CreateByAdminClosing,
    CreateBySettle, // USDC Futures delivery; position closed as a result of the delisting of a contract. This is recorded as a trade but not an order.
    CreateByStopOrder, // Futures conditional order
    CreateByTakeProfit, // Futures take profit order
    CreateByPartialTakeProfit, // Futures partial take profit order
    CreateByStopLoss, // Futures stop loss order
    CreateByPartialStopLoss, // Futures partial stop loss order
    CreateByTrailingStop, // Futures trailing stop order
    CreateByLiq,    // Laddered liquidation to reduce the required maintenance margin
    #[serde(rename = "CreateByTakeOver_PassThrough")]
    CreateByTakeOverPassThrough, // If the position is still subject to liquidation (i.e., does not meet the required maintenance margin level), the position shall be taken over by the liquidation engine and closed at the bankruptcy price.
    #[serde(rename = "CreateByAdl_PassThrough")]
    CreateByAdlPassThrough, // Auto-Deleveraging(ADL)
    #[serde(rename = "CreateByBlock_PassThrough")]
    CreateByBlockPassThrough, // Order placed via Paradigm
    #[serde(rename = "CreateByBlockTradeMovePosition_PassThrough")]
    CreateByBlockTradeMovePositionPassThrough, // Order created by move position
    CreateByClosing,  // The close order placed via web or app position area - web/app
    CreateByFGridBot, // Order created via grid bot - web/app
    CloseByFGridBot,  // Order closed via grid bot - web/app
    CreateByTWAP,     // Order created by TWAP - web/app
    CreateByTVSignal, // Order created by TV webhook - web/app
    CreateByMmRateClose, // Order created by Mm rate close function - web/app
    CreateByMartingaleBot, // Order created by Martingale bot - web/app
    CloseByMartingaleBot, // Order closed by Martingale bot - web/app
    CreateByIceBerg,  // Order created by Ice berg strategy - web/app
    CreateByArbitrage, // Order created by arbitrage - web/app
    CreateByDdh,      // Option dynamic delta hedge order - web/app
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum ExecType {
    Trade,
    AdlTrade,  // Auto-Deleveraging
    Funding,   // Funding fee
    BustTrade, // Takeover liquidation
    Delivery,  // USDC futures delivery; Position closed by contract delisted
    Settle,    // Inverse futures settlement; Position closed due to delisting
    BlockTrade,
    MovePosition,
    FutureSpread, // Spread leg execution
    UNKNOWN,      // May be returned by a classic account. Cannot query by this type
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum OrderType {
    Market,
    Limit,
    UNKNOWN, // is not a valid request parameter value. Is only used in some responses. Mainly, it is used when execType is Funding.
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum StopOrderType {
    TakeProfit,
    StopLoss,
    TrailingStop,
    Stop,
    PartialTakeProfit,
    PartialStopLoss,
    #[serde(rename = "tpslOrder")]
    TpslOrder, // spot TP/SL order
    OcoOrder,               // spot Oco order
    MmRateClose,            // On web or app can set MMR to close position
    BidirectionalTpslOrder, // Spot bidirectional tpsl order
    /// As Option::None
    /// Deprecated!
    /// TODO: write deserializer from ""
    #[serde(rename = "")]
    None,
}

#[derive(Debug, PartialEq, Deserialize)]
pub enum TickDirection {
    PlusTick,      // price rise
    ZeroPlusTick, // trade occurs at the same price as the previous trade, which occurred at a price higher than that for the trade preceding it
    MinusTick,    // price drop
    ZeroMinusTick, // trade occurs at the same price as the previous trade, which occurred at a price lower than that for the trade preceding it
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum Interval {
    #[serde(rename = "1")]
    Minute1,
    #[serde(rename = "3")]
    Minute3,
    #[serde(rename = "5")]
    Minute5,
    #[serde(rename = "15")]
    Minute15,
    #[serde(rename = "30")]
    Minute30,
    #[serde(rename = "60")]
    Minute60,
    #[serde(rename = "120")]
    Minute120,
    #[serde(rename = "240")]
    Minute240,
    #[serde(rename = "360")]
    Minute360,
    #[serde(rename = "720")]
    Minute720,
    #[serde(rename = "D")]
    Day,
    #[serde(rename = "W")]
    Week,
    #[serde(rename = "M")]
    Month,
}

impl fmt::Display for Interval {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            Self::Minute1 => "1",
            Self::Minute3 => "3",
            Self::Minute5 => "5",
            Self::Minute15 => "15",
            Self::Minute30 => "30",
            Self::Minute60 => "60",
            Self::Minute120 => "120",
            Self::Minute240 => "240",
            Self::Minute360 => "360",
            Self::Minute720 => "720",
            Self::Day => "D",
            Self::Week => "W",
            Self::Month => "M",
        };
        write!(f, "{value}")
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum IntervalTime {
    #[serde(rename = "5min")]
    Minute5,
    #[serde(rename = "15min")]
    Minute15,
    #[serde(rename = "30min")]
    Minute30,
    #[serde(rename = "1h")]
    Hour1,
    #[serde(rename = "4h")]
    Hour4,
    #[serde(rename = "1d")]
    Day1,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq)]
#[repr(u8)]
pub enum PositionIdx {
    /// 0:one-way mode position
    OneWay = 0,
    /// 1:Buy side of hedge-mode position
    Buy = 1,
    /// 2:Sell side of hedge-mode position
    Sell = 2,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum PositionStatus {
    Normal,
    Liq, // in the liquidation progress
    Adl, // in the auto-deleverage progress
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum RejectReason {
    #[serde(rename = "EC_NoError")]
    EcNoError,
    #[serde(rename = "EC_Others")]
    EcOthers,
    #[serde(rename = "EC_UnknownMessageType")]
    EcUnknownMessageType,
    #[serde(rename = "EC_MissingClOrdID")]
    EcMissingClOrdId,
    #[serde(rename = "EC_MissingOrigClOrdID")]
    EcMissingOrigClOrdId,
    #[serde(rename = "EC_ClOrdIDOrigClOrdIDAreTheSame")]
    EcClOrdIdorigClOrdIdareTheSame,
    #[serde(rename = "EC_DuplicatedClOrdID")]
    EcDuplicatedClOrdId,
    #[serde(rename = "EC_OrigClOrdIDDoesNotExist")]
    EcOrigClOrdIddoesNotExist,
    #[serde(rename = "EC_TooLateToCancel")]
    EcTooLateToCancel,
    #[serde(rename = "EC_UnknownOrderType")]
    EcUnknownOrderType,
    #[serde(rename = "EC_UnknownSide")]
    EcUnknownSide,
    #[serde(rename = "EC_UnknownTimeInForce")]
    EcUnknownTimeInForce,
    #[serde(rename = "EC_WronglyRouted")]
    EcWronglyRouted,
    #[serde(rename = "EC_MarketOrderPriceIsNotZero")]
    EcMarketOrderPriceIsNotZero,
    #[serde(rename = "EC_LimitOrderInvalidPrice")]
    EcLimitOrderInvalidPrice,
    #[serde(rename = "EC_NoEnoughQtyToFill")]
    EcNoEnoughQtyToFill,
    #[serde(rename = "EC_NoImmediateQtyToFill")]
    EcNoImmediateQtyToFill,
    #[serde(rename = "EC_PerCancelRequest")]
    EcPerCancelRequest,
    #[serde(rename = "EC_MarketOrderCannotBePostOnly")]
    EcMarketOrderCannotBePostOnly,
    #[serde(rename = "EC_PostOnlyWillTakeLiquidity")]
    EcPostOnlyWillTakeLiquidity,
    #[serde(rename = "EC_CancelReplaceOrder")]
    EcCancelReplaceOrder,
    #[serde(rename = "EC_InvalidSymbolStatus")]
    EcInvalidSymbolStatus,
    #[serde(rename = "EC_CancelForNoFullFill")]
    EcCancelForNoFullFill,
    #[serde(rename = "EC_BySelfMatch")]
    EcBySelfMatch,
    #[serde(rename = "EC_InCallAuctionStatus")]
    EcInCallAuctionStatus, // used for pre-market order operation, e.g., during 2nd phase of call auction, cancel order is not allowed, when the cancel request is failed to be rejected by trading server, the request will be rejected by matching box finally
    #[serde(rename = "EC_QtyCannotBeZero")]
    EcQtyCannotBeZero,
    #[serde(rename = "EC_MarketOrderNoSupportTIF")]
    EcMarketOrderNoSupportTif,
    #[serde(rename = "EC_ReachMaxTradeNum")]
    EcReachMaxTradeNum,
    #[serde(rename = "EC_InvalidPriceScale")]
    EcInvalidPriceScale,
    #[serde(rename = "EC_BitIndexInvalid")]
    EcBitIndexInvalid,
    #[serde(rename = "EC_StopBySelfMatch")]
    EcStopBySelfMatch,
    #[serde(rename = "EC_InvalidSmpType")]
    EcInvalidSmpType,
    #[serde(rename = "EC_CancelByMMP")]
    EcCancelByMmp,
    #[serde(rename = "EC_InvalidUserType")]
    EcInvalidUserType,
    #[serde(rename = "EC_InvalidMirrorOid")]
    EcInvalidMirrorOid,
    #[serde(rename = "EC_InvalidMirrorUid")]
    EcInvalidMirrorUid,
    #[serde(rename = "EC_EcInvalidQty")]
    EcEcInvalidQty,
    #[serde(rename = "EC_InvalidAmount")]
    EcInvalidAmount,
    #[serde(rename = "EC_LoadOrderCancel")]
    EcLoadOrderCancel,
    #[serde(rename = "EC_MarketQuoteNoSuppSell")]
    EcMarketQuoteNoSuppSell,
    #[serde(rename = "EC_DisorderOrderID")]
    EcDisorderOrderId,
    #[serde(rename = "EC_InvalidBaseValue")]
    EcInvalidBaseValue,
    #[serde(rename = "EC_LoadOrderCanMatch")]
    EcLoadOrderCanMatch,
    #[serde(rename = "EC_SecurityStatusFail")]
    EcSecurityStatusFail,
    #[serde(rename = "EC_ReachRiskPriceLimit")]
    EcReachRiskPriceLimit,
    #[serde(rename = "EC_OrderNotExist")]
    EcOrderNotExist,
    #[serde(rename = "EC_CancelByOrderValueZero")]
    EcCancelByOrderValueZero,
    #[serde(rename = "EC_CancelByMatchValueZero")]
    EcCancelByMatchValueZero,
    #[serde(rename = "EC_ReachMarketPriceLimit")]
    EcReachMarketPriceLimit,
}

#[derive(Debug)]
pub enum AccountType {
    CONTRACT, // Inverse Derivatives Account | Derivatives Account
    UNIFIED,  // Unified Trading Account
    FUND,     // Funding Account
    SPOT,     // Spot Account
}

impl AccountType {
    pub fn is_uta_2(&self) -> bool {
        matches!(self, Self::UNIFIED | Self::FUND)
    }
    pub fn is_uta_1(&self) -> bool {
        matches!(self, Self::CONTRACT | Self::UNIFIED | Self::FUND)
    }
    pub fn is_classic(&self) -> bool {
        matches!(self, Self::SPOT | Self::CONTRACT | Self::FUND)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum TransferStatus {
    SUCCESS,
    PENDING,
    FAILED,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum DepositStatus {
    #[serde(rename = "0")]
    Unknown,
    #[serde(rename = "1")]
    ToBeConfirmed,
    #[serde(rename = "2")]
    Processing,
    #[serde(rename = "3")]
    Success, // (finalised status of a success deposit)
    #[serde(rename = "4")]
    DepositFailed,
    #[serde(rename = "10011")]
    PendingToBeCreditedToFundingPool,
    #[serde(rename = "10012")]
    CreditedToFundingPoolSuccessfully,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum WithdrawStatus {
    SecurityCheck,
    Pending,
    #[serde(rename = "success")]
    Success,
    CancelByUser,
    Reject,
    Fail,
    BlockchainConfirmed,
    MoreInformationRequired,
    Unknown, // a rare status
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum TriggerBy {
    LastPrice,
    IndexPrice,
    MarkPrice,
    /// As Option::None
    /// Deprecated!
    /// TODO: write deserializer from ""
    #[serde(rename = "")]
    None,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum CancelType {
    CancelByUser,
    CancelByReduceOnly, // cancelled by reduceOnly
    CancelByPrepareLiq, // cancelled in order to attempt liquidation prevention by freeing up margin
    CancelAllBeforeLiq, // cancelled in order to attempt liquidation prevention by freeing up margin
    CancelByPrepareAdl, // cancelled due to ADL
    CancelAllBeforeAdl, // cancelled due to ADL
    CancelByAdmin,
    CancelBySettle,      // cancelled due to delisting contract
    CancelByTpSlTsClear, // TP/SL order cancelled when the position is cleared
    CancelBySmp,         // cancelled by SMP
    CancelByCannotAffordOrderCost,
    CancelByPmTrialMmOverEquity,
    CancelByAccountBlocking,
    CancelByDelivery,
    CancelByMmpTriggered,
    CancelByCrossSelfMuch,
    CancelByCrossReachMaxTradeNum,
    CancelByDCP,
    UNKNOWN,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum OptionPeriod {
    #[serde(rename = "7")]
    Day7,
    #[serde(rename = "14")]
    Day14,
    #[serde(rename = "21")]
    Day21,
    #[serde(rename = "30")]
    Day30,
    #[serde(rename = "60")]
    Day60,
    #[serde(rename = "90")]
    Day90,
    #[serde(rename = "180")]
    Day180,
    #[serde(rename = "270")]
    Day270,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum DataRecordingPeriod {
    #[serde(rename = "5min")]
    Minute5,
    #[serde(rename = "15min")]
    Minute15,
    #[serde(rename = "30min")]
    Minute30,
    #[serde(rename = "1h")]
    Hour1,
    #[serde(rename = "4h")]
    Hour4,
    #[serde(rename = "4d")]
    Day4,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum ContractType {
    InversePerpetual,
    LinearPerpetual,
    LinearFutures, // USDT/USDC Futures
    InverseFutures,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum Status {
    PreLaunch,
    Trading,
    Delivering,
    Closed,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum CurAuctionPhase {
    NotStarted, // Pre-market trading is not started
    Finished,   // Pre-market trading is finished
    // After the auction, if the pre-market contract fails to enter continues trading phase, it will be delisted and phase="Finished"
    // After the continuous trading, if the pre-market contract fails to be converted to official contract, it will be delisted and phase="Finished"
    CallAuction, // Auction phase of pre-market trading
    // only timeInForce=GTC, orderType=Limit order is allowed to submit
    // TP/SL are not supported; Conditional orders are not supported
    // cannot modify the order at this stage
    // order price range: [preOpenPrice x 0.5, maxPrice]
    CallAuctionNoCancel, // Auction no cancel phase of pre-market trading
    // only timeInForce=GTC, orderType=Limit order is allowed to submit
    // TP/SL are not supported; Conditional orders are not supported
    // cannot modify and cancel the order at this stage
    // order price range: Buy [lastPrice x 0.5, markPrice x 1.1], Sell [markPrice x 0.9, maxPrice]
    CrossMatching, // cross matching phase
    // cannot create, modify and cancel the order at this stage
    // Candle data is released from this stage
    ContinuousTrading, // Continuous trading phase
                       // There is no restriction to create, amend, cancel orders
                       // orderbook, public trade data is released from this stage
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum MarginTrading {
    #[serde(rename = "none")]
    None, // Regardless of normal account or UTA account, this trading pair does not support margin trading
    #[serde(rename = "both")]
    Both, // For both normal account and UTA account, this trading pair supports margin trading
    #[serde(rename = "utaOnly")]
    UtaOnly, // Only for UTA account,this trading pair supports margin trading
    #[serde(rename = "normalSpotOnly")]
    NormalSpotOnly, // Only for normal account, this trading pair supports margin trading
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum CopyTrading {
    #[serde(rename = "none")]
    None, // Regardless of normal account or UTA account, this trading pair does not support copy trading
    #[serde(rename = "both")]
    Both, // For both normal account and UTA account, this trading pair supports copy trading
    #[serde(rename = "utaOnly")]
    UtaOnly, // Only for UTA account,this trading pair supports copy trading
    #[serde(rename = "normalOnly")]
    NormalOnly, // Only for normal account, this trading pair supports copy trading
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Type {
    TransferIn,  // Assets that transferred into Unified | (inverse) derivatives wallet
    TransferOut, // Assets that transferred out from Unified | (inverse) derivatives wallet
    Trade,
    Settlement, // USDT Perp funding settlement, and USDC Perp funding settlement + USDC 8-hour session settlement
    // USDT / Inverse Perp funding settlement
    Delivery, // USDC Futures, Option delivery
    // Inverse Futures delivery
    Liquidation,
    ADL, // Auto-Deleveraging
    Airdrop,
    Bonus,          // Bonus claimed
    BonusRecollect, // Bonus expired
    FeeRefund,      // Trading fee refunded
    Interest,       // Interest occurred due to borrowing
    CurrencyBuy,    // Currency convert, and the liquidation for borrowing asset(UTA loan)
    CurrencySell,   // Currency convert, and the liquidation for borrowing asset(UTA loan)
    BorrowedAmountInsLoan,
    PrincipleRepaymentInsLoan,
    InterestRepaymentInsLoan,
    AutoSoldCollateralInsLoan, // the liquidation for borrowing asset(INS loan)
    AutoBuyLiabilityInsLoan,   // the liquidation for borrowing asset(INS loan)
    AutoPrincipleRepaymentInsLoan,
    AutoInterestRepaymentInsLoan,
    TransferInInsLoan,           // Transfer In when in the liquidation of OTC loan
    TransferOutInsLoan,          // Transfer Out when in the liquidation of OTC loan
    SpotRepaymentSell,           // One-click repayment currency sell
    SpotRepaymentBuy,            // One-click repayment currency buy
    TokensSubscription,          // Spot leverage token subscription
    TokensRedemption,            // Spot leverage token redemption
    AutoDeduction,               // Asset auto deducted by system (roll back)
    FlexibleStakingSubscription, // Byfi flexible stake subscription
    FlexibleStakingRedemption,   // Byfi flexible stake redemption
    FixedStakingSubscription,    // Byfi fixed stake subscription
    PremarketTransferOut,
    PremarketDeliverySellNewCoin,
    PremarketDeliveryBuyNewCoin,
    PremarketDeliveryPledgePaySeller,
    PremarketDeliveryPledgeBack,
    PremarketRollbackPledgeBack,
    PremarketRollbackPledgePenaltyToBuyer,
    CustodyNetworkFee,   // fireblocks business
    CustodySettleFee,    // fireblocks business
    CustodyLock,         // fireblocks / copper business
    CustodyUnlock,       // fireblocks business
    CustodyUnlockRefund, // fireblocks business
    LoansBorrowFunds,    // crypto loan
    LoansPledgeAsset,    // crypto loan repayment
    BonusTransferIn,
    BonusTransferOut,
    PefTransferIn,
    PefTransferOut,
    PefProfitShare,
    #[serde(rename = "Others")]
    Others,
}

impl Type {
    pub fn is_uta(&self) -> bool {
        matches!(
            self,
            Self::TransferIn
                | Self::TransferOut
                | Self::Trade
                | Self::Settlement
                | Self::Delivery
                | Self::Liquidation
                | Self::ADL
                | Self::Airdrop
                | Self::Bonus
                | Self::BonusRecollect
                | Self::FeeRefund
                | Self::Interest
                | Self::CurrencyBuy
                | Self::CurrencySell
                | Self::BorrowedAmountInsLoan
                | Self::PrincipleRepaymentInsLoan
                | Self::InterestRepaymentInsLoan
                | Self::AutoSoldCollateralInsLoan
                | Self::AutoBuyLiabilityInsLoan
                | Self::AutoPrincipleRepaymentInsLoan
                | Self::AutoInterestRepaymentInsLoan
                | Self::TransferInInsLoan
                | Self::TransferOutInsLoan
                | Self::SpotRepaymentSell
                | Self::SpotRepaymentBuy
                | Self::TokensSubscription
                | Self::TokensRedemption
                | Self::AutoDeduction
                | Self::FlexibleStakingSubscription
                | Self::FlexibleStakingRedemption
                | Self::FixedStakingSubscription
                | Self::PremarketTransferOut
                | Self::PremarketDeliverySellNewCoin
                | Self::PremarketDeliveryBuyNewCoin
                | Self::PremarketDeliveryPledgePaySeller
                | Self::PremarketDeliveryPledgeBack
                | Self::PremarketRollbackPledgeBack
                | Self::PremarketRollbackPledgePenaltyToBuyer
                | Self::CustodyNetworkFee
                | Self::CustodySettleFee
                | Self::CustodyLock
                | Self::CustodyUnlock
                | Self::CustodyUnlockRefund
                | Self::LoansBorrowFunds
                | Self::LoansPledgeAsset
                | Self::BonusTransferIn
                | Self::BonusTransferOut
                | Self::PefTransferIn
                | Self::PefTransferOut
                | Self::PefProfitShare
        )
    }
    pub fn is_contract(&self) -> bool {
        matches!(
            self,
            Self::TransferIn
                | Self::TransferOut
                | Self::Trade
                | Self::Settlement
                | Self::Delivery
                | Self::Liquidation
                | Self::ADL
                | Self::Airdrop
                | Self::Bonus
                | Self::BonusRecollect
                | Self::FeeRefund
                | Self::CurrencyBuy
                | Self::CurrencySell
                | Self::AutoDeduction
                | Self::Others
        )
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum UnifiedMarginStatus {
    #[serde(rename = "1")]
    ClassicAccount,
    #[serde(rename = "3")]
    UnifiedTradingAccount1, // 1.0
    #[serde(rename = "4")]
    UnifiedTradingAccount1Pro, // 1.0 (pro version)
    #[serde(rename = "5")]
    UnifiedTradingAccount2, // 2.0
    #[serde(rename = "6")]
    UnifiedTradingAccount2Pro, // 2.0 (pro version)
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum LtStatus {
    #[serde(rename = "1")]
    LTCanBePurchasedAndRedeemed,
    #[serde(rename = "2")]
    LTCanBePurchasedButNotRedeemed,
    #[serde(rename = "3")]
    LTCanBeRedeemedButNotPurchased,
    #[serde(rename = "4")]
    LTCannotBePurchasedNorRedeemed,
    #[serde(rename = "5")]
    AdjustingPosition,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum ConvertAccountType {
    #[serde(rename = "eb_convert_uta")]
    Uta, // Unified Trading Account
    #[serde(rename = "eb_convert_funding")]
    Funding, // Funding Account
    #[serde(rename = "eb_convert_inverse")]
    Inverse, // Inverse Derivatives Account (no USDT in this wallet))
    #[serde(rename = "eb_convert_spot")]
    Spot, // Spot Account
    #[serde(rename = "eb_convert_contract")]
    Contract, // Derivatives Account (contain USDT in this wallet)
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum VipLevel {
    #[serde(rename = "No VIP")]
    NoVIP,
    #[serde(rename = "VIP-1")]
    VIP1,
    #[serde(rename = "VIP-2")]
    VIP2,
    #[serde(rename = "VIP-3")]
    VIP3,
    #[serde(rename = "VIP-4")]
    VIP4,
    #[serde(rename = "VIP-5")]
    VIP5,
    #[serde(rename = "VIP-Supreme")]
    VIPSupreme,
    #[serde(rename = "PRO-1")]
    PRO1,
    #[serde(rename = "PRO-2")]
    PRO2,
    #[serde(rename = "PRO-3")]
    PRO3,
    #[serde(rename = "PRO-4")]
    PRO4,
    #[serde(rename = "PRO-5")]
    PRO5,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum AdlRankIndicator {
    #[serde(rename = "0")]
    Zero, // default value of empty position
    #[serde(rename = "1")]
    One,
    #[serde(rename = "2")]
    Two,
    #[serde(rename = "3")]
    Three,
    #[serde(rename = "4")]
    Four,
    #[serde(rename = "5")]
    Five,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum SmpType {
    None, // default
    CancelMaker,
    CancelTaker,
    CancelBoth,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum TpslMode {
    Full,
    Partial,
    /// As Option::None
    /// Deprecated!
    /// TODO: write deserializer from ""
    #[serde(rename = "")]
    None,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum OcoTriggerBy {
    #[serde(rename = "OcoTriggerByUnknown")]
    Unknown,
    #[serde(rename = "OcoTriggerByTp")]
    Tp,
    #[serde(rename = "OcoTriggerByBySl")]
    BySl,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq)]
#[repr(u8)]
pub enum TriggerDirection {
    UNKNOWN = 0,
    Rise = 1,
    Fall = 2,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Innovation {
    #[serde(rename = "0")]
    False,
    #[serde(rename = "1")]
    True,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum PlaceType {
    #[serde(rename = "option")]
    Option,
    #[serde(rename = "iv")]
    Iv,
    #[serde(rename = "price")]
    Price,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug, PartialEq)]
pub enum Pair {
    // example of BTCUSDT
    Base,  // BTC
    Quote, // USDT
}

#[derive(Debug, Deserialize, PartialEq)]
pub enum SlippageToleranceType {
    TickSize,
    Percent,
    UNKNOWN, // default
}
