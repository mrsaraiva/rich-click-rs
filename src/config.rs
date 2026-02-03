use std::sync::OnceLock;

use rich_rs::r#box::ROUNDED;
use rich_rs::{AlignMethod, ColorSystem, PaddingDimensions, Style};

use crate::theme::{apply_theme, ThemeError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorSystemMode {
    Auto,
    Standard,
    EightBit,
    TrueColor,
    Windows,
    None,
}

impl Default for ColorSystemMode {
    fn default() -> Self {
        Self::Auto
    }
}

impl ColorSystemMode {
    pub fn to_color_system(self, current: Option<ColorSystem>) -> Option<ColorSystem> {
        match self {
            ColorSystemMode::Auto => current,
            ColorSystemMode::Standard => Some(ColorSystem::Standard),
            ColorSystemMode::EightBit => Some(ColorSystem::EightBit),
            ColorSystemMode::TrueColor => Some(ColorSystem::TrueColor),
            ColorSystemMode::Windows => Some(ColorSystem::Windows),
            ColorSystemMode::None => None,
        }
    }
}

/// Panel configuration for help sections.
#[derive(Debug, Clone)]
pub struct PanelConfig {
    pub box_type: rich_rs::r#box::Box,
    pub border_style: Style,
    pub title_style: Style,
    pub panel_style: Style,
    pub padding: PaddingDimensions,
    pub align: AlignMethod,
    pub expand: bool,
}

impl Default for PanelConfig {
    fn default() -> Self {
        Self {
            box_type: ROUNDED,
            border_style: parse_style("dim"),
            title_style: Style::default(),
            panel_style: Style::default(),
            padding: PaddingDimensions::from((0, 1)),
            align: AlignMethod::Left,
            expand: true,
        }
    }
}

/// Table configuration for option/command tables.
#[derive(Debug, Clone)]
pub struct TableConfig {
    pub show_lines: bool,
    pub leading: usize,
    pub pad_edge: bool,
    pub padding: (usize, usize),
    pub expand: bool,
    pub box_type: Option<rich_rs::r#box::Box>,
    pub row_styles: Vec<Style>,
    pub border_style: Style,
}

impl Default for TableConfig {
    fn default() -> Self {
        Self {
            show_lines: false,
            leading: 0,
            pad_edge: false,
            padding: (0, 1),
            expand: false,
            box_type: None,
            row_styles: Vec::new(),
            border_style: parse_style("dim"),
        }
    }
}

/// Rich help configuration (subset of rich-click options).
#[derive(Debug, Clone)]
pub struct RichHelpConfig {
    pub theme: Option<String>,
    pub enable_theme_env_var: bool,

    pub width: Option<usize>,
    pub max_width: Option<usize>,
    pub color_system: ColorSystemMode,
    pub force_terminal: Option<bool>,

    pub style_option: Style,
    pub style_option_negative: Option<Style>,
    pub style_argument: Style,
    pub style_command: Style,
    pub style_command_aliases: Style,
    pub style_switch: Style,
    pub style_switch_negative: Option<Style>,
    pub style_metavar: Style,
    pub style_metavar_append: Style,
    pub style_metavar_separator: Style,
    pub style_range_append: Option<Style>,
    pub style_header_text: Style,
    pub style_epilog_text: Style,
    pub style_footer_text: Style,
    pub style_usage: Style,
    pub style_usage_command: Style,
    pub style_usage_separator: Style,
    pub style_deprecated: Style,
    pub style_helptext_first_line: Style,
    pub style_helptext: Style,
    pub style_helptext_aliases: Option<Style>,
    pub style_option_help: Style,
    pub style_command_help: Style,
    pub style_option_default: Style,
    pub style_option_envvar: Style,
    pub style_required_short: Style,
    pub style_required_long: Style,
    pub style_options_panel_border: Style,
    pub style_options_panel_box: Option<rich_rs::r#box::Box>,
    pub style_options_panel_help_style: Style,
    pub style_options_panel_title_style: Style,
    pub style_options_panel_padding: PaddingDimensions,
    pub style_options_panel_style: Style,
    pub align_options_panel: AlignMethod,
    pub style_options_table_show_lines: bool,
    pub style_options_table_leading: usize,
    pub style_options_table_pad_edge: bool,
    pub style_options_table_padding: PaddingDimensions,
    pub style_options_table_expand: bool,
    pub style_options_table_box: Option<rich_rs::r#box::Box>,
    pub style_options_table_row_styles: Vec<Style>,
    pub style_options_table_border_style: Style,
    pub style_commands_panel_border: Style,
    pub panel_inline_help_in_title: bool,
    pub panel_inline_help_delimiter: String,
    pub style_commands_panel_box: Option<rich_rs::r#box::Box>,
    pub style_commands_panel_help_style: Style,
    pub style_commands_panel_title_style: Style,
    pub style_commands_panel_padding: PaddingDimensions,
    pub style_commands_panel_style: Style,
    pub align_commands_panel: AlignMethod,
    pub style_commands_table_show_lines: bool,
    pub style_commands_table_leading: usize,
    pub style_commands_table_pad_edge: bool,
    pub style_commands_table_padding: PaddingDimensions,
    pub style_commands_table_expand: bool,
    pub style_commands_table_box: Option<rich_rs::r#box::Box>,
    pub style_commands_table_row_styles: Vec<Style>,
    pub style_commands_table_border_style: Style,
    pub style_errors_panel_border: Style,
    pub style_errors_panel_box: Option<rich_rs::r#box::Box>,
    pub align_errors_panel: AlignMethod,
    pub style_errors_suggestion: Option<Style>,
    pub style_errors_suggestion_command: Option<Style>,
    pub style_padding_errors: Style,
    pub style_aborted: Style,
    pub style_padding_usage: Style,
    pub style_padding_helptext: Style,
    pub style_padding_epilog: Style,

    pub panel_title_padding: usize,

    pub options_table_column_types: Vec<String>,
    pub commands_table_column_types: Vec<String>,
    pub options_table_help_sections: Vec<String>,
    pub commands_table_help_sections: Vec<String>,

    pub header_text: Option<String>,
    pub footer_text: Option<String>,
    pub panel_title_string: String,
    pub deprecated_string: String,
    pub deprecated_with_reason_string: String,
    pub default_string: String,
    pub envvar_string: String,
    pub required_short_string: String,
    pub required_long_string: String,
    pub range_string: String,
    pub append_metavars_help_string: String,
    pub append_range_help_string: String,
    pub helptext_aliases_string: String,
    pub arguments_panel_title: String,
    pub options_panel_title: String,
    pub commands_panel_title: String,
    pub errors_panel_title: String,
    pub delimiter_comma: String,
    pub delimiter_slash: String,
    pub errors_suggestion: Option<String>,
    pub errors_epilogue: Option<String>,
    pub aborted_text: String,

    pub padding_header_text: PaddingDimensions,
    pub padding_usage: PaddingDimensions,
    pub padding_helptext: PaddingDimensions,
    pub padding_helptext_deprecated: PaddingDimensions,
    pub padding_helptext_first_line: PaddingDimensions,
    pub padding_epilog: PaddingDimensions,
    pub padding_footer_text: PaddingDimensions,
    pub padding_errors_panel: PaddingDimensions,
    pub padding_errors_suggestion: PaddingDimensions,
    pub padding_errors_epilogue: PaddingDimensions,

    pub show_arguments: Option<bool>,
    pub show_commands: Option<bool>,
    pub show_metavars_column: Option<bool>,
    pub commands_before_options: bool,
    pub default_panels_first: bool,
    pub append_metavars_help: Option<bool>,
    pub group_arguments_options: bool,
    pub option_envvar_first: Option<bool>,
    pub text_emojis: Option<bool>,
    pub use_click_short_help: bool,
    pub helptext_show_aliases: bool,

    pub panel: PanelConfig,
    pub table: TableConfig,
}

impl Default for RichHelpConfig {
    fn default() -> Self {
        let mut cfg = Self {
            theme: None,
            enable_theme_env_var: true,
            width: None,
            max_width: None,
            color_system: ColorSystemMode::Auto,
            force_terminal: None,
            style_option: Style::default(),
            style_option_negative: None,
            style_argument: Style::default(),
            style_command: Style::default(),
            style_command_aliases: Style::default(),
            style_switch: Style::default(),
            style_switch_negative: None,
            style_metavar: Style::default(),
            style_metavar_append: Style::default(),
            style_metavar_separator: Style::default(),
            style_range_append: None,
            style_header_text: Style::default(),
            style_epilog_text: Style::default(),
            style_footer_text: Style::default(),
            style_usage: Style::default(),
            style_usage_command: Style::default(),
            style_usage_separator: Style::default(),
            style_deprecated: Style::default(),
            style_helptext_first_line: Style::default(),
            style_helptext: Style::default(),
            style_helptext_aliases: None,
            style_option_help: Style::default(),
            style_command_help: Style::default(),
            style_option_default: Style::default(),
            style_option_envvar: Style::default(),
            style_required_short: Style::default(),
            style_required_long: Style::default(),
            style_options_panel_border: Style::default(),
            style_options_panel_box: None,
            style_options_panel_help_style: Style::default(),
            style_options_panel_title_style: Style::default(),
            style_options_panel_padding: PaddingDimensions::from((0, 1)),
            style_options_panel_style: Style::default(),
            align_options_panel: AlignMethod::Left,
            style_options_table_show_lines: false,
            style_options_table_leading: 0,
            style_options_table_pad_edge: false,
            style_options_table_padding: PaddingDimensions::from((0, 1)),
            style_options_table_expand: true,
            style_options_table_box: None,
            style_options_table_row_styles: Vec::new(),
            style_options_table_border_style: Style::default(),
            style_commands_panel_border: Style::default(),
            panel_inline_help_in_title: false,
            panel_inline_help_delimiter: " - ".to_string(),
            style_commands_panel_box: None,
            style_commands_panel_help_style: Style::default(),
            style_commands_panel_title_style: Style::default(),
            style_commands_panel_padding: PaddingDimensions::from((0, 1)),
            style_commands_panel_style: Style::default(),
            align_commands_panel: AlignMethod::Left,
            style_commands_table_show_lines: false,
            style_commands_table_leading: 0,
            style_commands_table_pad_edge: false,
            style_commands_table_padding: PaddingDimensions::from((0, 1)),
            style_commands_table_expand: true,
            style_commands_table_box: None,
            style_commands_table_row_styles: Vec::new(),
            style_commands_table_border_style: Style::default(),
            style_errors_panel_border: Style::default(),
            style_errors_panel_box: None,
            align_errors_panel: AlignMethod::Left,
            style_errors_suggestion: None,
            style_errors_suggestion_command: None,
            style_padding_errors: Style::default(),
            style_aborted: parse_style("red"),
            style_padding_usage: Style::default(),
            style_padding_helptext: Style::default(),
            style_padding_epilog: Style::default(),
            panel_title_padding: 1,
            options_table_column_types: vec![
                "required".to_string(),
                "opt_long".to_string(),
                "opt_short".to_string(),
                "metavar".to_string(),
                "help".to_string(),
            ],
            commands_table_column_types: vec!["name".to_string(), "aliases".to_string(), "help".to_string()],
            options_table_help_sections: vec![
                "help".to_string(),
                "deprecated".to_string(),
                "envvar".to_string(),
                "default".to_string(),
                "required".to_string(),
            ],
            commands_table_help_sections: vec!["help".to_string(), "deprecated".to_string()],
            header_text: None,
            footer_text: None,
            panel_title_string: "{}".to_string(),
            deprecated_string: "[deprecated]".to_string(),
            deprecated_with_reason_string: "[deprecated: {}]".to_string(),
            default_string: "[default: {}]".to_string(),
            envvar_string: "[env var: {}]".to_string(),
            required_short_string: "*".to_string(),
            required_long_string: "[required]".to_string(),
            range_string: "[{}]".to_string(),
            append_metavars_help_string: "({})".to_string(),
            append_range_help_string: "[range: {}]".to_string(),
            helptext_aliases_string: "Aliases: {}".to_string(),
            arguments_panel_title: "Arguments".to_string(),
            options_panel_title: "Options".to_string(),
            commands_panel_title: "Commands".to_string(),
            errors_panel_title: "Error".to_string(),
            delimiter_comma: ",".to_string(),
            delimiter_slash: "/".to_string(),
            errors_suggestion: None,
            errors_epilogue: None,
            aborted_text: "Aborted.".to_string(),
            padding_header_text: PaddingDimensions::from((1, 1, 0, 1)),
            padding_usage: PaddingDimensions::from(1),
            padding_helptext: PaddingDimensions::from((0, 1, 1, 1)),
            padding_helptext_deprecated: PaddingDimensions::from(0),
            padding_helptext_first_line: PaddingDimensions::from(0),
            padding_epilog: PaddingDimensions::from(1),
            padding_footer_text: PaddingDimensions::from(1),
            padding_errors_panel: PaddingDimensions::from((0, 0, 1, 0)),
            padding_errors_suggestion: PaddingDimensions::from((0, 1, 0, 1)),
            padding_errors_epilogue: PaddingDimensions::from((0, 1, 1, 1)),
            show_arguments: None,
            show_commands: None,
            show_metavars_column: None,
            commands_before_options: false,
            default_panels_first: false,
            append_metavars_help: None,
            group_arguments_options: false,
            option_envvar_first: None,
            text_emojis: None,
            use_click_short_help: false,
            helptext_show_aliases: true,
            panel: PanelConfig::default(),
            table: TableConfig::default(),
        };

        let _ = cfg.apply_theme_name("default-box");
        cfg
    }
}

impl RichHelpConfig {
    pub fn global() -> &'static Self {
        static GLOBAL: OnceLock<RichHelpConfig> = OnceLock::new();
        GLOBAL.get_or_init(|| RichHelpConfig::load_from_env())
    }

    pub fn builder() -> RichHelpConfigBuilder {
        RichHelpConfigBuilder {
            config: RichHelpConfig::global().clone(),
        }
    }

    pub fn load_from_env() -> Self {
        let mut cfg = RichHelpConfig::default();
        if !cfg.enable_theme_env_var {
            return cfg;
        }
        let env = std::env::var("RICH_CLICK_THEME").ok();
        if let Some(value) = env {
            if value.trim_start().starts_with('{') {
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(&value) {
                    cfg.apply_json_overrides(&json);
                }
            } else {
                cfg.theme = Some(value.trim().to_string());
                let _ = cfg.apply_theme_name(value.trim());
            }
        }
        cfg.sync_render_config();
        cfg
    }

    pub fn apply_theme_name(&mut self, theme: &str) -> Result<(), ThemeError> {
        let result = apply_theme(self, theme);
        if result.is_ok() {
            self.sync_render_config();
        }
        result
    }

    fn apply_json_overrides(&mut self, value: &serde_json::Value) {
        if let Some(theme) = value.get("theme").and_then(|v| v.as_str()) {
            self.theme = Some(theme.to_string());
            let _ = self.apply_theme_name(theme);
        }
        if let Some(obj) = value.as_object() {
            for (key, v) in obj {
                if key == "theme" {
                    continue;
                }
                self.apply_override(key, v);
            }
        }
    }

    fn apply_override(&mut self, key: &str, value: &serde_json::Value) {
        match key {
            "width" => self.width = value.as_u64().map(|v| v as usize),
            "max_width" => self.max_width = value.as_u64().map(|v| v as usize),
            "panel_title_string" => if let Some(v) = value.as_str() { self.panel_title_string = v.to_string(); },
            "deprecated_string" => if let Some(v) = value.as_str() { self.deprecated_string = v.to_string(); },
            "deprecated_with_reason_string" => if let Some(v) = value.as_str() { self.deprecated_with_reason_string = v.to_string(); },
            "default_string" => if let Some(v) = value.as_str() { self.default_string = v.to_string(); },
            "envvar_string" => if let Some(v) = value.as_str() { self.envvar_string = v.to_string(); },
            "required_short_string" => if let Some(v) = value.as_str() { self.required_short_string = v.to_string(); },
            "required_long_string" => if let Some(v) = value.as_str() { self.required_long_string = v.to_string(); },
            "range_string" => if let Some(v) = value.as_str() { self.range_string = v.to_string(); },
            "append_metavars_help_string" => if let Some(v) = value.as_str() { self.append_metavars_help_string = v.to_string(); },
            "append_range_help_string" => if let Some(v) = value.as_str() { self.append_range_help_string = v.to_string(); },
            "delimiter_comma" => if let Some(v) = value.as_str() { self.delimiter_comma = v.to_string(); },
            "delimiter_slash" => if let Some(v) = value.as_str() { self.delimiter_slash = v.to_string(); },
            "options_panel_title" => if let Some(v) = value.as_str() { self.options_panel_title = v.to_string(); },
            "commands_panel_title" => if let Some(v) = value.as_str() { self.commands_panel_title = v.to_string(); },
            "arguments_panel_title" => if let Some(v) = value.as_str() { self.arguments_panel_title = v.to_string(); },
            "style_option" => if let Some(v) = value.as_str() { self.style_option = parse_style(v); },
            "style_argument" => if let Some(v) = value.as_str() { self.style_argument = parse_style(v); },
            "style_command" => if let Some(v) = value.as_str() { self.style_command = parse_style(v); },
            "style_metavar" => if let Some(v) = value.as_str() { self.style_metavar = parse_style(v); },
            "style_usage" => if let Some(v) = value.as_str() { self.style_usage = parse_style(v); },
            "style_deprecated" => if let Some(v) = value.as_str() { self.style_deprecated = parse_style(v); },
            "style_helptext" => if let Some(v) = value.as_str() { self.style_helptext = parse_style(v); },
            "style_option_help" => if let Some(v) = value.as_str() { self.style_option_help = parse_style(v); },
            "style_command_help" => if let Some(v) = value.as_str() { self.style_command_help = parse_style(v); },
            "style_options_panel_border" => if let Some(v) = value.as_str() { self.style_options_panel_border = parse_style(v); },
            "style_commands_panel_border" => if let Some(v) = value.as_str() { self.style_commands_panel_border = parse_style(v); },
            "style_options_table_border_style" => if let Some(v) = value.as_str() { self.style_options_table_border_style = parse_style(v); },
            "style_commands_table_border_style" => if let Some(v) = value.as_str() { self.style_commands_table_border_style = parse_style(v); },
            _ => {}
        }
    }

    fn sync_render_config(&mut self) {
        self.panel.box_type = self.style_options_panel_box.unwrap_or(ROUNDED);
        self.panel.border_style = self.style_options_panel_border;
        self.panel.title_style = self.style_options_panel_title_style;
        self.panel.panel_style = self.style_options_panel_style;
        self.panel.padding = self.style_options_panel_padding;
        self.panel.align = self.align_options_panel;
        self.panel.expand = true;

        let padding = self.style_options_table_padding.unpack();
        self.table.show_lines = self.style_options_table_show_lines;
        self.table.leading = self.style_options_table_leading;
        self.table.pad_edge = self.style_options_table_pad_edge;
        self.table.padding = (padding.3, padding.1);
        self.table.expand = self.style_options_table_expand;
        self.table.box_type = self.style_options_table_box;
        self.table.border_style = self.style_options_table_border_style;
    }
}

pub struct RichHelpConfigBuilder {
    config: RichHelpConfig,
}

impl RichHelpConfigBuilder {
    pub fn build(self) -> RichHelpConfig {
        let mut cfg = self.config;
        cfg.sync_render_config();
        cfg
    }

    pub fn theme(mut self, theme: &str) -> Self {
        self.config.theme = Some(theme.to_string());
        let _ = self.config.apply_theme_name(theme);
        self
    }

    pub fn width(mut self, width: usize) -> Self {
        self.config.width = Some(width);
        self
    }

    pub fn max_width(mut self, width: usize) -> Self {
        self.config.max_width = Some(width);
        self
    }

    pub fn color_system(mut self, mode: ColorSystemMode) -> Self {
        self.config.color_system = mode;
        self
    }

    pub fn force_terminal(mut self, force: bool) -> Self {
        self.config.force_terminal = Some(force);
        self
    }

    pub fn show_arguments(mut self, show: bool) -> Self {
        self.config.show_arguments = Some(show);
        self
    }

    pub fn commands_before_options(mut self, value: bool) -> Self {
        self.config.commands_before_options = value;
        self
    }
}

fn parse_style(input: &str) -> Style {
    Style::parse(input).unwrap_or_default()
}
