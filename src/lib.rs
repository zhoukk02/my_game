pub mod app;
pub mod audio;
pub mod data;
pub mod input;
pub mod loader;

pub mod prelude {
    pub use crate::app;
    pub use crate::audio;
    pub use crate::data;
    pub use crate::input;
    pub use crate::loader;
}
