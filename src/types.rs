use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Default, Debug)]
pub struct MinMax {
    pub min: Option<f32>,
    pub max: Option<f32>,
}
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct EffectTimeDuration {
    pub units: Option<String>,
    pub min: Option<f32>,
    pub max: Option<f32>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ResponseError {
    pub message: Option<String>,
    pub code: Option<String>,
}
