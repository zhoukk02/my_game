pub mod app;
pub mod core;
pub mod input;
pub mod loader;
pub mod logic;

pub mod prelude {
    pub use crate::app;
    pub use crate::core;
    pub use crate::input;
    pub use crate::loader;
    pub use crate::logic;
}
