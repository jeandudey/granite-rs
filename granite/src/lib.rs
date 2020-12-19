mod auto;
pub use self::auto::*;

pub mod widgets;

pub mod prelude {
    pub use super::{ApplicationExt, widgets::WelcomeExt};
}
