use crate::types::{ MinMax, EffectTimeDuration };
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Substance {
    pub name: Option<String>,
    #[serde(rename(deserialize = "roas", serialize = "roas"))]
    pub routes_of_administration: Option<Vec<RoutesOfAdministration>>,
    pub effects: Option<Vec<Effect>>
}
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct RoutesOfAdministration {
    pub bioavailability: Option<MinMax>,
    pub duration: Option<Duration>,
    pub name: Option<String>,
    pub dose: Option<Dose>,
}
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Dose {
    pub threshold: Option<f32>,
    pub strong: Option<MinMax>,
    pub common: Option<MinMax>,
    pub light: Option<MinMax>,
    pub heavy: Option<f32>,
    pub units: Option<String>,
}
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Duration {
    pub afterglow: Option<EffectTimeDuration>,
    pub duration: Option<EffectTimeDuration>,
    pub comeup: Option<EffectTimeDuration>,
    pub offset: Option<EffectTimeDuration>,
    pub onset: Option<EffectTimeDuration>,
    pub total: Option<EffectTimeDuration>,
    pub peak: Option<EffectTimeDuration>,
}
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Effect {
    pub name: Option<String>,
    pub url: Option<String>
}