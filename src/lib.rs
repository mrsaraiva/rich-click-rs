//! rich-click-rs
//!
//! Rich help formatting for click-rs using rich-rs renderables.

mod config;
mod render;
mod theme;
mod rich_main;
mod testing;

pub use config::{
    ColorSystemMode, GroupConfig, PanelConfig, RichHelpConfig, RichHelpConfigBuilder, TableConfig,
    TextMarkup,
};
pub use render::{
    main_rich_command, main_rich_command_with_errors, main_rich_group, main_rich_group_with_errors, RichHelp,
    RichHelpRenderer,
};
pub use theme::{list_themes, ThemeError};
pub use rich_main::RichMainExt;
pub use testing::RichCliRunner;

#[macro_export]
macro_rules! rich_main {
    ($command:expr) => {{
        let args: Vec<String> = std::env::args().skip(1).collect();
        let cfg = $crate::RichHelpConfig::global().clone();
        $crate::main_rich_command(&$command, args, &cfg)
    }};
}

#[macro_export]
macro_rules! rich_main_with_errors {
    ($command:expr) => {{
        let args: Vec<String> = std::env::args().skip(1).collect();
        let cfg = $crate::RichHelpConfig::global().clone();
        $crate::main_rich_command_with_errors(&$command, args, &cfg)
    }};
}

#[macro_export]
macro_rules! rich_main_group {
    ($group:expr) => {{
        let args: Vec<String> = std::env::args().skip(1).collect();
        let cfg = $crate::RichHelpConfig::global().clone();
        $crate::main_rich_group(&$group, args, &cfg)
    }};
}

#[macro_export]
macro_rules! rich_main_group_with_errors {
    ($group:expr) => {{
        let args: Vec<String> = std::env::args().skip(1).collect();
        let cfg = $crate::RichHelpConfig::global().clone();
        $crate::main_rich_group_with_errors(&$group, args, &cfg)
    }};
}
