use rich_rs::r#box::ROUNDED;
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
    match theme {
        "default" | "default-box" => {
            apply_default_box(cfg);
            Ok(())
        }
        _ => Err(ThemeError::unknown(theme)),
    }
}

fn apply_default_box(cfg: &mut RichHelpConfig) {
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

fn parse_style(input: &str) -> Style {
    Style::parse(input).unwrap_or_default()
}
