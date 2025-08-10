pub mod task_view;

pub use task_view::TaskView;

mod navigation_bar;
mod task;
mod tasks;

use super::task_view::{navigation_bar::NavigationBar, task::Task, tasks::Tasks};
