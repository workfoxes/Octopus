use super::transport::Transport;

/// The binance client
#[derive(Clone)]
pub struct Client {
    pub transport: Transport,
}