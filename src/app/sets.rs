use bevy::prelude::*;

#[derive(
    SystemSet, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
pub enum LogicSet {
    Input,
    Action,
    Simulation,
}

#[derive(
    SystemSet, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
pub enum RenderSet {
    Sync,
    Animation,
    Visual,
    Ui,
    Audio,
}
