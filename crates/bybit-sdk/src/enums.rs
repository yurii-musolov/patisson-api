use serde::{Deserialize, Serialize};
use std::fmt;

// Unified Account: spot | linear | inverse | option
// Classic Account: linear | inverse | spot
#[derive(Debug, Deserialize, Serialize, Clone)]
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

#[derive(Debug)]
pub enum OrderStatus {
    // open status
    New, // order has been placed successfully
    PartiallyFilled,
    Untriggered, // Conditional orders are createdRejected
    // closed status
    Rejected,
    PartiallyFilledCanceled, // Only spot has this order status
    Filled,
    Cancelled,   // In derivatives, orders with this status may have an executed qty
    Triggered,   // instantaneous state for conditional orders from Untriggered to New
    Deactivated, // UTA: Spot tp/sl order, conditional order, OCO order are cancelled before they are triggered
}

impl OrderStatus {
    pub fn is_open_status(&self) -> bool {
        matches!(self, Self::New | Self::PartiallyFilled | Self::Untriggered)
    }
    pub fn is_closed_status(&self) -> bool {
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

#[derive(Debug)]
pub enum AccountType {
    CONTRACT,   // Inverse Derivatives Account | Derivatives Account
    UNIFIED,    // Unified Trading Account
    FUND,       // Funding Account
    SPOT,       // Spot Account
    OPTION,     // USDC Derivatives
    INVESTMENT, // ByFi Account (this service is now offline)
}

impl AccountType {
    pub fn is_unified_trading_account(&self) -> bool {
        matches!(self, Self::CONTRACT | Self::UNIFIED | Self::FUND)
    }
    pub fn is_classic_account(&self) -> bool {
        matches!(
            self,
            Self::SPOT | Self::CONTRACT | Self::OPTION | Self::FUND | Self::INVESTMENT
        )
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum Side {
    Buy,
    Sell,
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
    NotStarted,
    Finished,
    CallAuction,
    CallAuctionNoCancel,
    CrossMatching,
    ContinuousTrading,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum CopyTrading {
    None,
    Both,
    UtaOnly,
    NormalOnly,
}
