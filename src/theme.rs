use rich_rs::r#box::{HORIZONTALS, MINIMAL, ROUNDED};
use rich_rs::{PaddingDimensions, Style};

use crate::config::RichHelpConfig;

#[derive(Debug, Clone)]
pub struct ThemeError {
    pub message: String,
}

impl ThemeError {
    fn unknown(name: &str) -> Self {
        Self {
            message: format!("unknown theme '{name}'"),
        }
    }
}

pub fn apply_theme(cfg: &mut RichHelpConfig, theme: &str) -> Result<(), ThemeError> {
    let theme = theme.trim();
    if theme.is_empty() {
        return Err(ThemeError::unknown(theme));
    }

    let (color_name, format_name) = split_theme(theme);

    let color_ok = apply_color_theme(cfg, color_name);
    let format_ok = apply_format_theme(cfg, format_name);

    if color_ok && format_ok {
        return Ok(());
    }

    if color_ok && format_name.is_empty() {
        apply_format_box(cfg);
        return Ok(());
    }

    if format_ok && color_name.is_empty() {
        apply_color_default(cfg);
        return Ok(());
    }

    Err(ThemeError::unknown(theme))
}

pub fn list_themes() -> Vec<&'static str> {
    vec![
        "default",
        "default-box",
        "solarized",
        "nord",
        "box",
        "nu",
        "slim",
    ]
}

fn split_theme(theme: &str) -> (&str, &str) {
    if theme == "default" || theme == "default-box" {
        return ("default", "box");
    }
    if theme == "box" || theme == "nu" || theme == "slim" {
        return ("", theme);
    }
    if theme == "solarized" || theme == "nord" {
        return (theme, "box");
    }
    if let Some((left, right)) = theme.split_once('-') {
        return (left, right);
    }
    (theme, "")
}

fn apply_color_theme(cfg: &mut RichHelpConfig, name: &str) -> bool {
    match name {
        "default" => {
            apply_color_default(cfg);
            true
        }
        "solarized" => {
            apply_color_solarized(cfg);
            true
        }
        "nord" => {
            apply_color_nord(cfg);
            true
        }
        _ => false,
    }
}

fn apply_format_theme(cfg: &mut RichHelpConfig, name: &str) -> bool {
    match name {
        "" => false,
        "box" => {
            apply_format_box(cfg);
            true
        }
        "nu" => {
            apply_format_nu(cfg);
            true
        }
        "slim" => {
            apply_format_slim(cfg);
            true
        }
        _ => false,
    }
}

fn apply_color_default(cfg: &mut RichHelpConfig) {
    cfg.style_option = parse_style("bold cyan");
    cfg.style_option_negative = None;
    cfg.style_argument = parse_style("bold cyan");
    cfg.style_command = parse_style("bold cyan");
    cfg.style_command_aliases = parse_style("bold green");
    cfg.style_switch = parse_style("bold green");
    cfg.style_switch_negative = None;
    cfg.style_metavar = parse_style("bold yellow");
    cfg.style_metavar_append = parse_style("dim yellow");
    cfg.style_metavar_separator = parse_style("dim");
    cfg.style_range_append = None;
    cfg.style_usage = parse_style("yellow");
    cfg.style_usage_command = parse_style("bold");
    cfg.style_usage_separator = Style::default();
    cfg.style_deprecated = parse_style("red");
    cfg.style_options_panel_help_style = parse_style("dim");
    cfg.style_commands_panel_help_style = parse_style("dim");
    cfg.style_options_table_border_style = parse_style("dim");
    cfg.style_commands_table_border_style = parse_style("dim");
    cfg.style_options_panel_border = parse_style("dim");
    cfg.style_commands_panel_border = parse_style("dim");
    cfg.style_options_panel_title_style = Style::default();
    cfg.style_commands_panel_title_style = Style::default();
    cfg.style_required_long = parse_style("dim red");
    cfg.style_required_short = parse_style("red");
    cfg.style_option_help = Style::default();
    cfg.style_command_help = Style::default();
    cfg.style_option_default = parse_style("dim");
    cfg.style_option_envvar = parse_style("dim yellow");
    cfg.style_helptext_first_line = Style::default();
    cfg.style_helptext = parse_style("dim");
    cfg.style_header_text = Style::default();
    cfg.style_epilog_text = Style::default();
    cfg.style_footer_text = Style::default();
    cfg.style_options_panel_style = Style::default();
    cfg.style_commands_panel_style = Style::default();
    cfg.style_padding_usage = Style::default();
    cfg.style_padding_helptext = Style::default();
    cfg.style_padding_epilog = Style::default();
    cfg.style_padding_errors = Style::default();
    cfg.style_errors_panel_border = parse_style("red");
}

