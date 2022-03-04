pub mod binance;
pub mod constant;
pub mod ib;

use crate::binance::model::KlineEvent;
use chrono::{DateTime, Utc};

use crate::broker::binance::Binance;
use crate::constant::Brokers;


struct TradeItem {
    time: DateTime<Utc>,
    symbol: String,
    interval: String,
    open: f32,
    close: f32,
    high: f32,
    low: f32,
    trades: f32,
}

pub trait Broker {
    fn new() -> Box<Self>;
    fn init(self: &mut Self);
    fn processor(self: &mut Self, symbols: Vec<String>);
    fn push(self: &mut Self, event: KlineEvent);
}

/// get(broker) - this function will allow to get the object reference for the any broker
pub fn get(broker: Brokers) -> Box<Binance> {
    Binance::new()
    // match broker {
    //     Brokers::Binance => {
    //         return Binance::new()
    //     }
    //     Brokers::InteractiveBroker => {
    //         return InteractiveBroker::new()
    //     }
    //     _ => {}
    // }
}
