
use std::sync::atomic::AtomicBool;

use binance::websockets::{WebsocketEvent, WebSockets};
use influxdb::{Client, Timestamp};
use influxdb::InfluxDbWriteable;
use influxdb;
use chrono::{DateTime, Utc};


#[derive(InfluxDbWriteable)]
struct Item {
    time: DateTime<Utc>,
    symbol: String,
    interval: String,
    #[influxdb(tag)] open: f32,
    #[influxdb(tag)] close: f32,
    #[influxdb(tag)] high: f32,
    #[influxdb(tag)] low: f32,
}

fn processor(symbols: Vec<String>) {
    let mut endpoints: Vec<String> = Vec::new();
    let client = Client::new("http://localhost:8086", "test")
        .with_auth("admin", "password");

    for symbol in symbols.iter() {
        endpoints.push(format!("{}@kline_1m", symbol.to_lowercase()));
        // endpoints.push(format!("{}@kline_5m", symbol.to_lowercase()));
        // endpoints.push(format!("{}@kline_15m", symbol.to_lowercase()));
    }
    let keep_running = AtomicBool::new(true);
    let mut web_socket: WebSockets<'_> = WebSockets::new(|event: WebsocketEvent| {
        if let WebsocketEvent::Kline(event) = event {
            if event.kline.is_final_bar {
                println!("{:?}", event.kline);
                let item = Item {
                    time: Timestamp::Hours(1).into(),
                    symbol: event.symbol.clone(),
                    interval: event.kline.interval,
                    open: event.kline.open.parse().unwrap(),
                    close: event.kline.close.parse().unwrap(),
                    high: event.kline.high.parse().unwrap(),
                    low: event.kline.low.parse().unwrap()
                };

                let _a = client.query(item.into_query(event.symbol.clone()));
            }
        }
        Ok(())
    });

    web_socket.connect_multiple_streams(&endpoints).unwrap(); // check error
    if let Err(e) = web_socket.event_loop(&keep_running) {
        println!("Error: {:?}", e);
    }
    web_socket.disconnect().unwrap();
}

fn main() {
    println!("Hello, world! starting trading bot");
    processor(vec!["ethbtc", "bnbeth"].into_iter().map(String::from).collect())

}