fn apply_color_solarized(cfg: &mut RichHelpConfig) {
    cfg.style_option = parse_style("#2aa198");
    cfg.style_option_negative = None;
    cfg.style_argument = parse_style("#d33682");
    cfg.style_command = parse_style("#2aa198");
    cfg.style_command_aliases = parse_style("#859900");
    cfg.style_switch = parse_style("#859900");
    cfg.style_switch_negative = None;
    cfg.style_metavar = parse_style("#b58900");
    cfg.style_metavar_append = parse_style("#b58900");
    cfg.style_metavar_separator = Style::default();
    cfg.style_range_append = None;
    cfg.style_usage = parse_style("#b58900");
    cfg.style_usage_command = Style::default();
    cfg.style_usage_separator = Style::default();
    cfg.style_options_panel_help_style = parse_style("dim");
    cfg.style_commands_panel_help_style = parse_style("dim");
    cfg.style_deprecated = parse_style("#6c71c4");
    cfg.style_options_table_border_style = parse_style("dim #268bd2");
    cfg.style_commands_table_border_style = parse_style("dim #268bd2");
    cfg.style_options_panel_border = parse_style("dim #268bd2");
    cfg.style_commands_panel_border = parse_style("dim #268bd2");
    cfg.style_options_panel_title_style = parse_style("#268bd2 not dim");
    cfg.style_commands_panel_title_style = parse_style("#268bd2 not dim");
    cfg.style_required_long = parse_style("#dc322f");
    cfg.style_required_short = parse_style("#dc322f");
    cfg.style_option_help = Style::default();
    cfg.style_command_help = Style::default();
    cfg.style_option_default = parse_style("#d33682");
    cfg.style_option_envvar = parse_style("#d33682");
    cfg.style_helptext_first_line = Style::default();
    cfg.style_helptext = parse_style("dim");
    cfg.style_header_text = Style::default();
    cfg.style_epilog_text = Style::default();
    cfg.style_footer_text = Style::default();
    cfg.style_options_panel_style = Style::default();
    cfg.style_commands_panel_style = Style::default();
    cfg.style_padding_usage = Style::default();
    cfg.style_padding_helptext = Style::default();
    cfg.style_padding_epilog = Style::default();
    cfg.style_padding_errors = Style::default();
    cfg.style_errors_panel_border = parse_style("#dc322f");
}

