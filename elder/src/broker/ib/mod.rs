use crate::broker::{Broker, TradeItem};
use binance::model::KlineEvent;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

pub struct InteractiveBroker {
    sender: Sender<TradeItem>,
    receiver: Receiver<TradeItem>,
}

impl Broker for InteractiveBroker {
    fn new() -> Box<Self> {
        let (tx, rx): (Sender<TradeItem>, Receiver<TradeItem>) = mpsc::channel();
        return Box::from(InteractiveBroker {
            sender: tx,
            receiver: rx,
        });
    }
    fn init(&mut self) {
        todo!()
    }
    /// processor - is sample function that will allow to process all the data
    fn processor(self: &mut Self, symbols: Vec<String>) {
        todo!()
    }

    fn push(self: &mut Self, event: KlineEvent) {
        todo!()
    }
}
