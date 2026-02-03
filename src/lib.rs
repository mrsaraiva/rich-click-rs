//! rich-click-rs
//!
//! Rich help formatting for click-rs using rich-rs renderables.

mod config;
mod render;

pub use config::{PanelConfig, RichHelpConfig, TableConfig};
pub use render::{main_rich_command, main_rich_group, RichHelp, RichHelpRenderer};
