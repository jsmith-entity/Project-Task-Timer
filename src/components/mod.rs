pub mod window;
pub use super::components::window::Window;

mod controls;
mod log_view;
mod main_view;
mod paginator;
mod popups;

use super::components::{controls::*, log_view::*, main_view::*, paginator::*, popups::*};
