
pub mod credentials;
pub mod parameters;
pub mod errors;
pub mod transport;
pub mod client;

use parameters::BinanceParameters;
use client::Client;
use transport::Transport;
use crate::shared::traits::info::{ExchangeInfo, ExchangeInfoRetrieval, MarketPairInfo, MarketPairHandle};

/// The main struct of the openlimits-binance module
#[derive(Clone)]
pub struct Binance {
    pub exchange_info: ExchangeInfo,
    pub client: client::Client,
}


#[async_trait]
impl Exchange for Binance {
    type InitParams = BinanceParameters;
    type InnerClient = Client;

    async fn new(parameters: Self::InitParams) -> Result<Self> {
        let binance = match parameters.credentials {
            Some(credentials) => Binance {
                exchange_info: ExchangeInfo::new(),
                client: Client {
                    transport: Transport::with_credential(
                        &credentials.api_key,
                        &credentials.api_secret,
                        parameters.environment == Environment::Sandbox,
                    )?,
                },
            },
            None => Binance {
                exchange_info: ExchangeInfo::new(),
                client: Client {
                    transport: Transport::new(parameters.environment == Environment::Sandbox)?,
                },
            },
        };

        binance.refresh_market_info().await?;
        Ok(binance)
    }

    fn inner_client(&self) -> Option<&Self::InnerClient> {
        Some(&self.client)
    }
}

#[async_trait]
impl ExchangeInfoRetrieval for Binance {
    async fn retrieve_pairs(&self) -> Result<Vec<MarketPairInfo>> {
        self.client.get_exchange_info().await.map(|v| {
            v.symbols
                .into_iter()
                .map(|symbol| {
                    let lot_size = symbol
                        .filters
                        .iter()
                        .find_map(|f| match f {
                            SymbolFilter::LotSize {
                                max_qty: _,
                                min_qty: _,
                                step_size,
                            } => Some(step_size),
                            _ => None,
                        })
                        .expect("Couldn't find lot size.");

                    let tick_size = symbol
                        .filters
                        .iter()
                        .find_map(|f| match f {
                            SymbolFilter::PriceFilter {
                                min_price: _,
                                max_price: _,
                                tick_size,
                            } => Some(tick_size),
                            _ => None,
                        })
                        .expect("Couldn't find tick size.");

                    MarketPairInfo {
                        base: symbol.base_asset,
                        quote: symbol.quote_asset,
                        symbol: symbol.symbol,
                        base_increment: *lot_size,
                        quote_increment: *tick_size,
                        min_base_trade_size: None,
                        min_quote_trade_size: None,
                    }
                })
                .collect()
        })
    }

    async fn refresh_market_info(&self) -> Result<Vec<MarketPairHandle>> {
        self.exchange_info
            .refresh(self as &dyn ExchangeInfoRetrieval)
            .await
    }

    async fn get_pair(&self, market_pair: &MarketPair) -> Result<MarketPairHandle> {
        let name = crate::model::MarketPair::from(market_pair.clone()).0;
        self.exchange_info.get_pair(&name)
    }
}