fn apply_color_nord(cfg: &mut RichHelpConfig) {
    cfg.style_option = parse_style("#5e81ac");
    cfg.style_option_negative = None;
    cfg.style_argument = parse_style("#b48ead");
    cfg.style_command = parse_style("#5e81ac");
    cfg.style_command_aliases = parse_style("#a3be8c");
    cfg.style_switch = parse_style("#a3be8c");
    cfg.style_switch_negative = None;
    cfg.style_metavar = parse_style("#ebcb8b");
    cfg.style_metavar_append = parse_style("#ebcb8b");
    cfg.style_metavar_separator = Style::default();
    cfg.style_range_append = None;
    cfg.style_usage = parse_style("#ebcb8b");
    cfg.style_usage_command = Style::default();
    cfg.style_usage_separator = Style::default();
    cfg.style_options_panel_help_style = parse_style("dim");
    cfg.style_commands_panel_help_style = parse_style("dim");
    cfg.style_deprecated = parse_style("#b48ead");
    cfg.style_options_table_border_style = parse_style("dim #5e81ac");
    cfg.style_commands_table_border_style = parse_style("dim #5e81ac");
    cfg.style_options_panel_border = parse_style("dim #5e81ac");
    cfg.style_commands_panel_border = parse_style("dim #5e81ac");
    cfg.style_options_panel_title_style = parse_style("#5e81ac not dim");
    cfg.style_commands_panel_title_style = parse_style("#5e81ac not dim");
    cfg.style_required_long = parse_style("#bf616a");
    cfg.style_required_short = parse_style("#bf616a");
    cfg.style_option_help = Style::default();
    cfg.style_command_help = Style::default();
    cfg.style_option_default = parse_style("#b48ead");
    cfg.style_option_envvar = parse_style("#b48ead");
    cfg.style_helptext_first_line = Style::default();
    cfg.style_helptext = parse_style("dim");
    cfg.style_header_text = Style::default();
    cfg.style_epilog_text = Style::default();
    cfg.style_footer_text = Style::default();
    cfg.style_options_panel_style = Style::default();
    cfg.style_commands_panel_style = Style::default();
    cfg.style_padding_usage = Style::default();
    cfg.style_padding_helptext = Style::default();
    cfg.style_padding_epilog = Style::default();
    cfg.style_padding_errors = Style::default();
    cfg.style_errors_panel_border = parse_style("#bf616a");
}

fn apply_format_box(cfg: &mut RichHelpConfig) {
    cfg.style_options_panel_box = Some(ROUNDED);
    cfg.style_commands_panel_box = Some(ROUNDED);
    cfg.style_errors_panel_box = Some(ROUNDED);
    cfg.style_options_table_box = None;
    cfg.style_commands_table_box = None;
    cfg.style_options_table_expand = true;
    cfg.style_commands_table_expand = true;
    cfg.style_options_panel_padding = PaddingDimensions::from((0, 1));
    cfg.style_commands_panel_padding = PaddingDimensions::from((0, 1));
    cfg.panel_inline_help_in_title = false;
    cfg.panel_inline_help_delimiter = " - ".to_string();
    cfg.options_table_column_types = vec![
        "required".to_string(),
        "opt_long".to_string(),
        "opt_short".to_string(),
        "metavar".to_string(),
        "help".to_string(),
    ];
    cfg.commands_table_column_types = vec!["name".to_string(), "aliases".to_string(), "help".to_string()];
    cfg.options_table_help_sections = vec![
        "help".to_string(),
        "deprecated".to_string(),
        "envvar".to_string(),
        "default".to_string(),
        "required".to_string(),
    ];
    cfg.commands_table_help_sections = vec!["help".to_string(), "deprecated".to_string()];
    cfg.panel_title_string = "{}".to_string();
    cfg.deprecated_string = "[deprecated]".to_string();
    cfg.deprecated_with_reason_string = "[deprecated: {}]".to_string();
    cfg.default_string = "[default: {}]".to_string();
    cfg.envvar_string = "[env var: {}]".to_string();
    cfg.required_long_string = "[required]".to_string();
    cfg.range_string = "[{}]".to_string();
    cfg.append_range_help_string = "[range: {}]".to_string();
    cfg.required_short_string = "*".to_string();
    cfg.padding_header_text = PaddingDimensions::from((1, 1, 0, 1));
    cfg.padding_helptext = PaddingDimensions::from((0, 1, 1, 1));
    cfg.padding_usage = PaddingDimensions::from(1);
    cfg.delimiter_comma = ",".to_string();
    cfg.delimiter_slash = "/".to_string();
    cfg.panel_title_padding = 1;
    cfg.padding_epilog = PaddingDimensions::from(1);
    cfg.padding_footer_text = PaddingDimensions::from(1);
    cfg.append_metavars_help_string = "({})".to_string();
}

