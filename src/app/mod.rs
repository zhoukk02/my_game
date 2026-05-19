mod plugin;
mod sets;
mod states;

pub type Id = u32;

pub use {
    plugin::plugin,
    sets::{LogicSet, RenderSet},
    states::AppState,
};
