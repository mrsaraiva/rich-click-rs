//! rich-click-rs
//!
//! Rich help formatting for click-rs using rich-rs renderables.

mod config;
mod render;
mod theme;

pub use config::{
    ColorSystemMode, PanelConfig, RichHelpConfig, RichHelpConfigBuilder, TableConfig, TextMarkup,
};
pub use render::{main_rich_command, main_rich_group, RichHelp, RichHelpRenderer};
pub use theme::ThemeError;
