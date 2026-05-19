use bevy::prelude::*;

use serde::Deserialize;

use crate::app::Id;

#[derive(Asset, TypePath, Deserialize, Debug, Clone)]
pub struct AudioData {
    pub id: Id,
    pub name: String,
    pub path: String,
    pub volume: f32,
    pub playback_rate: f64,
    pub panning: f32,
}
