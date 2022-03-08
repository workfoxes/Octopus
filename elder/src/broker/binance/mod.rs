// use binance::model::KlineEvent;
// use std::sync::atomic::AtomicBool;
// use std::sync::mpsc;
// use std::sync::mpsc::{Receiver, Sender};

// use binance::websockets::{WebSockets, WebsocketEvent};
// use chrono::{DateTime, NaiveDateTime, Utc};

// use crate::broker::{Broker, TradeItem};

// pub struct Binance {
//     sender: Sender<TradeItem>,
//     receiver: Receiver<TradeItem>,
// }

// impl Binance {}

// impl Broker for Binance {
//     fn new() -> Box<Self> {
//         let (tx, rx): (Sender<TradeItem>, Receiver<TradeItem>) = mpsc::channel();
//         return Box::from(Binance {
//             sender: tx,
//             receiver: rx,
//         });
//     }

//     fn init(self: &mut Self) {
//         println!("init")
//     }

//     /// processor - is sample function that will allow to process all the data
//     fn processor(self: &mut Self, symbols: Vec<String>) {
//         let mut endpoints: Vec<String> = Vec::new();

//         for symbol in symbols.iter() {
//             endpoints.push(format!("{}@kline_1m", symbol.to_lowercase()));
//             // endpoints.push(format!("{}@kline_5m", symbol.to_lowercase()));
//             // endpoints.push(format!("{}@kline_15m", symbol.to_lowercase()));
//         }
//         let keep_running = AtomicBool::new(true);
//         let mut web_socket: WebSockets<'_> = WebSockets::new(|event: WebsocketEvent| {
//             if let WebsocketEvent::Kline(event) = event {
//                 self.push(event);
//             }
//             Ok(())
//         });

//         web_socket.connect_multiple_streams(&endpoints).unwrap(); // check error
//         if let Err(e) = web_socket.event_loop(&keep_running) {
//             println!("Error: {:?}", e);
//         }
//         web_socket.disconnect().unwrap();
//     }
//     fn push(self: &mut Self, event: KlineEvent) {
//         if event.kline.is_final_bar {
//             println!(
//                 "final bar, [{}:{}], {:?}",
//                 event.kline.symbol, event.kline.interval, event
//             );
//             let datetime: DateTime<Utc> = DateTime::from_utc(
//                 NaiveDateTime::from_timestamp(event.kline.close_time, 0),
//                 Utc,
//             );
//             let trade_item = TradeItem {
//                 open: event.kline.open.parse().unwrap(),
//                 close: event.kline.close.parse().unwrap(),
//                 high: event.kline.high.parse().unwrap(),
//                 low: event.kline.low.parse().unwrap(),
//                 trades: event.kline.number_of_trades as f32,
//                 interval: event.kline.interval.to_string(),
//                 symbol: event.kline.symbol.to_string(),
//                 time: datetime,
//             };
//             self.sender.send(trade_item).unwrap();
//         }
//     }
// }
