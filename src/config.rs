use std::sync::OnceLock;
use std::collections::HashMap;

use rich_rs::r#box::{
    ASCII, ASCII2, ASCII_DOUBLE_HEAD, DOUBLE, DOUBLE_EDGE, HEAVY, HEAVY_EDGE, HEAVY_HEAD,
    HORIZONTALS, MARKDOWN, MINIMAL, MINIMAL_DOUBLE_HEAD, MINIMAL_HEAVY_HEAD, ROUNDED, SIMPLE,
    SIMPLE_HEAD, SIMPLE_HEAVY, SQUARE, SQUARE_DOUBLE_HEAD,
};
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextMarkup {
    Ansi,
    Rich,
    Markdown,
    None,
}

impl Default for TextMarkup {
    fn default() -> Self {
        TextMarkup::Ansi
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
    pub collapse_padding: bool,
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
            collapse_padding: false,
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
    pub style_options_table_collapse_padding: bool,
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
    pub style_commands_table_collapse_padding: bool,
    pub style_commands_table_expand: bool,
    pub style_commands_table_box: Option<rich_rs::r#box::Box>,
    pub style_commands_table_row_styles: Vec<Style>,
    pub style_commands_table_border_style: Style,
    pub style_commands_table_column_width_ratio: Option<(Option<usize>, Option<usize>)>,
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
    pub errors_show_param_source: bool,
    pub errors_param_source_format: String,

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
    pub text_markup: TextMarkup,
    pub text_emojis: Option<bool>,
    pub text_paragraph_linebreaks: Option<String>,
    pub use_click_short_help: bool,
    pub helptext_show_aliases: bool,
    pub show_prompt: bool,
    pub prompt_string: String,
    pub prompt_confirm_string: String,
    pub prompt_hidden_string: String,
    pub prompt_confirm_hidden_string: String,
    pub command_aliases: HashMap<String, Vec<String>>,
    pub option_groups: Vec<GroupConfig>,
    pub command_groups: Vec<GroupConfig>,

    pub panel_options: PanelConfig,
    pub panel_commands: PanelConfig,
    pub panel_arguments: PanelConfig,
    pub table_options: TableConfig,
    pub table_commands: TableConfig,
    pub table_arguments: TableConfig,
}

#[derive(Debug, Clone)]
pub struct GroupConfig {
    pub name: String,
    pub items: Vec<String>,
    pub help: Option<String>,
    pub inline_help_in_title: Option<bool>,
    pub title_style: Option<Style>,
    pub help_style: Option<Style>,
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
            style_options_table_collapse_padding: false,
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
            style_commands_table_collapse_padding: false,
            style_commands_table_expand: true,
            style_commands_table_box: None,
            style_commands_table_row_styles: Vec::new(),
            style_commands_table_border_style: Style::default(),
            style_commands_table_column_width_ratio: None,
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
            errors_show_param_source: false,
            errors_param_source_format: " (from {})".to_string(),
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
            text_markup: TextMarkup::Ansi,
            text_emojis: None,
            text_paragraph_linebreaks: None,
            use_click_short_help: false,
            helptext_show_aliases: false,
            show_prompt: false,
            prompt_string: "[prompt]".to_string(),
            prompt_confirm_string: "[prompt (confirm)]".to_string(),
            prompt_hidden_string: "[prompt (hidden)]".to_string(),
            prompt_confirm_hidden_string: "[prompt (confirm, hidden)]".to_string(),
            command_aliases: HashMap::new(),
            option_groups: Vec::new(),
            command_groups: Vec::new(),
            panel_options: PanelConfig::default(),
            panel_commands: PanelConfig::default(),
            panel_arguments: PanelConfig::default(),
            table_options: TableConfig::default(),
            table_commands: TableConfig::default(),
            table_arguments: TableConfig::default(),
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
        apply_terminal_width_env(&mut cfg);
        apply_force_terminal_env(&mut cfg);
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
            "color_system" => if let Some(mode) = parse_color_system_mode(value) { self.color_system = mode; },
            "force_terminal" => self.force_terminal = value.as_bool(),
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
            "errors_panel_title" => if let Some(v) = value.as_str() { self.errors_panel_title = v.to_string(); },
            "aborted_text" => if let Some(v) = value.as_str() { self.aborted_text = v.to_string(); },
            "errors_show_param_source" => if let Some(v) = value.as_bool() { self.errors_show_param_source = v; },
            "errors_param_source_format" => if let Some(v) = value.as_str() { self.errors_param_source_format = v.to_string(); },
            "show_prompt" => if let Some(v) = value.as_bool() { self.show_prompt = v; },
            "prompt_string" => if let Some(v) = value.as_str() { self.prompt_string = v.to_string(); },
            "prompt_confirm_string" => if let Some(v) = value.as_str() { self.prompt_confirm_string = v.to_string(); },
            "prompt_hidden_string" => if let Some(v) = value.as_str() { self.prompt_hidden_string = v.to_string(); },
            "prompt_confirm_hidden_string" => if let Some(v) = value.as_str() { self.prompt_confirm_hidden_string = v.to_string(); },
            "style_option" => if let Some(v) = value.as_str() { self.style_option = parse_style(v); },
            "style_option_negative" => self.style_option_negative = parse_optional_style(value),
            "style_argument" => if let Some(v) = value.as_str() { self.style_argument = parse_style(v); },
            "style_command" => if let Some(v) = value.as_str() { self.style_command = parse_style(v); },
            "style_command_aliases" => if let Some(v) = value.as_str() { self.style_command_aliases = parse_style(v); },
            "style_switch" => if let Some(v) = value.as_str() { self.style_switch = parse_style(v); },
            "style_switch_negative" => self.style_switch_negative = parse_optional_style(value),
            "style_metavar" => if let Some(v) = value.as_str() { self.style_metavar = parse_style(v); },
            "style_metavar_append" => if let Some(v) = value.as_str() { self.style_metavar_append = parse_style(v); },
            "style_metavar_separator" => if let Some(v) = value.as_str() { self.style_metavar_separator = parse_style(v); },
            "style_range_append" => self.style_range_append = parse_optional_style(value),
            "style_header_text" => if let Some(v) = value.as_str() { self.style_header_text = parse_style(v); },
            "style_epilog_text" => if let Some(v) = value.as_str() { self.style_epilog_text = parse_style(v); },
            "style_footer_text" => if let Some(v) = value.as_str() { self.style_footer_text = parse_style(v); },
            "style_usage" => if let Some(v) = value.as_str() { self.style_usage = parse_style(v); },
            "style_usage_command" => if let Some(v) = value.as_str() { self.style_usage_command = parse_style(v); },
            "style_usage_separator" => if let Some(v) = value.as_str() { self.style_usage_separator = parse_style(v); },
            "style_deprecated" => if let Some(v) = value.as_str() { self.style_deprecated = parse_style(v); },
            "style_helptext_first_line" => if let Some(v) = value.as_str() { self.style_helptext_first_line = parse_style(v); },
            "style_helptext" => if let Some(v) = value.as_str() { self.style_helptext = parse_style(v); },
            "style_helptext_aliases" => self.style_helptext_aliases = parse_optional_style(value),
            "style_option_help" => if let Some(v) = value.as_str() { self.style_option_help = parse_style(v); },
            "style_command_help" => if let Some(v) = value.as_str() { self.style_command_help = parse_style(v); },
            "style_option_default" => if let Some(v) = value.as_str() { self.style_option_default = parse_style(v); },
            "style_option_envvar" => if let Some(v) = value.as_str() { self.style_option_envvar = parse_style(v); },
            "style_required_short" => if let Some(v) = value.as_str() { self.style_required_short = parse_style(v); },
            "style_required_long" => if let Some(v) = value.as_str() { self.style_required_long = parse_style(v); },
            "style_options_panel_border" => if let Some(v) = value.as_str() { self.style_options_panel_border = parse_style(v); },
            "style_commands_panel_border" => if let Some(v) = value.as_str() { self.style_commands_panel_border = parse_style(v); },
            "style_errors_panel_border" => if let Some(v) = value.as_str() { self.style_errors_panel_border = parse_style(v); },
            "style_options_panel_box" => self.style_options_panel_box = parse_box_value(value),
            "style_commands_panel_box" => self.style_commands_panel_box = parse_box_value(value),
            "style_errors_panel_box" => self.style_errors_panel_box = parse_box_value(value),
            "style_options_panel_help_style" => if let Some(v) = value.as_str() { self.style_options_panel_help_style = parse_style(v); },
            "style_commands_panel_help_style" => if let Some(v) = value.as_str() { self.style_commands_panel_help_style = parse_style(v); },
            "style_options_panel_title_style" => if let Some(v) = value.as_str() { self.style_options_panel_title_style = parse_style(v); },
            "style_commands_panel_title_style" => if let Some(v) = value.as_str() { self.style_commands_panel_title_style = parse_style(v); },
            "style_options_panel_style" => if let Some(v) = value.as_str() { self.style_options_panel_style = parse_style(v); },
            "style_commands_panel_style" => if let Some(v) = value.as_str() { self.style_commands_panel_style = parse_style(v); },
            "style_padding_usage" => if let Some(v) = value.as_str() { self.style_padding_usage = parse_style(v); },
            "style_padding_helptext" => if let Some(v) = value.as_str() { self.style_padding_helptext = parse_style(v); },
            "style_padding_epilog" => if let Some(v) = value.as_str() { self.style_padding_epilog = parse_style(v); },
            "style_padding_errors" => if let Some(v) = value.as_str() { self.style_padding_errors = parse_style(v); },
            "style_aborted" => if let Some(v) = value.as_str() { self.style_aborted = parse_style(v); },
            "style_options_table_border_style" => if let Some(v) = value.as_str() { self.style_options_table_border_style = parse_style(v); },
            "style_commands_table_border_style" => if let Some(v) = value.as_str() { self.style_commands_table_border_style = parse_style(v); },
            "style_commands_table_column_width_ratio" => if let Some(v) = parse_ratio_pair(value) { self.style_commands_table_column_width_ratio = Some(v); },
            "style_options_table_show_lines" => if let Some(v) = value.as_bool() { self.style_options_table_show_lines = v; },
            "style_commands_table_show_lines" => if let Some(v) = value.as_bool() { self.style_commands_table_show_lines = v; },
            "style_options_table_leading" => if let Some(v) = value.as_u64() { self.style_options_table_leading = v as usize; },
            "style_commands_table_leading" => if let Some(v) = value.as_u64() { self.style_commands_table_leading = v as usize; },
            "style_options_table_pad_edge" => if let Some(v) = value.as_bool() { self.style_options_table_pad_edge = v; },
            "style_commands_table_pad_edge" => if let Some(v) = value.as_bool() { self.style_commands_table_pad_edge = v; },
            "style_options_table_collapse_padding" => if let Some(v) = value.as_bool() { self.style_options_table_collapse_padding = v; },
            "style_commands_table_collapse_padding" => if let Some(v) = value.as_bool() { self.style_commands_table_collapse_padding = v; },
            "style_options_table_expand" => if let Some(v) = value.as_bool() { self.style_options_table_expand = v; },
            "style_commands_table_expand" => if let Some(v) = value.as_bool() { self.style_commands_table_expand = v; },
            "style_options_table_box" => self.style_options_table_box = parse_box_value(value),
            "style_commands_table_box" => self.style_commands_table_box = parse_box_value(value),
            "style_options_table_row_styles" => if let Some(v) = parse_style_list(value) { self.style_options_table_row_styles = v; },
            "style_commands_table_row_styles" => if let Some(v) = parse_style_list(value) { self.style_commands_table_row_styles = v; },
            "style_options_panel_padding" => if let Some(v) = parse_padding_value(value) { self.style_options_panel_padding = v; },
            "style_commands_panel_padding" => if let Some(v) = parse_padding_value(value) { self.style_commands_panel_padding = v; },
            "style_options_table_padding" => if let Some(v) = parse_padding_value(value) { self.style_options_table_padding = v; },
            "style_commands_table_padding" => if let Some(v) = parse_padding_value(value) { self.style_commands_table_padding = v; },
            "panel_title_padding" => if let Some(v) = value.as_u64() { self.panel_title_padding = v as usize; },
            "align_options_panel" => if let Some(v) = parse_align(value) { self.align_options_panel = v; },
            "align_commands_panel" => if let Some(v) = parse_align(value) { self.align_commands_panel = v; },
            "align_errors_panel" => if let Some(v) = parse_align(value) { self.align_errors_panel = v; },
            "panel_inline_help_in_title" => if let Some(v) = value.as_bool() { self.panel_inline_help_in_title = v; },
            "panel_inline_help_delimiter" => if let Some(v) = value.as_str() { self.panel_inline_help_delimiter = v.to_string(); },
            "options_table_column_types" => if let Some(v) = parse_string_list(value) { self.options_table_column_types = v; },
            "commands_table_column_types" => if let Some(v) = parse_string_list(value) { self.commands_table_column_types = v; },
            "options_table_help_sections" => if let Some(v) = parse_string_list(value) { self.options_table_help_sections = v; },
            "commands_table_help_sections" => if let Some(v) = parse_string_list(value) { self.commands_table_help_sections = v; },
            "show_arguments" => self.show_arguments = value.as_bool(),
            "show_commands" => self.show_commands = value.as_bool(),
            "show_metavars_column" => self.show_metavars_column = value.as_bool(),
            "commands_before_options" => if let Some(v) = value.as_bool() { self.commands_before_options = v; },
            "default_panels_first" => if let Some(v) = value.as_bool() { self.default_panels_first = v; },
            "append_metavars_help" => self.append_metavars_help = value.as_bool(),
            "group_arguments_options" => if let Some(v) = value.as_bool() { self.group_arguments_options = v; },
            "option_envvar_first" => self.option_envvar_first = value.as_bool(),
            "text_markup" => if let Some(v) = parse_text_markup(value) { self.text_markup = v; },
            "text_emojis" => self.text_emojis = value.as_bool(),
            "text_paragraph_linebreaks" => if let Some(v) = value.as_str() { self.text_paragraph_linebreaks = Some(v.to_string()); },
            "use_click_short_help" => if let Some(v) = value.as_bool() { self.use_click_short_help = v; },
            "helptext_show_aliases" => if let Some(v) = value.as_bool() { self.helptext_show_aliases = v; },
            "command_aliases" => if let Some(v) = parse_alias_map(value) { self.command_aliases = v; },
            "option_groups" => if let Some(v) = parse_group_list(value, "options") { self.option_groups = v; },
            "command_groups" => if let Some(v) = parse_group_list(value, "commands") { self.command_groups = v; },
            "padding_header_text" => if let Some(v) = parse_padding_value(value) { self.padding_header_text = v; },
            "padding_usage" => if let Some(v) = parse_padding_value(value) { self.padding_usage = v; },
            "padding_helptext" => if let Some(v) = parse_padding_value(value) { self.padding_helptext = v; },
            "padding_helptext_deprecated" => if let Some(v) = parse_padding_value(value) { self.padding_helptext_deprecated = v; },
            "padding_helptext_first_line" => if let Some(v) = parse_padding_value(value) { self.padding_helptext_first_line = v; },
            "padding_epilog" => if let Some(v) = parse_padding_value(value) { self.padding_epilog = v; },
            "padding_footer_text" => if let Some(v) = parse_padding_value(value) { self.padding_footer_text = v; },
            "padding_errors_panel" => if let Some(v) = parse_padding_value(value) { self.padding_errors_panel = v; },
            "padding_errors_suggestion" => if let Some(v) = parse_padding_value(value) { self.padding_errors_suggestion = v; },
            "padding_errors_epilogue" => if let Some(v) = parse_padding_value(value) { self.padding_errors_epilogue = v; },
            _ => {}
        }
    }

    fn sync_render_config(&mut self) {
        if self.text_emojis.is_none() {
            self.text_emojis = Some(matches!(
                self.text_markup,
                TextMarkup::Markdown | TextMarkup::Rich
            ));
        }

        self.panel_options.box_type = self.style_options_panel_box.unwrap_or(ROUNDED);
        self.panel_options.border_style = self.style_options_panel_border;
        self.panel_options.title_style = self.style_options_panel_title_style;
        self.panel_options.panel_style = self.style_options_panel_style;
        self.panel_options.padding = self.style_options_panel_padding;
        self.panel_options.align = self.align_options_panel;
        self.panel_options.expand = true;

        self.panel_commands.box_type = self.style_commands_panel_box.unwrap_or(ROUNDED);
        self.panel_commands.border_style = self.style_commands_panel_border;
        self.panel_commands.title_style = self.style_commands_panel_title_style;
        self.panel_commands.panel_style = self.style_commands_panel_style;
        self.panel_commands.padding = self.style_commands_panel_padding;
        self.panel_commands.align = self.align_commands_panel;
        self.panel_commands.expand = true;

        self.panel_arguments = self.panel_options.clone();

        let padding = self.style_options_table_padding.unpack();
        self.table_options.show_lines = self.style_options_table_show_lines;
        self.table_options.leading = self.style_options_table_leading;
        self.table_options.pad_edge = self.style_options_table_pad_edge;
        self.table_options.padding = (padding.3, padding.1);
        self.table_options.collapse_padding = self.style_options_table_collapse_padding;
        self.table_options.expand = self.style_options_table_expand;
        self.table_options.box_type = self.style_options_table_box;
        self.table_options.border_style = self.style_options_table_border_style;

        let padding = self.style_commands_table_padding.unpack();
        self.table_commands.show_lines = self.style_commands_table_show_lines;
        self.table_commands.leading = self.style_commands_table_leading;
        self.table_commands.pad_edge = self.style_commands_table_pad_edge;
        self.table_commands.padding = (padding.3, padding.1);
        self.table_commands.collapse_padding = self.style_commands_table_collapse_padding;
        self.table_commands.expand = self.style_commands_table_expand;
        self.table_commands.box_type = self.style_commands_table_box;
        self.table_commands.border_style = self.style_commands_table_border_style;

        self.table_arguments = self.table_options.clone();
    }
}

