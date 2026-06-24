use serde::Serialize;
use std::path::PathBuf;

/// Output mode for subscription data
#[derive(Debug, Clone)]
pub enum OutputMode {
    /// Output to stdout (NDJSON format)
    Stdout,
    /// Save to files at specified path
    File(PathBuf),
    /// Save to files at default path (~/.opens/nine-futu/data/live/)
    FileDefault,
}

/// Supported stock markets
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub enum Market {
    /// Hong Kong Stock Exchange
    HK,
    /// US Stock Exchange
    US,
    /// Shanghai Stock Exchange
    SH,
    /// Shenzhen Stock Exchange
    SZ,
    /// Singapore Exchange
    SG,
    /// Japan Exchange Group
    JP,
    /// Cryptocurrency
    CC,
}

impl Market {
    /// Parse market from prefix string (e.g., "HK", "US")
    pub fn from_prefix(prefix: &str) -> Option<Self> {
        match prefix {
            "HK" => Some(Self::HK),
            "US" => Some(Self::US),
            "SH" => Some(Self::SH),
            "SZ" => Some(Self::SZ),
            "SG" => Some(Self::SG),
            "JP" => Some(Self::JP),
            "CC" => Some(Self::CC),
            _ => None,
        }
    }

    /// Get market prefix as string
    pub fn to_str(&self) -> &'static str {
        match self {
            Self::HK => "HK",
            Self::US => "US",
            Self::SH => "SH",
            Self::SZ => "SZ",
            Self::SG => "SG",
            Self::JP => "JP",
            Self::CC => "CC",
        }
    }
}

/// Subscription data types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub enum SubType {
    /// Real-time quotes
    Quote,
    /// Order book depth
    OrderBook,
    /// Ticker (trades)
    Ticker,
    /// 1-minute K-line
    K1M,
    /// 3-minute K-line
    K3M,
    /// 5-minute K-line
    K5M,
    /// 10-minute K-line
    K10M,
    /// 15-minute K-line
    K15M,
    /// 30-minute K-line
    K30M,
    /// 60-minute K-line
    K60M,
    /// Daily K-line
    KDay,
    /// Weekly K-line
    KWeek,
    /// Monthly K-line
    KMon,
    /// Quarterly K-line
    KQuarter,
    /// Yearly K-line
    KYear,
    /// Real-time data
    RtData,
    /// Broker queue
    Broker,
    /// Price reminder
    PriceReminder,
}

impl SubType {
    /// Parse subscription type from string
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "QUOTE" => Some(Self::Quote),
            "ORDER_BOOK" => Some(Self::OrderBook),
            "TICKER" => Some(Self::Ticker),
            "K_1M" | "K1M" => Some(Self::K1M),
            "K_3M" | "K3M" => Some(Self::K3M),
            "K_5M" | "K5M" => Some(Self::K5M),
            "K_10M" | "K10M" => Some(Self::K10M),
            "K_15M" | "K15M" => Some(Self::K15M),
            "K_30M" | "K30M" => Some(Self::K30M),
            "K_60M" | "K60M" => Some(Self::K60M),
            "K_DAY" | "KDAY" => Some(Self::KDay),
            "K_WEEK" | "KWEEK" => Some(Self::KWeek),
            "K_MON" | "KMON" => Some(Self::KMon),
            "K_QUARTER" | "KQUARTER" => Some(Self::KQuarter),
            "K_YEAR" | "KYEAR" => Some(Self::KYear),
            "RT_DATA" => Some(Self::RtData),
            "BROKER" => Some(Self::Broker),
            "PRICE_REMINDER" => Some(Self::PriceReminder),
            _ => None,
        }
    }

    /// Get proto value for this subscription type
    pub fn to_proto_value(&self) -> i32 {
        match self {
            Self::Quote => 1,      // SubType_Basic
            Self::OrderBook => 2,  // SubType_OrderBook
            Self::Ticker => 4,     // SubType_Ticker
            Self::RtData => 5,     // SubType_RT
            Self::KDay => 6,       // SubType_KL_Day
            Self::K5M => 7,        // SubType_KL_5Min
            Self::K15M => 8,       // SubType_KL_15Min
            Self::K30M => 9,       // SubType_KL_30Min
            Self::K60M => 10,      // SubType_KL_60Min
            Self::K1M => 11,       // SubType_KL_1Min
            Self::KWeek => 12,     // SubType_KL_Week
            Self::KMon => 13,      // SubType_KL_Month
            Self::Broker => 14,    // SubType_Broker
            Self::KQuarter => 15,  // SubType_KL_Qurater
            Self::KYear => 16,     // SubType_KL_Year
            Self::K3M => 17,       // SubType_KL_3Min
            Self::K10M => 17,      // Map to 3Min (closest)
            Self::PriceReminder => 19,
        }
    }
}

/// Stock code with market prefix
#[derive(Debug, Clone, Serialize)]
pub struct StockCode {
    /// Market identifier
    pub market: Market,
    /// Stock code (e.g., "00700", "AAPL")
    pub code: String,
}

impl StockCode {
    /// Parse stock code from string (e.g., "HK.00700", "US.AAPL", "700")
    pub fn parse(s: &str) -> Option<Self> {
        let parts: Vec<&str> = s.splitn(2, '.').collect();
        if parts.len() != 2 {
            return None;
        }
        let market = Market::from_prefix(parts[0])?;
        Some(Self {
            market,
            code: parts[1].to_string(),
        })
    }

    /// Get full stock code with market prefix
    pub fn to_string(&self) -> String {
        format!("{}.{}", self.market.to_str(), self.code)
    }
}

