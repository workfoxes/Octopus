
pub mod credentials;
pub mod parameters;
pub mod errors;
pub mod transport;
pub mod client;

/// The main struct of the openlimits-binance module
#[derive(Clone)]
pub struct Binance {
    // pub exchange_info: ExchangeInfo,
    pub client: client::Client,
}
