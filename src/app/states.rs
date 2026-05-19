use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AppState {
    Loading,
    Building,
    Running,
}
