use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Strategy {
    name: String,
    description: String,
    take_profile: String,
    stop_loss: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct FilterCheck {
    id: i32,
    name: String,
    strategy_id: String,
    params: Vec<IndicatorParams>,
}

#[derive(Serialize, Deserialize, Debug)]
struct IndicatorParams {
    id: i32,
    key: String,
    value: String,
}