fn apply_format_nu(cfg: &mut RichHelpConfig) {
    cfg.style_options_panel_box = Some(HORIZONTALS);
    cfg.style_commands_panel_box = Some(HORIZONTALS);
    cfg.style_errors_panel_box = Some(ROUNDED);
    cfg.style_options_table_box = None;
    cfg.style_commands_table_box = None;
    cfg.style_options_table_expand = false;
    cfg.style_commands_table_expand = false;
    cfg.panel_title_string = "{}".to_string();
    cfg.style_options_panel_padding = PaddingDimensions::from(0);
    cfg.style_commands_panel_padding = PaddingDimensions::from(0);
    cfg.deprecated_string = "(Deprecated)".to_string();
    cfg.deprecated_with_reason_string = "(Deprecated: {})".to_string();
    cfg.default_string = "(Default: {})".to_string();
    cfg.envvar_string = "(Env: {})".to_string();
    cfg.required_long_string = "(Required)".to_string();
    cfg.range_string = "{}".to_string();
    cfg.append_range_help_string = "(Range: {})".to_string();
    cfg.append_metavars_help_string = "[{}]".to_string();
    cfg.required_short_string = "#".to_string();
    cfg.padding_header_text = PaddingDimensions::from((0, 1, 1, 1));
    cfg.padding_helptext = PaddingDimensions::from((0, 1, 1, 1));
    cfg.padding_usage = PaddingDimensions::from((0, 1, 1, 1));
    cfg.delimiter_comma = ",".to_string();
    cfg.delimiter_slash = "/".to_string();
    cfg.panel_title_padding = 1;
    cfg.options_table_column_types = vec![
        "required".to_string(),
        "opt_long".to_string(),
        "opt_short".to_string(),
        "help".to_string(),
    ];
    cfg.commands_table_column_types = vec!["name".to_string(), "aliases".to_string(), "help".to_string()];
    cfg.options_table_help_sections = vec![
        "help".to_string(),
        "metavar".to_string(),
        "required".to_string(),
        "default".to_string(),
        "envvar".to_string(),
        "deprecated".to_string(),
    ];
    cfg.commands_table_help_sections = vec!["help".to_string(), "deprecated".to_string()];
    cfg.panel_inline_help_delimiter = " - ".to_string();
    cfg.padding_epilog = PaddingDimensions::from((0, 0, 1, 1));
    cfg.padding_footer_text = PaddingDimensions::from((0, 0, 1, 1));
    cfg.panel_inline_help_in_title = false;
}

fn apply_format_slim(cfg: &mut RichHelpConfig) {
    cfg.style_options_panel_box = Some(MINIMAL);
    cfg.style_commands_panel_box = Some(MINIMAL);
    cfg.style_errors_panel_box = Some(ROUNDED);
    cfg.style_options_table_box = None;
    cfg.style_commands_table_box = None;
    cfg.style_options_table_expand = false;
    cfg.style_commands_table_expand = false;
    cfg.panel_title_string = "{}:".to_string();
    cfg.style_options_panel_padding = PaddingDimensions::from((0, 0, 0, 1));
    cfg.style_commands_panel_padding = PaddingDimensions::from((0, 0, 0, 1));
    cfg.padding_header_text = PaddingDimensions::from((0, 0, 1, 0));
    cfg.padding_helptext = PaddingDimensions::from((0, 0, 1, 0));
    cfg.padding_usage = PaddingDimensions::from((0, 0, 1, 0));
    cfg.padding_epilog = PaddingDimensions::from((0, 0, 1, 0));
    cfg.padding_footer_text = PaddingDimensions::from((0, 0, 1, 0));
    cfg.default_string = "[default={}]".to_string();
    cfg.options_table_column_types = vec![
        "opt_short".to_string(),
        "opt_long_metavar".to_string(),
        "help".to_string(),
    ];
    cfg.commands_table_column_types = vec!["name_with_aliases".to_string(), "help".to_string()];
    cfg.options_table_help_sections = vec!["help".to_string(), "default".to_string(), "envvar".to_string()];
    cfg.commands_table_help_sections = vec!["help".to_string(), "deprecated".to_string()];
}

fn parse_style(input: &str) -> Style {
    Style::parse(input).unwrap_or_default()
}
