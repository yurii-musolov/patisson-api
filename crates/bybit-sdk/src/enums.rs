use serde::{Deserialize, Serialize};
use std::fmt;

// Unified Account: spot | linear | inverse | option
// Classic Account: linear | inverse | spot
#[derive(Debug, Deserialize, Serialize)]
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
        match self {
            OrderStatus::New => true,
            OrderStatus::PartiallyFilled => true,
            OrderStatus::Untriggered => true,
            _ => false,
        }
    }
    pub fn is_closed_status(&self) -> bool {
        match self {
            OrderStatus::Rejected => true,
            OrderStatus::PartiallyFilledCanceled => true,
            OrderStatus::Filled => true,
            OrderStatus::Cancelled => true,
            OrderStatus::Triggered => true,
            OrderStatus::Deactivated => true,
            _ => false,
        }
    }
}

#[derive(PartialEq, Deserialize, Debug)]
pub enum TickDirection {
    PlusTick,      // price rise
    ZeroPlusTick, // trade occurs at the same price as the previous trade, which occurred at a price higher than that for the trade preceding it
    MinusTick,    // price drop
    ZeroMinusTick, // trade occurs at the same price as the previous trade, which occurred at a price lower than that for the trade preceding it
}

#[derive(Debug, Deserialize, Serialize)]
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
            Interval::Minute1 => "1",
            Interval::Minute3 => "3",
            Interval::Minute5 => "5",
            Interval::Minute15 => "15",
            Interval::Minute30 => "30",
            Interval::Minute60 => "60",
            Interval::Minute120 => "120",
            Interval::Minute240 => "240",
            Interval::Minute360 => "360",
            Interval::Minute720 => "720",
            Interval::Day => "D",
            Interval::Week => "W",
            Interval::Month => "M",
        };
        write!(f, "{value}")
    }
}

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
        match self {
            AccountType::CONTRACT => true,
            AccountType::UNIFIED => true,
            AccountType::FUND => true,
            _ => false,
        }
    }
    pub fn is_classic_account(&self) -> bool {
        match self {
            AccountType::SPOT => true,
            AccountType::CONTRACT => true,
            AccountType::OPTION => true,
            AccountType::FUND => true,
            AccountType::INVESTMENT => true, // Deprecated.
            _ => false,
        }
    }
}
