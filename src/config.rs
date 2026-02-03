use rich_rs::r#box::ROUNDED;
use rich_rs::{AlignMethod, ColorSystem, PaddingDimensions, Style};

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
    pub width: Option<usize>,
    pub max_width: Option<usize>,
    pub color_system: Option<ColorSystem>,
    pub force_terminal: Option<bool>,

    pub style_usage: Style,
    pub style_usage_command: Style,
    pub style_help: Style,
    pub style_option: Style,
    pub style_option_help: Style,
    pub style_argument: Style,
    pub style_command: Style,
    pub style_command_help: Style,
    pub style_metavar: Style,
    pub style_deprecated: Style,

    pub options_panel_title: String,
    pub arguments_panel_title: String,
    pub commands_panel_title: String,

    pub show_arguments: bool,
    pub show_commands: bool,
    pub commands_before_options: bool,

    pub panel: PanelConfig,
    pub table: TableConfig,
}

impl Default for RichHelpConfig {
    fn default() -> Self {
        Self {
            width: None,
            max_width: None,
            color_system: None,
            force_terminal: None,
            style_usage: parse_style("yellow"),
            style_usage_command: parse_style("bold"),
            style_help: parse_style("dim"),
            style_option: parse_style("bold cyan"),
            style_option_help: Style::default(),
            style_argument: parse_style("bold cyan"),
            style_command: parse_style("bold cyan"),
            style_command_help: Style::default(),
            style_metavar: parse_style("bold yellow"),
            style_deprecated: parse_style("red"),
            options_panel_title: "Options".to_string(),
            arguments_panel_title: "Arguments".to_string(),
            commands_panel_title: "Commands".to_string(),
            show_arguments: true,
            show_commands: true,
            commands_before_options: false,
            panel: PanelConfig::default(),
            table: TableConfig::default(),
        }
    }
}

fn parse_style(input: &str) -> Style {
    Style::parse(input).unwrap_or_default()
}
