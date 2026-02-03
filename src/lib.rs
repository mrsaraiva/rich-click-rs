//! rich-click-rs
//!
//! Rich help formatting for click-rs using rich-rs renderables.

mod config;
mod render;
mod theme;

pub use config::{
    ColorSystemMode, GroupConfig, PanelConfig, RichHelpConfig, RichHelpConfigBuilder, TableConfig,
    TextMarkup,
};
pub use render::{
    main_rich_command, main_rich_command_with_errors, main_rich_group, main_rich_group_with_errors, RichHelp,
    RichHelpRenderer,
};
pub use theme::{list_themes, ThemeError};