fn apply_terminal_width_env(cfg: &mut RichHelpConfig) {
    let width = std::env::var("TERMINAL_WIDTH").ok();
    if let Some(value) = width {
        if let Ok(parsed) = value.trim().parse::<usize>() {
            if cfg.width.is_none() {
                cfg.width = Some(parsed);
            }
            if cfg.max_width.is_none() {
                cfg.max_width = Some(parsed);
            }
        }
    }
}

fn apply_force_terminal_env(cfg: &mut RichHelpConfig) {
    if cfg.force_terminal.is_some() {
        return;
    }
    for key in ["FORCE_COLOR", "PY_COLORS", "GITHUB_ACTIONS"] {
        if let Ok(value) = std::env::var(key) {
            if let Some(parsed) = parse_truthy(&value) {
                cfg.force_terminal = Some(parsed);
                return;
            }
        }
    }
}

fn parse_truthy(value: &str) -> Option<bool> {
    match value.trim().to_ascii_lowercase().as_str() {
        "1" | "true" | "yes" | "on" => Some(true),
        "0" | "false" | "no" | "off" => Some(false),
        _ => None,
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

    pub fn text_markup(mut self, markup: TextMarkup) -> Self {
        self.config.text_markup = markup;
        self
    }

    pub fn text_emojis(mut self, enabled: bool) -> Self {
        self.config.text_emojis = Some(enabled);
        self
    }

    pub fn text_paragraph_linebreaks(mut self, value: &str) -> Self {
        self.config.text_paragraph_linebreaks = Some(value.to_string());
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

fn parse_optional_style(value: &serde_json::Value) -> Option<Style> {
    if value.is_null() {
        return None;
    }
    value.as_str().map(parse_style)
}

fn parse_style_list(value: &serde_json::Value) -> Option<Vec<Style>> {
    let list = value.as_array()?;
    let mut styles = Vec::with_capacity(list.len());
    for item in list {
        if let Some(s) = item.as_str() {
            styles.push(parse_style(s));
        }
    }
    Some(styles)
}

fn parse_string_list(value: &serde_json::Value) -> Option<Vec<String>> {
    let list = value.as_array()?;
    let mut out = Vec::with_capacity(list.len());
    for item in list {
        if let Some(s) = item.as_str() {
            out.push(s.to_string());
        }
    }
    Some(out)
}

fn parse_alias_map(value: &serde_json::Value) -> Option<HashMap<String, Vec<String>>> {
    let obj = value.as_object()?;
    let mut map = HashMap::new();
    for (key, val) in obj {
        if let Some(list) = parse_string_list(val) {
            map.insert(key.to_string(), list);
        }
    }
    Some(map)
}

fn parse_group_list(value: &serde_json::Value, key: &str) -> Option<Vec<GroupConfig>> {
    let list = value.as_array()?;
    let mut groups = Vec::new();
    for item in list {
        let obj = item.as_object()?;
        let name = obj.get("name").and_then(|v| v.as_str()).unwrap_or("Group").to_string();
        let items = obj
            .get(key)
            .and_then(parse_string_list)
            .or_else(|| obj.get("items").and_then(parse_string_list))
            .unwrap_or_default();
        let help = obj.get("help").and_then(|v| v.as_str()).map(|v| v.to_string());
        let inline_help_in_title = obj.get("inline_help_in_title").and_then(|v| v.as_bool());
        let title_style = obj.get("title_style").and_then(|v| v.as_str()).map(parse_style);
        let help_style = obj.get("help_style").and_then(|v| v.as_str()).map(parse_style);
        groups.push(GroupConfig {
            name,
            items,
            help,
            inline_help_in_title,
            title_style,
            help_style,
        });
    }
    Some(groups)
}

fn parse_padding_value(value: &serde_json::Value) -> Option<PaddingDimensions> {
    if let Some(v) = value.as_u64() {
        return Some(PaddingDimensions::from(v as usize));
    }
    let list = value.as_array()?;
    match list.len() {
        1 => list[0].as_u64().map(|v| PaddingDimensions::from(v as usize)),
        2 => {
            let v0 = list[0].as_u64()?;
            let v1 = list[1].as_u64()?;
            Some(PaddingDimensions::from((v0 as usize, v1 as usize)))
        }
        4 => {
            let v0 = list[0].as_u64()?;
            let v1 = list[1].as_u64()?;
            let v2 = list[2].as_u64()?;
            let v3 = list[3].as_u64()?;
            Some(PaddingDimensions::from((v0 as usize, v1 as usize, v2 as usize, v3 as usize)))
        }
        _ => None,
    }
}

fn parse_ratio_pair(value: &serde_json::Value) -> Option<(Option<usize>, Option<usize>)> {
    let list = value.as_array()?;
    if list.len() != 2 {
        return None;
    }
    let left = list[0].as_u64().map(|v| v as usize);
    let right = list[1].as_u64().map(|v| v as usize);
    Some((left, right))
}

fn parse_box_value(value: &serde_json::Value) -> Option<rich_rs::r#box::Box> {
    if value.is_null() {
        return None;
    }
    let name = value.as_str()?.to_ascii_uppercase();
    if name == "BLANK" || name == "NONE" {
        return None;
    }
    Some(match name.as_str() {
        "ASCII" => ASCII,
        "ASCII2" => ASCII2,
        "ASCII_DOUBLE_HEAD" => ASCII_DOUBLE_HEAD,
        "SQUARE" => SQUARE,
        "SQUARE_DOUBLE_HEAD" => SQUARE_DOUBLE_HEAD,
        "MINIMAL" => MINIMAL,
        "MINIMAL_HEAVY_HEAD" => MINIMAL_HEAVY_HEAD,
        "MINIMAL_DOUBLE_HEAD" => MINIMAL_DOUBLE_HEAD,
        "SIMPLE" => SIMPLE,
        "SIMPLE_HEAD" => SIMPLE_HEAD,
        "SIMPLE_HEAVY" => SIMPLE_HEAVY,
        "HORIZONTALS" => HORIZONTALS,
        "HORIZONTALS_DOUBLE_TOP" => HORIZONTALS,
        "ROUNDED" => ROUNDED,
        "HEAVY" => HEAVY,
        "HEAVY_EDGE" => HEAVY_EDGE,
        "HEAVY_HEAD" => HEAVY_HEAD,
        "DOUBLE" => DOUBLE,
        "DOUBLE_EDGE" => DOUBLE_EDGE,
        "MARKDOWN" => MARKDOWN,
        _ => return None,
    })
}

fn parse_align(value: &serde_json::Value) -> Option<AlignMethod> {
    match value.as_str()? {
        "left" | "Left" | "LEFT" => Some(AlignMethod::Left),
        "center" | "Center" | "CENTER" => Some(AlignMethod::Center),
        "right" | "Right" | "RIGHT" => Some(AlignMethod::Right),
        _ => None,
    }
}

fn parse_color_system_mode(value: &serde_json::Value) -> Option<ColorSystemMode> {
    match value.as_str()? {
        "auto" => Some(ColorSystemMode::Auto),
        "standard" => Some(ColorSystemMode::Standard),
        "256" => Some(ColorSystemMode::EightBit),
        "truecolor" => Some(ColorSystemMode::TrueColor),
        "windows" => Some(ColorSystemMode::Windows),
        "none" => Some(ColorSystemMode::None),
        _ => None,
    }
}

fn parse_text_markup(value: &serde_json::Value) -> Option<TextMarkup> {
    match value.as_str()? {
        "ansi" => Some(TextMarkup::Ansi),
        "rich" => Some(TextMarkup::Rich),
        "markdown" => Some(TextMarkup::Markdown),
        "none" => Some(TextMarkup::None),
        _ => None,
    }
}