/// Real-time stock quote data
#[derive(Debug, Clone, Serialize)]
pub struct StockQuote {
    /// Stock code with market prefix
    pub code: String,
    /// Last traded price
    pub last_done: f64,
    /// Previous close price
    pub prev_close_price: f64,
    /// Opening price
    pub open_price: f64,
    /// Highest price
    pub high_price: f64,
    /// Lowest price
    pub low_price: f64,
    /// Trading volume
    pub volume: i64,
    /// Trading turnover
    pub turnover: f64,
    /// Price change value
    pub change_val: f64,
    /// Price change rate (percentage)
    pub change_rate: f64,
    /// Price amplitude (percentage)
    pub amplitude: f64,
    /// Price-to-earnings ratio
    pub pe_ratio: f64,
    /// Dividend yield rate
    pub yield_rate: f64,
}

/// K-line bar data
#[derive(Debug, Clone, Serialize)]
pub struct KlineBar {
    /// Stock code with market prefix
    pub code: String,
    /// Time key (format: "YYYY-MM-DD HH:MM:SS")
    pub time_key: String,
    /// Opening price
    pub open: f64,
    /// Closing price
    pub close: f64,
    /// Highest price
    pub high: f64,
    /// Lowest price
    pub low: f64,
    /// Trading volume
    pub volume: i64,
    /// Trading turnover
    pub turnover: f64,
    /// Price change rate (percentage)
    pub change_rate: f64,
}

/// Order book entry
#[derive(Debug, Clone, Serialize)]
pub struct OrderBookEntry {
    /// Price level
    pub price: f64,
    /// Volume at this price level
    pub volume: i64,
}

/// Order book data
#[derive(Debug, Clone, Serialize)]
pub struct OrderBook {
    /// Stock code with market prefix
    pub code: String,
    /// Bid (buy) orders
    pub bid: Vec<OrderBookEntry>,
    /// Ask (sell) orders
    pub ask: Vec<OrderBookEntry>,
}

/// Ticker (trade) data
#[derive(Debug, Clone, Serialize)]
pub struct Ticker {
    /// Stock code with market prefix
    pub code: String,
    /// Trade time
    pub time: String,
    /// Trade price
    pub price: f64,
    /// Trade volume
    pub volume: i64,
    /// Trade turnover
    pub turnover: f64,
    /// Trade direction (1=Buy, 2=Sell, 3=Neutral)
    pub ticker_direction: i32,
    /// Trade sequence number
    pub sequence: i64,
}

/// Market snapshot data
#[derive(Debug, Clone, Serialize)]
pub struct SnapshotData {
    /// Stock code with market prefix
    pub code: String,
    /// Stock name
    pub name: String,
    /// Last traded price
    pub last_done: f64,
    /// Previous close price
    pub prev_close_price: f64,
    /// Opening price
    pub open_price: f64,
    /// Highest price
    pub high_price: f64,
    /// Lowest price
    pub low_price: f64,
    /// Trading volume
    pub volume: i64,
    /// Trading turnover
    pub turnover: f64,
    /// Market capitalization
    pub market_cap: f64,
    /// Price-to-earnings ratio
    pub pe_ratio: f64,
    /// Price-to-book ratio
    pub pb_ratio: f64,
    /// Dividend yield rate
    pub yield_rate: f64,
}

/// Market state information
#[derive(Debug, Clone, Serialize)]
pub struct MarketStateInfo {
    /// Stock code with market prefix
    pub code: String,
    /// Stock name
    pub name: String,
    /// Market state (0=Closed, 1=Morning, 2=Afternoon, etc.)
    pub market_state: i32,
    /// Last update time
    pub last_update_time: String,
}

/// Capital flow data
#[derive(Debug, Clone, Serialize)]
pub struct CapitalFlowData {
    /// Stock code with market prefix
    pub code: String,
    /// Capital inflow
    pub inflow: f64,
    /// Capital outflow
    pub outflow: f64,
    /// Net capital inflow
    pub net_inflow: f64,
}

/// Plate (sector) information
#[derive(Debug, Clone, Serialize)]
pub struct PlateInfo {
    /// Plate code
    pub code: String,
    /// Plate name
    pub name: String,
    /// Plate type (1=Industry, 2=Region, 3=Concept)
    pub plate_type: i32,
}

/// Stock in a plate
#[derive(Debug, Clone, Serialize)]
pub struct PlateStock {
    /// Stock code with market prefix
    pub code: String,
    /// Stock name
    pub name: String,
}

/// Historical K-line quota information
#[derive(Debug, Clone, Serialize)]
pub struct HistoryKlQuota {
    /// Number of quota used
    pub used_count: i32,
    /// Total quota available
    pub total_count: i32,
    /// Remaining quota
    pub remain_count: i32,
}

/// Subscription information
#[derive(Debug, Clone, Serialize)]
pub struct SubscriptionInfo {
    /// Stock code with market prefix
    pub code: String,
    /// Subscription type
    pub sub_type: SubType,
    /// Whether currently subscribed
    pub is_subscribed: bool,
}

/// Real-time data (time frame)
#[derive(Debug, Clone, Serialize)]
pub struct RtData {
    /// Stock code with market prefix
    pub code: String,
    /// Time string
    pub time: String,
    /// Current price
    pub price: f64,
    /// Last close price
    pub prev_close_price: f64,
    /// Average price
    pub avg_price: f64,
    /// Trading volume
    pub volume: i64,
    /// Trading turnover
    pub turnover: f64,
}

/// Broker queue data
#[derive(Debug, Clone, Serialize)]
pub struct BrokerQueue {
    /// Stock code with market prefix
    pub code: String,
    /// Broker ID
    pub broker_id: i32,
    /// Broker name
    pub broker_name: String,
    /// Buy volume
    pub buy_volume: i64,
    /// Sell volume
    pub sell_volume: i64,
}
