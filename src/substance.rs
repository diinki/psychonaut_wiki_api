use crate::types::{ MinMax, EffectTimeDuration };



pub struct Substance {
    name: String,
    routes_of_administration: RoutesOfAdministration,
    effects: Vec<Effect>
}

pub struct RoutesOfAdministration {
    name: String,
    dose: Dose,
    duration: Duration,

    bioavailability: MinMax,
}

pub struct Dose {
    units: String,
    threshold: f32,

    heavy: f32,

    common: MinMax,
    light: MinMax,
    strong: MinMax,
}

pub struct Duration {
    afterglow: EffectTimeDuration,
    comeup: EffectTimeDuration,
    duration: EffectTimeDuration,
    offset: EffectTimeDuration,
    onset: EffectTimeDuration,
    peak: EffectTimeDuration,
    total: EffectTimeDuration,
}

pub struct Effect {
    name: String,
    url: String
}