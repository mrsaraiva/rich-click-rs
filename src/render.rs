use std::io;
use std::sync::Arc;

use click::argument::Argument;
use click::command::Command;
use click::context::{
    get_current_context, pop_context, push_context, Context, ContextBuilder, HelpRenderer,
};
use click::error::ClickError;
use click::group::{CommandLike, Group};
use click::parameter::Parameter;
use rich_rs::markdown::Markdown;
use rich_rs::r#box::HORIZONTALS;
use rich_rs::{
    Column, Console, ConsoleOptions, Padding, PaddingDimensions, Panel, Row, Style, Table, Text,
};

use crate::config::{PanelConfig, RichHelpConfig, TableConfig, TextMarkup};

const VERSION_METAVAR_PREFIX: &str = "__click_version__:";

/// Rich help renderer.
#[derive(Debug, Clone)]
pub struct RichHelpRenderer {
    config: RichHelpConfig,
}

#[derive(Debug, Clone)]
struct CommandEntry {
    name: String,
    help: String,
    aliases: Vec<String>,
}

impl RichHelpRenderer {
    pub fn new(config: RichHelpConfig) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &RichHelpConfig {
        &self.config
    }

    pub fn render_command_help(&self, command: &Command, ctx: &Context) -> String {
        let mut console = self.create_capture_console();
        let _ = self.render_command_help_into(&mut console, command, ctx);
        console.get_captured()
    }

    pub fn render_group_help(&self, group: &Group, ctx: &Context) -> String {
        let mut console = self.create_capture_console();
        let _ = self.render_group_help_into(&mut console, group, ctx);
        console.get_captured()
    }

    pub fn render_error(&self, error: &ClickError) -> String {
        let mut console = self.create_capture_console();
        let _ = self.render_error_into(&mut console, error);
        console.get_captured()
    }

    fn render_command_help_into<W: io::Write>(
        &self,
        console: &mut Console<W>,
        command: &Command,
        ctx: &Context,
    ) -> io::Result<()> {
        let usage = command.get_usage(ctx);
        let usage_text = self.render_usage_text(&usage);
        self.print_with_padding(
            console,
            usage_text,
            self.config.padding_usage,
            self.config.style_padding_usage,
        )?;

        self.render_help_text(
            console,
            command.help.as_deref(),
            command.deprecated.as_deref(),
        )?;

        if self.is_slim_theme() {
            let options = self.collect_options(command, ctx);
            let arguments = if self.config.show_arguments.unwrap_or(false) {
                command
                    .arguments
                    .iter()
                    .filter_map(|arg| arg.get_help_record())
                    .filter(|(_, help)| !help.is_empty())
                    .collect::<Vec<_>>()
            } else {
                Vec::new()
            };
            self.render_slim_sections(console, &arguments, &options, &[])?;
            if let Some(epilog) = command.epilog.as_deref() {
                if !epilog.is_empty() {
                    console.print_text("")?;
                    console.print(
                        &Text::styled(epilog, self.config.style_helptext),
                        None,
                        None,
                        None,
                        false,
                        "\n",
                    )?;
                }
            }
            return Ok(());
        }

        let mut sections_printed = false;

        if self.config.show_arguments.unwrap_or(false) {
            if command.arguments.iter().any(|arg| !arg.hidden()) {
                self.print_section_spacing(console, &mut sections_printed)?;
                self.print_argument_panel(console, &command.arguments)?;
            }
        }

        let options = self.collect_options(command, ctx);
        if !options.is_empty() {
            self.print_section_spacing(console, &mut sections_printed)?;
            self.print_option_panels(console, &options)?;
        }

        if let Some(epilog) = command.epilog.as_deref() {
            if !epilog.is_empty() {
                console.print_text("")?;
                console.print(
                    &Text::styled(epilog, self.config.style_helptext),
                    None,
                    None,
                    None,
                    false,
                    "\n",
                )?;
            }
        }

        Ok(())
    }

    fn render_group_help_into<W: io::Write>(
        &self,
        console: &mut Console<W>,
        group: &Group,
        ctx: &Context,
    ) -> io::Result<()> {
        let usage = CommandLike::get_usage(group, ctx);
        let usage_text = self.render_usage_text(&usage);
        self.print_with_padding(
            console,
            usage_text,
            self.config.padding_usage,
            self.config.style_padding_usage,
        )?;

        self.render_help_text(
            console,
            group.command.help.as_deref(),
            group.command.deprecated.as_deref(),
        )?;

        if self.is_slim_theme() {
            let commands = self.collect_commands(group);
            let options = self.collect_options(&group.command, ctx);
            let arguments = if self.config.show_arguments.unwrap_or(false) {
                group
                    .command
                    .arguments
                    .iter()
                    .filter_map(|arg| arg.get_help_record())
                    .filter(|(_, help)| !help.is_empty())
                    .collect::<Vec<_>>()
            } else {
                Vec::new()
            };
            self.render_slim_sections(console, &arguments, &options, &commands)?;
            if let Some(epilog) = group.command.epilog.as_deref() {
                if !epilog.is_empty() {
                    console.print_text("")?;
                    console.print(
                        &Text::styled(epilog, self.config.style_helptext),
                        None,
                        None,
                        None,
                        false,
                        "\n",
                    )?;
                }
            }
            return Ok(());
        }

        let mut sections_printed = false;

        let commands = self.collect_commands(group);

        let options = self.collect_options(&group.command, ctx);
        let use_rule_options =
            self.use_rule_panels(&self.config.panel_options, &self.config.table_options);
        let use_rule_commands =
            self.use_rule_panels(&self.config.panel_commands, &self.config.table_commands);

        let show_args = self.config.show_arguments.unwrap_or(false);
        let has_arguments = show_args && group.command.arguments.iter().any(|arg| !arg.hidden());

        if self.config.commands_before_options {
            if !commands.is_empty() {
                self.print_section_spacing(console, &mut sections_printed)?;
                self.print_command_panels(console, &commands)?;
            }
        }

        if has_arguments {
            self.print_section_spacing(console, &mut sections_printed)?;
            self.print_argument_panel(console, &group.command.arguments)?;
        }

        if !options.is_empty() {
            self.print_section_spacing(console, &mut sections_printed)?;
            self.print_option_panels(console, &options)?;
            if !self.config.commands_before_options
                && !commands.is_empty()
                && (use_rule_options || use_rule_commands)
            {
                self.print_slim_blank(console)?;
            }
        }

        if !self.config.commands_before_options {
            if !commands.is_empty() {
                self.print_section_spacing(console, &mut sections_printed)?;
                self.print_command_panels(console, &commands)?;
            }
        }

        if let Some(epilog) = group.command.epilog.as_deref() {
            if !epilog.is_empty() {
                console.print_text("")?;
                console.print(
                    &Text::styled(epilog, self.config.style_helptext),
                    None,
                    None,
                    None,
                    false,
                    "\n",
                )?;
            }
        }

        Ok(())
    }

    fn render_help_text<W: io::Write>(
        &self,
        console: &mut Console<W>,
        help: Option<&str>,
        deprecated: Option<&str>,
    ) -> io::Result<()> {
        let mut rendered = false;
        if let Some(help_text) = help {
            if !help_text.trim().is_empty() {
                let mut decorated = help_text.to_string();
                if let Some(dep) = deprecated {
                    if dep.is_empty() {
                        decorated = format!("{} {}", decorated, self.config.deprecated_string);
                    } else {
                        decorated = format!(
                            "{} {}",
                            decorated,
                            self.config.deprecated_with_reason_string.replace("{}", dep)
                        );
                    }
                }
                if self.config.text_markup == TextMarkup::Markdown {
                    let decorated = self.apply_paragraph_linebreaks(&decorated);
                    let mut normalized = decorated.replace("\n\n>", "\n__RC_BLOCK__\n>");
                    normalized = normalized.replace("\n\n", "  \n");
                    normalized = normalized.replace("\n__RC_BLOCK__\n>", "\n\n>");
                    let md = Markdown::new(&normalized).with_hyperlinks(true);
                    self.print_with_padding(
                        console,
                        md,
                        self.config.padding_helptext,
                        self.config.style_padding_helptext,
                    )?;
                } else {
                    let help_text = self.render_help_text_block(&decorated, deprecated.is_some());
                    self.print_with_padding(
                        console,
                        help_text,
                        self.config.padding_helptext,
                        self.config.style_padding_helptext,
                    )?;
                }
                rendered = true;
            }
        }
        if !rendered {
            if let Some(dep) = deprecated {
                let dep_msg = if dep.is_empty() {
                    self.config.deprecated_string.clone()
                } else {
                    self.config.deprecated_with_reason_string.replace("{}", dep)
                };
                let dep_text = Text::styled(&dep_msg, self.config.style_deprecated);
                self.print_with_padding(
                    console,
                    dep_text,
                    self.config.padding_helptext,
                    self.config.style_padding_helptext,
                )?;
            }
        }
        Ok(())
    }

    fn render_error_into<W: io::Write>(
        &self,
        console: &mut Console<W>,
        error: &ClickError,
    ) -> io::Result<()> {
        let mut message = if matches!(error, ClickError::Abort) {
            self.config.aborted_text.clone()
        } else {
            error.format_full()
        };
        if let Some(source) = self.param_source_for_error(error) {
            let suffix = if self.config.errors_param_source_format.contains("{}") {
                self.config
                    .errors_param_source_format
                    .replace("{}", &source)
            } else {
                format!("{} {}", self.config.errors_param_source_format, source)
            };
            message = append_suffix_to_first_line(&message, &suffix);
        }
        let body = Text::styled(&message, self.config.style_padding_errors);
        let mut panel = Panel::new(Box::new(body))
            .with_border_style(self.config.style_errors_panel_border)
            .with_title_align(self.config.align_errors_panel)
            .with_padding(self.config.padding_errors_panel);
        if let Some(box_type) = self.config.style_errors_panel_box {
            panel = panel.with_box(box_type);
        }
        let title_text = self.build_panel_title(
            &self.config.errors_panel_title,
            self.config.style_errors_panel_border,
            None,
            self.config.style_errors_panel_border,
            false,
        );
        panel = panel.with_title_text(title_text);
        console.print(&panel, None, None, None, false, "")?;

        if let Some(ref suggestion) = self.config.errors_suggestion {
            let text = Text::styled(
                suggestion,
                self.config
                    .style_errors_suggestion
                    .unwrap_or(self.config.style_option_help),
            );
            console.print(&text, None, None, None, false, "\n")?;
        }
        if let Some(ref epilogue) = self.config.errors_epilogue {
            let text = Text::styled(epilogue, self.config.style_padding_errors);
            console.print(&text, None, None, None, false, "\n")?;
        }
        Ok(())
    }

    fn param_source_for_error(&self, error: &ClickError) -> Option<String> {
        if !self.config.errors_show_param_source {
            return None;
        }
        let ctx = get_current_context()?;
        let mut candidates: Vec<String> = Vec::new();
        match error {
            ClickError::BadParameter {
                param_name,
                param_hint,
                ..
            } => {
                if let Some(hints) = param_hint {
                    for hint in hints {
                        candidates.extend(collect_param_candidates(hint));
                    }
                }
                if let Some(name) = param_name {
                    candidates.extend(collect_param_candidates(name));
                }
            }
            ClickError::MissingParameter {
                param_name,
                param_hint,
                ..
            } => {
                if let Some(hints) = param_hint {
                    for hint in hints {
                        candidates.extend(collect_param_candidates(hint));
                    }
                }
                if let Some(name) = param_name {
                    candidates.extend(collect_param_candidates(name));
                }
            }
            ClickError::BadOptionUsage { option_name, .. } => {
                candidates.extend(collect_param_candidates(option_name));
            }
            _ => {}
        }

        for raw in candidates {
            if let Some(name) = normalize_param_name(&raw) {
                if let Some(source) = ctx.get_parameter_source(&name) {
                    return Some(source.to_string());
                }
            }
        }
        None
    }

    fn print_section_spacing<W: io::Write>(
        &self,
        _console: &mut Console<W>,
        printed_any: &mut bool,
    ) -> io::Result<()> {
        if !*printed_any {
            *printed_any = true;
        }
        Ok(())
    }

    fn print_option_panels<W: io::Write>(
        &self,
        console: &mut Console<W>,
        options: &[click::option::ClickOption],
    ) -> io::Result<()> {
        let mut remaining = options.to_vec();
        let mut panels = Vec::new();
        let mut panel_cfg = self.config.panel_options.clone();
        if self.config.table_options.leading > 0 && self.config.table_options.box_type.is_none() {
            let (top, right, bottom, left) = panel_cfg.padding.unpack();
            if top == 0 {
                panel_cfg.padding = PaddingDimensions::from((1, right, bottom, left));
            }
        }
        let mut rule_panels: Vec<(String, Table, Option<String>, Style, Option<Style>, bool)> =
            Vec::new();
        let use_rule = self.use_rule_panels(&self.config.panel_options, &self.config.table_options);

        if !self.config.option_groups.is_empty() {
            for group in &self.config.option_groups {
                let (group_items, rest) = self.partition_options(&remaining, &group.items);
                remaining = rest;
                if group_items.is_empty() {
                    continue;
                }
                let rows = self.build_option_rows(&group_items);
                let flex_last = self.should_flex_last_options(&group_items);
                let table =
                    self.build_table_from_rows(rows, &self.config.table_options, None, flex_last);
                let title_style = group
                    .title_style
                    .unwrap_or(self.config.panel_options.title_style);
                let help_style = group
                    .help_style
                    .unwrap_or(self.config.style_options_panel_help_style);
                let inline = group
                    .inline_help_in_title
                    .unwrap_or(self.config.panel_inline_help_in_title);
                if use_rule {
                    rule_panels.push((
                        group.name.clone(),
                        table,
                        group.help.clone(),
                        help_style,
                        Some(title_style),
                        inline,
                    ));
                } else {
                    let panel = self.build_panel(
                        &group.name,
                        table,
                        &panel_cfg,
                        group.help.as_deref(),
                        help_style,
                        Some(title_style),
                        inline,
                    );
                    panels.push(panel);
                }
            }
        }

        if !remaining.is_empty() {
            let rows = self.build_option_rows(&remaining);
            let flex_last = self.should_flex_last_options(&remaining);
            let table =
                self.build_table_from_rows(rows, &self.config.table_options, None, flex_last);
            if use_rule {
                rule_panels.push((
                    self.config.options_panel_title.clone(),
                    table,
                    None,
                    self.config.style_options_panel_help_style,
                    None,
                    self.config.panel_inline_help_in_title,
                ));
            } else {
                let panel = self.build_panel(
                    &self.config.options_panel_title,
                    table,
                    &panel_cfg,
                    None,
                    self.config.style_options_panel_help_style,
                    None,
                    self.config.panel_inline_help_in_title,
                );
                if self.config.default_panels_first {
                    panels.insert(0, panel);
                } else {
                    panels.push(panel);
                }
            }
        }

        if use_rule {
            let last = rule_panels.len().saturating_sub(1);
            for (idx, (title, table, help, help_style, title_style, inline)) in
                rule_panels.into_iter().enumerate()
            {
                self.render_rule_panel(
                    console,
                    &title,
                    table,
                    &panel_cfg,
                    help.as_deref(),
                    help_style,
                    title_style,
                    inline,
                )?;
                if idx < last {
                    self.print_slim_blank(console)?;
                }
            }
        } else {
            for panel in panels {
                console.print(&panel, None, None, None, false, "")?;
            }
        }
        Ok(())
    }

    fn print_command_panels<W: io::Write>(
        &self,
        console: &mut Console<W>,
        commands: &[CommandEntry],
    ) -> io::Result<()> {
        let mut remaining = commands.to_vec();
        let mut panels = Vec::new();
        let mut rule_panels: Vec<(String, Table, Option<String>, Style, Option<Style>, bool)> =
            Vec::new();
        let use_rule =
            self.use_rule_panels(&self.config.panel_commands, &self.config.table_commands);

        if !self.config.command_groups.is_empty() {
            for group in &self.config.command_groups {
                let (group_items, rest) = self.partition_commands(&remaining, &group.items);
                remaining = rest;
                if group_items.is_empty() {
                    continue;
                }
                let rows = self.build_command_rows(&group_items);
                let mut table_cfg = self.config.table_commands.clone();
                if use_rule {
                    let max_name_len = group_items
                        .iter()
                        .map(|entry| entry.name.len())
                        .max()
                        .unwrap_or(0);
                    if max_name_len <= 6 {
                        table_cfg.padding = (table_cfg.padding.0, 1);
                    }
                }
                let table = self.build_table_from_rows(
                    rows,
                    &table_cfg,
                    self.config.style_commands_table_column_width_ratio,
                    false,
                );
                let title_style = group
                    .title_style
                    .unwrap_or(self.config.panel_commands.title_style);
                let help_style = group
                    .help_style
                    .unwrap_or(self.config.style_commands_panel_help_style);
                let inline = group
                    .inline_help_in_title
                    .unwrap_or(self.config.panel_inline_help_in_title);
                if use_rule {
                    rule_panels.push((
                        group.name.clone(),
                        table,
                        group.help.clone(),
                        help_style,
                        Some(title_style),
                        inline,
                    ));
                } else {
                    let panel = self.build_panel(
                        &group.name,
                        table,
                        &self.config.panel_commands,
                        group.help.as_deref(),
                        help_style,
                        Some(title_style),
                        inline,
                    );
                    panels.push(panel);
                }
            }
        }

        if !remaining.is_empty() {
            let rows = self.build_command_rows(&remaining);
            let mut table_cfg = self.config.table_commands.clone();
            if use_rule {
                let max_name_len = remaining
                    .iter()
                    .map(|entry| entry.name.len())
                    .max()
                    .unwrap_or(0);
                if max_name_len <= 6 {
                    table_cfg.padding = (table_cfg.padding.0, 1);
                }
            }
            let table = self.build_table_from_rows(
                rows,
                &table_cfg,
                self.config.style_commands_table_column_width_ratio,
                false,
            );
            if use_rule {
                rule_panels.push((
                    self.config.commands_panel_title.clone(),
                    table,
                    None,
                    self.config.style_commands_panel_help_style,
                    None,
                    self.config.panel_inline_help_in_title,
                ));
            } else {
                let panel = self.build_panel(
                    &self.config.commands_panel_title,
                    table,
                    &self.config.panel_commands,
                    None,
                    self.config.style_commands_panel_help_style,
                    None,
                    self.config.panel_inline_help_in_title,
                );
                if self.config.default_panels_first {
                    panels.insert(0, panel);
                } else {
                    panels.push(panel);
                }
            }
        }

        if use_rule {
            let total = rule_panels.len();
            let last = total.saturating_sub(1);
            for (idx, (title, table, help, help_style, title_style, inline)) in
                rule_panels.into_iter().enumerate()
            {
                self.render_rule_panel(
                    console,
                    &title,
                    table,
                    &self.config.panel_commands,
                    help.as_deref(),
                    help_style,
                    title_style,
                    inline,
                )?;
                if idx < last {
                    self.print_slim_blank(console)?;
                }
            }
            if total > 0 {
                self.print_slim_blank(console)?;
            }
        } else {
            for panel in panels {
                console.print(&panel, None, None, None, false, "")?;
            }
        }
        Ok(())
    }

    fn print_argument_panel<W: io::Write>(
        &self,
        console: &mut Console<W>,
        arguments: &[Argument],
    ) -> io::Result<()> {
        let rows = self.build_argument_rows(arguments);
        let mut table_cfg = self.config.table_arguments.clone();
        table_cfg.padding = (0, 2);
        table_cfg.collapse_padding = true;
        table_cfg.expand = false;
        let table = self.build_table_from_rows(rows, &table_cfg, None, false);
        let panel = self.build_panel(
            &self.config.arguments_panel_title,
            table,
            &self.config.panel_arguments,
            None,
            self.config.style_options_panel_help_style,
            None,
            self.config.panel_inline_help_in_title,
        );
        console.print(&panel, None, None, None, false, "")
    }

    fn build_table_from_rows(
        &self,
        rows: Vec<Vec<Box<dyn rich_rs::Renderable + Send + Sync>>>,
        table_cfg: &TableConfig,
        ratios: Option<(Option<usize>, Option<usize>)>,
        flex_last: bool,
    ) -> Table {
        let mut table = if table_cfg.box_type.is_some() {
            Table::new().with_show_header(false).with_show_footer(false)
        } else {
            Table::grid()
        }
        .with_padding(table_cfg.padding.0, table_cfg.padding.1)
        .with_collapse_padding(table_cfg.collapse_padding)
        .with_pad_edge(table_cfg.pad_edge)
        .with_show_lines(table_cfg.show_lines)
        .with_leading(table_cfg.leading)
        .with_expand(table_cfg.expand)
        .with_border_style(table_cfg.border_style);

        if let Some(box_type) = table_cfg.box_type {
            table = table.with_box(Some(box_type));
            table = table.with_show_edge(true);
        }

        if !table_cfg.row_styles.is_empty() {
            table = table.with_row_styles(table_cfg.row_styles.clone());
        }

        if let Some(first_row) = rows.first() {
            let last_idx = first_row.len().saturating_sub(1);
            for idx in 0..first_row.len() {
                let mut col = Column::default();
                if let Some((left, right)) = ratios {
                    if idx == 0 {
                        col.ratio = left;
                    } else if idx == 1 {
                        col.ratio = right;
                    }
                }
                if flex_last && idx == last_idx && col.ratio.is_none() {
                    col.ratio = Some(1);
                }
                table.add_column(col);
            }
        }

        for row_cells in rows {
            let row = Row::new(row_cells);
            table.add_row(row);
        }

        table
    }

    fn build_panel(
        &self,
        title: &str,
        table: Table,
        panel_cfg: &PanelConfig,
        help_text: Option<&str>,
        help_style: Style,
        title_override: Option<Style>,
        inline_help: bool,
    ) -> Panel {
        let title_style = title_override.unwrap_or(panel_cfg.title_style);
        let title_text =
            self.build_panel_title(title, title_style, help_text, help_style, inline_help);
        Panel::new(Box::new(table))
            .with_box(panel_cfg.box_type)
            .with_title_text(title_text)
            .with_title_align(panel_cfg.align)
            .with_border_style(panel_cfg.border_style)
            .with_style(panel_cfg.panel_style)
            .with_padding(panel_cfg.padding)
            .with_expand(panel_cfg.expand)
    }

    fn render_rule_panel<W: io::Write>(
        &self,
        console: &mut Console<W>,
        title: &str,
        table: Table,
        panel_cfg: &PanelConfig,
        help_text: Option<&str>,
        help_style: Style,
        title_override: Option<Style>,
        inline_help: bool,
    ) -> io::Result<()> {
        let title_style = title_override.unwrap_or(panel_cfg.title_style);
        let title_text =
            self.build_panel_title(title, title_style, help_text, help_style, inline_help);
        let (_top, right, _bottom, left) = self.config.padding_helptext.unpack();
        let width = self.line_width();
        let left_pad = " ".repeat(left);

        let title_line = format!("{}{}", left_pad, title_text.plain_text());
        console.print_text(&self.pad_line(&title_line, width))?;

        let rule_width = width.saturating_sub(left + right);
        let rule_line = format!("{}{}", left_pad, "─".repeat(rule_width));
        console.print_text(&self.pad_line(&rule_line, width))?;

        self.print_with_padding_end(
            console,
            table,
            self.config.padding_helptext,
            self.config.style_padding_helptext,
            "",
        )
    }

    fn create_capture_console(&self) -> Console<Vec<u8>> {
        let mut options = ConsoleOptions::from_terminal();
        if let Some(width) = self.config.width {
            options.size.0 = width;
            options.max_width = width;
        }
        if let Some(max_width) = self.config.max_width {
            options.max_width = max_width;
        }
        options.color_system = self
            .config
            .color_system
            .to_color_system(options.color_system);
        if let Some(force_terminal) = self.config.force_terminal {
            options.is_terminal = force_terminal;
            if !force_terminal {
                options.color_system = None;
            }
        }
        Console::capture_with_options(options)
    }

    fn render_usage_text(&self, usage: &str) -> Text {
        let mut text = Text::styled(usage, self.config.style_usage);
        let prefix = "Usage: ";
        if usage.starts_with(prefix) {
            let start = prefix.len();
            let end = usage[start..]
                .find(' ')
                .map(|idx| start + idx)
                .unwrap_or_else(|| usage.len());
            if start < end {
                text.stylize(start, end, self.config.style_usage_command);
            }
        }
        text
    }

    fn render_help_text_block(&self, text: &str, has_deprecated: bool) -> Text {
        let source = self.apply_paragraph_linebreaks(text);
        match self.config.text_markup {
            TextMarkup::Markdown => Text::plain(text),
            TextMarkup::Rich => {
                let emojis = self.config.text_emojis.unwrap_or(true);
                let mut rendered =
                    Text::from_markup(&source, emojis).unwrap_or_else(|_| Text::plain(&source));
                self.apply_help_styles(&mut rendered, &source, has_deprecated, true);
                rendered
            }
            TextMarkup::Ansi => {
                let mut rendered = Text::from_ansi(&source);
                self.apply_help_styles(&mut rendered, &source, has_deprecated, true);
                rendered
            }
            TextMarkup::None => {
                let mut rendered = Text::plain(&source);
                self.apply_help_styles(&mut rendered, &source, has_deprecated, true);
                rendered
            }
        }
    }

    fn apply_help_styles(
        &self,
        rendered: &mut Text,
        raw: &str,
        has_deprecated: bool,
        style_base: bool,
    ) {
        let first_end = raw.find('\n').unwrap_or(raw.len());
        if style_base {
            if first_end < raw.len() {
                rendered.stylize(first_end, raw.len(), self.config.style_helptext);
            } else {
                rendered.stylize(0, raw.len(), self.config.style_helptext_first_line);
            }
        }
        if first_end > 0 {
            rendered.stylize(0, first_end, self.config.style_helptext_first_line);
        }

        if has_deprecated {
            if let Some(idx) = raw.rfind(&self.config.deprecated_string) {
                rendered.stylize(idx, raw.len(), self.config.style_deprecated);
            } else if let Some(idx) = raw.rfind("[deprecated:") {
                rendered.stylize(idx, raw.len(), self.config.style_deprecated);
            }
        }
    }

    fn apply_paragraph_linebreaks(&self, input: &str) -> String {
        let mut text = input;
        if let Some((before, _)) = input.split_once('\u{000c}') {
            text = before;
        }

        if self.config.text_markup == TextMarkup::Markdown {
            if let Some(ref breaks) = self.config.text_paragraph_linebreaks {
                if breaks == "\n" {
                    return text.replace("\n\n", "\n");
                }
                if breaks != "\n\n" {
                    return text.replace("\n\n", breaks);
                }
            }
            return text.to_string();
        }

        let joiner = self
            .config
            .text_paragraph_linebreaks
            .as_deref()
            .unwrap_or("\n");

        let paragraphs = text
            .split("\n\n")
            .map(|para| {
                if para.starts_with('\u{0008}') {
                    para.replace("\u{0008}\n", "").replace('\u{0008}', "")
                } else {
                    para.replace('\n', " ")
                }
            })
            .collect::<Vec<_>>();

        paragraphs.join(joiner)
    }

    fn normalize_help_text(&self, input: &str) -> String {
        let mut text = input;
        if let Some((before, _)) = input.split_once('\u{000c}') {
            text = before;
        }

        if self.config.text_markup == TextMarkup::Markdown {
            return text.to_string();
        }

        let paragraphs = text
            .split("\n\n")
            .map(|para| {
                if para.starts_with('\u{0008}') {
                    para.replace("\u{0008}\n", "").replace('\u{0008}', "")
                } else {
                    para.replace('\n', " ")
                }
            })
            .collect::<Vec<_>>();

        paragraphs.join("\n").trim().to_string()
    }

    fn normalize_first_paragraph(&self, input: &str) -> String {
        let mut text = input;
        if let Some((before, _)) = input.split_once('\u{000c}') {
            text = before;
        }

        let para = text.split("\n\n").next().unwrap_or("");
        let normalized = if self.config.text_markup == TextMarkup::Markdown {
            para.to_string()
        } else if para.starts_with('\u{0008}') {
            para.replace("\u{0008}\n", "").replace('\u{0008}', "")
        } else {
            para.replace('\n', " ")
        };

        normalized.trim().to_string()
    }

    fn print_with_padding<W: io::Write, R: rich_rs::Renderable + Send + Sync + 'static>(
        &self,
        console: &mut Console<W>,
        renderable: R,
        padding: PaddingDimensions,
        style: Style,
    ) -> io::Result<()> {
        let padded = Padding::new(Box::new(renderable), padding).with_style(style);
        console.print(&padded, None, None, None, false, "")
    }

    fn print_with_padding_end<W: io::Write, R: rich_rs::Renderable + Send + Sync + 'static>(
        &self,
        console: &mut Console<W>,
        renderable: R,
        padding: PaddingDimensions,
        style: Style,
        end: &str,
    ) -> io::Result<()> {
        let padded = Padding::new(Box::new(renderable), padding).with_style(style);
        console.print(&padded, None, None, None, false, end)
    }

    fn use_rule_panels(&self, panel_cfg: &PanelConfig, table_cfg: &TableConfig) -> bool {
        panel_cfg.box_type == HORIZONTALS
            && table_cfg.box_type.is_none()
            && self.config.panel_title_padding == 0
            && self.config.panel_inline_help_in_title
    }

    fn build_panel_title(
        &self,
        title: &str,
        style: Style,
        help: Option<&str>,
        help_style: Style,
        inline_help: bool,
    ) -> Text {
        let formatted = self.config.panel_title_string.replace("{}", title);
        let padding = self.config.panel_title_padding;
        let mut padded = String::new();
        for _ in 0..padding {
            padded.push(' ');
        }
        padded.push_str(&formatted);
        for _ in 0..padding {
            padded.push(' ');
        }
        if inline_help {
            if let Some(help_text) = help {
                let mut text = Text::styled(padded, style);
                text.append(
                    &self.config.panel_inline_help_delimiter,
                    Some(self.config.style_option_help),
                );
                text.append(help_text, Some(help_style));
                return text;
            }
        }
        Text::styled(padded, style)
    }

    fn collect_options(&self, command: &Command, ctx: &Context) -> Vec<click::option::ClickOption> {
        let mut options: Vec<click::option::ClickOption> = command
            .options
            .iter()
            .filter(|opt| !opt.hidden())
            .cloned()
            .collect();
        if let Some(help_opt) = command.get_help_option(ctx) {
            if !help_opt.hidden() {
                options.push(help_opt);
            }
        }
        options
    }

    fn collect_commands(&self, group: &Group) -> Vec<CommandEntry> {
        let mut commands = Vec::new();
        let mut seen = std::collections::HashSet::new();
        if self.config.show_commands.unwrap_or(true) {
            for (registered, cmd) in group.list_command_entries() {
                let ptr = cmd as *const dyn CommandLike as *const () as usize;
                if seen.contains(&ptr) {
                    continue;
                }
                seen.insert(ptr);
                if cmd.is_hidden() {
                    continue;
                }
                let canonical = cmd.name().unwrap_or(&registered).to_string();
                let mut aliases = group.list_command_aliases(&registered);
                if registered != canonical && !aliases.iter().any(|a| a == &registered) {
                    aliases.push(registered.clone());
                }
                if let Some(extra) = self.config.command_aliases.get(&registered) {
                    aliases.extend(extra.iter().cloned());
                }
                aliases.sort();
                aliases.dedup();
                aliases.retain(|a| a != &canonical);

                let mut help = if self.config.use_click_short_help {
                    cmd.get_short_help()
                } else {
                    let raw = if let Some(cmd) = cmd.as_any().downcast_ref::<Command>() {
                        cmd.short_help.as_deref().or(cmd.help.as_deref())
                    } else if let Some(group) = cmd.as_any().downcast_ref::<Group>() {
                        group
                            .command
                            .short_help
                            .as_deref()
                            .or(group.command.help.as_deref())
                    } else {
                        None
                    };
                    raw.map(|v| self.normalize_first_paragraph(v))
                        .unwrap_or_default()
                };
                if self.config.helptext_show_aliases && !aliases.is_empty() {
                    let joined = aliases.join(&self.config.delimiter_comma);
                    let alias_text = self.config.helptext_aliases_string.replace("{}", &joined);
                    if help.is_empty() {
                        help = alias_text;
                    } else {
                        help = format!("{}  {}", help, alias_text);
                    }
                }

                commands.push(CommandEntry {
                    name: canonical,
                    help,
                    aliases,
                });
            }
        }
        commands
    }

    fn build_option_rows(
        &self,
        options: &[click::option::ClickOption],
    ) -> Vec<Vec<Box<dyn rich_rs::Renderable + Send + Sync>>> {
        let mut rows = Vec::new();
        let mut column_types = self.config.options_table_column_types.clone();
        if self.config.show_metavars_column == Some(false) {
            column_types.retain(|c| !c.contains("metavar"));
        }
        if column_types.iter().any(|c| c == "required") && !options.iter().any(|opt| opt.required())
        {
            column_types.retain(|c| c != "required");
        }
        if column_types.iter().any(|c| c == "opt_short")
            && !options.iter().any(|opt| !opt.short.is_empty())
        {
            column_types.retain(|c| c != "opt_short");
        }
        if column_types.iter().any(|c| c == "metavar")
            && !options.iter().any(|opt| self.option_metavar(opt).is_some())
        {
            column_types.retain(|c| c != "metavar");
        }

        for opt in options {
            let mut row = Vec::new();
            for col in &column_types {
                let cell = self.build_option_column(opt, col);
                if let Some(renderable) = cell {
                    row.push(renderable);
                } else {
                    row.push(
                        Box::new(Text::plain("")) as Box<dyn rich_rs::Renderable + Send + Sync>
                    );
                }
            }
            rows.push(row);
        }

        rows
    }

    fn build_argument_rows(
        &self,
        arguments: &[Argument],
    ) -> Vec<Vec<Box<dyn rich_rs::Renderable + Send + Sync>>> {
        let mut rows = Vec::new();
        for arg in arguments {
            if arg.hidden() {
                continue;
            }
            let required = if arg.required() {
                Text::styled(
                    &self.config.required_short_string,
                    self.config.style_required_short,
                )
            } else {
                Text::plain("")
            };
            let name = Text::styled(arg.config.name.to_uppercase(), self.config.style_argument);
            let mut metavar_text = arg
                .type_converter()
                .get_metavar()
                .unwrap_or_else(|| arg.type_converter().name().to_string());
            if metavar_text.contains('|') && !metavar_text.contains('{') {
                metavar_text = format!("{{{}}}", metavar_text);
            }
            let metavar = Text::styled(metavar_text, self.config.style_metavar);
            let mut help_text = Text::styled("", self.config.style_option_help);
            if let Some(help) = arg.help() {
                let normalized = self.normalize_help_text(help);
                if !normalized.is_empty() {
                    help_text.append(normalized, Some(self.config.style_option_help));
                }
            }
            if arg.required() {
                if !help_text.plain_text().is_empty() {
                    help_text.append(" ", Some(self.config.style_option_help));
                }
                help_text.append(
                    self.config.required_long_string.clone(),
                    Some(self.config.style_required_long),
                );
            }
            let row = vec![
                Box::new(required) as Box<dyn rich_rs::Renderable + Send + Sync>,
                Box::new(name),
                Box::new(metavar),
                Box::new(help_text),
            ];
            rows.push(row);
        }
        rows
    }

    fn build_command_rows(
        &self,
        commands: &[CommandEntry],
    ) -> Vec<Vec<Box<dyn rich_rs::Renderable + Send + Sync>>> {
        let mut rows = Vec::new();
        let mut column_types = self.config.commands_table_column_types.clone();
        if column_types.iter().any(|c| c == "aliases")
            && commands.iter().all(|entry| entry.aliases.is_empty())
        {
            column_types.retain(|c| c != "aliases");
        }

        for entry in commands {
            let alias_str = if entry.aliases.is_empty() {
                String::new()
            } else {
                entry.aliases.join(&self.config.delimiter_comma)
            };

            let mut row = Vec::new();
            for col in &column_types {
                let cell: Box<dyn rich_rs::Renderable + Send + Sync> = match col.as_str() {
                    "name" => Box::new(Text::styled(&entry.name, self.config.style_command)),
                    "aliases" => {
                        Box::new(Text::styled(&alias_str, self.config.style_command_aliases))
                    }
                    "name_with_aliases" => {
                        if alias_str.is_empty() {
                            Box::new(Text::styled(&entry.name, self.config.style_command))
                        } else {
                            let mut t = Text::styled(&entry.name, self.config.style_command);
                            t.append(
                                &self.config.delimiter_slash,
                                Some(self.config.style_option_help),
                            );
                            t.append(&alias_str, Some(self.config.style_command_aliases));
                            Box::new(t)
                        }
                    }
                    "help" => Box::new(Text::styled(&entry.help, self.config.style_command_help)),
                    _ => Box::new(Text::plain("")),
                };
                row.push(cell);
            }
            rows.push(row);
        }

        rows
    }

    fn build_option_column(
        &self,
        opt: &click::option::ClickOption,
        column: &str,
    ) -> Option<Box<dyn rich_rs::Renderable + Send + Sync>> {
        match column {
            "required" => {
                if opt.required() {
                    Some(Box::new(Text::styled(
                        &self.config.required_short_string,
                        self.config.style_required_short,
                    )))
                } else {
                    None
                }
            }
            "opt_long" => self.build_option_long(opt),
            "opt_short" => {
                if opt.is_bool_flag && opt.short.len() == 2 {
                    let mut text = Text::styled("", self.config.style_switch);
                    text.append(&opt.short[0], Some(self.config.style_switch));
                    text.append(
                        &self.config.delimiter_slash,
                        Some(self.config.style_option_help),
                    );
                    text.append(&opt.short[1], Some(self.config.style_switch));
                    Some(Box::new(text))
                } else {
                    self.build_option_list(&opt.short, self.config.style_switch)
                }
            }
            "opt_all" => Some(Box::new(self.build_option_all_text(opt)) as Box<_>),
            "opt_long_metavar" => self.build_option_with_metavar(&opt.long, opt),
            "opt_all_metavar" => {
                let mut text = self.build_option_all_text(opt);
                if let Some(mv) = self.option_metavar_display(opt) {
                    text.append(" ", Some(self.config.style_option_help));
                    text.append(mv, Some(self.config.style_metavar));
                }
                Some(Box::new(text))
            }
            "metavar" | "metavar_short" => self
                .option_metavar_display(opt)
                .map(|mv| Box::new(Text::styled(mv, self.config.style_metavar)) as Box<_>),
            "help" => {
                if self.config.text_markup == TextMarkup::Markdown {
                    let markdown = self.build_option_help_markdown(opt);
                    Some(Box::new(Markdown::new(markdown).with_hyperlinks(true)) as Box<_>)
                } else {
                    Some(Box::new(self.build_option_help_text(opt)) as Box<_>)
                }
            }
            _ => None,
        }
    }

    fn build_option_list(
        &self,
        items: &[String],
        style: Style,
    ) -> Option<Box<dyn rich_rs::Renderable + Send + Sync>> {
        if items.is_empty() {
            return None;
        }
        let mut text = Text::styled("", style);
        for (idx, item) in items.iter().enumerate() {
            if idx > 0 {
                text.append(
                    &self.config.delimiter_comma,
                    Some(self.config.style_option_help),
                );
            }
            text.append(item, Some(style));
        }
        Some(Box::new(text))
    }

    fn build_option_long(
        &self,
        opt: &click::option::ClickOption,
    ) -> Option<Box<dyn rich_rs::Renderable + Send + Sync>> {
        if opt.long.is_empty() {
            return None;
        }
        if opt.is_bool_flag && opt.long.len() == 2 {
            let mut text = Text::styled("", self.config.style_option);
            text.append(&opt.long[0], Some(self.config.style_option));
            text.append(
                &self.config.delimiter_slash,
                Some(self.config.style_option_help),
            );
            text.append(&opt.long[1], Some(self.config.style_option));
            return Some(Box::new(text));
        }
        self.build_option_list(&opt.long, self.config.style_option)
    }

    fn prompt_label_for_option(&self, opt: &click::option::ClickOption) -> Option<String> {
        if !self.config.show_prompt || opt.prompt.is_none() {
            return None;
        }
        let base = if opt.confirmation_prompt {
            if opt.hide_input {
                &self.config.prompt_confirm_hidden_string
            } else {
                &self.config.prompt_confirm_string
            }
        } else if opt.hide_input {
            &self.config.prompt_hidden_string
        } else {
            &self.config.prompt_string
        };
        let mut label = base.clone();
        if let Some(prompt) = opt.prompt.as_deref() {
            if label.contains("{}") {
                label = label.replace("{}", prompt);
            }
        }
        Some(label)
    }

    fn build_option_all_text(&self, opt: &click::option::ClickOption) -> Text {
        let mut text = Text::styled("", self.config.style_option);
        let mut first = true;
        for item in &opt.short {
            if !first {
                text.append(
                    &self.config.delimiter_comma,
                    Some(self.config.style_option_help),
                );
            }
            text.append(item, Some(self.config.style_switch));
            first = false;
        }
        for item in &opt.long {
            if !first {
                text.append(
                    &self.config.delimiter_comma,
                    Some(self.config.style_option_help),
                );
            }
            text.append(item, Some(self.config.style_option));
            first = false;
        }
        text
    }

    fn build_option_with_metavar(
        &self,
        items: &[String],
        opt: &click::option::ClickOption,
    ) -> Option<Box<dyn rich_rs::Renderable + Send + Sync>> {
        if items.is_empty() {
            return None;
        }
        let mut text = Text::styled("", self.config.style_option);
        for (idx, item) in items.iter().enumerate() {
            if idx > 0 {
                text.append(
                    &self.config.delimiter_comma,
                    Some(self.config.style_option_help),
                );
            }
            text.append(item, Some(self.config.style_option));
        }
        if let Some(mv) = self.option_metavar_display(opt) {
            text.append(" ", Some(self.config.style_option_help));
            text.append(mv, Some(self.config.style_metavar));
        }
        Some(Box::new(text))
    }

    fn build_option_help_text(&self, opt: &click::option::ClickOption) -> Text {
        let mut text = Text::styled("", self.config.style_option_help);
        let mut first = true;
        let separator = if self.config.text_markup == TextMarkup::Markdown {
            "\n"
        } else {
            " "
        };
        for section in &self.config.options_table_help_sections {
            let piece = match section.as_str() {
                "help" => opt.help().map(|v| self.normalize_help_text(v)),
                "envvar" => {
                    if opt.show_envvar {
                        opt.envvar().map(|vars| {
                            self.config
                                .envvar_string
                                .replace("{}", &vars.join(&self.config.delimiter_comma))
                        })
                    } else {
                        None
                    }
                }
                "default" => {
                    if opt.show_default {
                        let default_value = if opt.is_bool_flag {
                            match opt.default.as_deref() {
                                Some("false") => {
                                    if let Some(no_opt) =
                                        opt.long.iter().find(|l| l.starts_with("--no-"))
                                    {
                                        Some(no_opt.trim_start_matches('-').to_string())
                                    } else if opt.long.len() > 1 {
                                        Some(opt.long[1].trim_start_matches('-').to_string())
                                    } else {
                                        opt.default.clone()
                                    }
                                }
                                Some("true") => {
                                    if let Some(yes_opt) =
                                        opt.long.iter().find(|l| !l.starts_with("--no-"))
                                    {
                                        Some(yes_opt.trim_start_matches('-').to_string())
                                    } else {
                                        opt.default.clone()
                                    }
                                }
                                _ => opt.default.clone(),
                            }
                        } else {
                            opt.default.clone()
                        };
                        default_value.map(|v| self.config.default_string.replace("{}", &v))
                    } else {
                        None
                    }
                }
                "required" => {
                    if opt.required() {
                        Some(self.config.required_long_string.clone())
                    } else {
                        None
                    }
                }
                "metavar" => self
                    .option_metavar(opt)
                    .map(|mv| self.config.append_metavars_help_string.replace("{}", &mv)),
                "range" => self
                    .option_metavar(opt)
                    .map(|mv| self.config.append_range_help_string.replace("{}", &mv)),
                "prompt" => self.prompt_label_for_option(opt),
                "deprecated" => None,
                _ => None,
            };
            if let Some(piece) = piece {
                if !first {
                    text.append(separator, Some(self.config.style_option_help));
                }
                if section == "help" && self.config.text_markup == TextMarkup::Rich {
                    let emojis = self.config.text_emojis.unwrap_or(true);
                    let mut rendered =
                        Text::from_markup(&piece, emojis).unwrap_or_else(|_| Text::plain(&piece));
                    rendered.stylize(
                        0,
                        rendered.plain_text().len(),
                        self.config.style_option_help,
                    );
                    text.append_text(&rendered);
                } else if section == "help" && self.config.text_markup == TextMarkup::Ansi {
                    let rendered = Text::from_ansi(&piece);
                    text.append_text(&rendered);
                } else {
                    let style = match section.as_str() {
                        "envvar" => self.config.style_option_envvar,
                        "default" => self.config.style_option_default,
                        "required" => self.config.style_required_long,
                        "metavar" | "range" => self.config.style_metavar_append,
                        _ => self.config.style_option_help,
                    };
                    text.append(piece, Some(style));
                }
                first = false;
            }
            if section == "help" && self.config.append_metavars_help == Some(true) {
                if let Some(mv) = self.option_metavar(opt) {
                    let value = self.config.append_metavars_help_string.replace("{}", &mv);
                    if !first {
                        text.append(separator, Some(self.config.style_option_help));
                    }
                    text.append(value, Some(self.config.style_metavar_append));
                    first = false;
                }
            }
        }
        if self.config.show_prompt
            && !self
                .config
                .options_table_help_sections
                .iter()
                .any(|s| s == "prompt")
        {
            if let Some(piece) = self.prompt_label_for_option(opt) {
                if !first {
                    text.append(separator, Some(self.config.style_option_help));
                }
                text.append(piece, Some(self.config.style_option_help));
            }
        }
        text
    }

    fn build_option_help_markdown(&self, opt: &click::option::ClickOption) -> String {
        let mut parts: Vec<String> = Vec::new();
        for section in &self.config.options_table_help_sections {
            let piece = match section.as_str() {
                "help" => opt.help().map(|v| self.normalize_help_text(v)),
                "envvar" => {
                    if opt.show_envvar {
                        opt.envvar().map(|vars| {
                            self.config
                                .envvar_string
                                .replace("{}", &vars.join(&self.config.delimiter_comma))
                        })
                    } else {
                        None
                    }
                }
                "default" => {
                    if opt.show_default {
                        let default_value = if opt.is_bool_flag {
                            match opt.default.as_deref() {
                                Some("false") => {
                                    if let Some(no_opt) =
                                        opt.long.iter().find(|l| l.starts_with("--no-"))
                                    {
                                        Some(no_opt.trim_start_matches('-').to_string())
                                    } else if opt.long.len() > 1 {
                                        Some(opt.long[1].trim_start_matches('-').to_string())
                                    } else {
                                        opt.default.clone()
                                    }
                                }
                                Some("true") => {
                                    if let Some(yes_opt) =
                                        opt.long.iter().find(|l| !l.starts_with("--no-"))
                                    {
                                        Some(yes_opt.trim_start_matches('-').to_string())
                                    } else {
                                        opt.default.clone()
                                    }
                                }
                                _ => opt.default.clone(),
                            }
                        } else {
                            opt.default.clone()
                        };
                        default_value.map(|v| self.config.default_string.replace("{}", &v))
                    } else {
                        None
                    }
                }
                "required" => {
                    if opt.required() {
                        Some(self.config.required_long_string.clone())
                    } else {
                        None
                    }
                }
                "metavar" => self
                    .option_metavar(opt)
                    .map(|mv| self.config.append_metavars_help_string.replace("{}", &mv)),
                "range" => self
                    .option_metavar(opt)
                    .map(|mv| self.config.append_range_help_string.replace("{}", &mv)),
                "prompt" => self.prompt_label_for_option(opt),
                "deprecated" => None,
                _ => None,
            };
            if let Some(piece) = piece {
                parts.push(piece);
            }
            if section == "help" && self.config.append_metavars_help == Some(true) {
                if let Some(mv) = self.option_metavar(opt) {
                    let value = self.config.append_metavars_help_string.replace("{}", &mv);
                    parts.push(value);
                }
            }
        }
        if self.config.show_prompt
            && !self
                .config
                .options_table_help_sections
                .iter()
                .any(|s| s == "prompt")
        {
            if let Some(piece) = self.prompt_label_for_option(opt) {
                parts.push(piece);
            }
        }
        parts.join("  \n")
    }

    fn option_metavar(&self, opt: &click::option::ClickOption) -> Option<String> {
        if opt.is_flag || opt.count {
            return None;
        }
        opt.get_metavar()
    }

    fn option_metavar_display(&self, opt: &click::option::ClickOption) -> Option<String> {
        let mv = self.option_metavar(opt)?;
        if mv.contains('|') && !mv.starts_with('[') && !mv.ends_with(']') {
            return Some(format!("[{}]", mv));
        }
        Some(mv)
    }

    fn should_flex_last_options(&self, options: &[click::option::ClickOption]) -> bool {
        let mut has_long_metavar = false;
        for opt in options {
            if let Some(mv) = self.option_metavar(opt) {
                if mv.len() > 30 && !mv.contains(' ') && !mv.contains('\n') {
                    has_long_metavar = true;
                    break;
                }
            }
        }
        !has_long_metavar
    }

    fn is_slim_theme(&self) -> bool {
        if let Some(theme) = &self.config.theme {
            let theme = theme.trim();
            if theme.eq_ignore_ascii_case("slim") || theme.to_ascii_lowercase().ends_with("-slim") {
                return true;
            }
        }
        false
    }

    fn render_slim_sections<W: io::Write>(
        &self,
        console: &mut Console<W>,
        arguments: &[(String, String)],
        options: &[click::option::ClickOption],
        commands: &[CommandEntry],
    ) -> io::Result<()> {
        let mut printed_any = false;
        if !arguments.is_empty() {
            self.render_slim_list(console, "Arguments:", arguments)?;
            printed_any = true;
        }
        if !options.is_empty() {
            if printed_any {
                self.print_slim_blank(console)?;
            }
            let rows = options
                .iter()
                .map(|opt| {
                    let mut left = self.slim_option_name(opt);
                    if let Some(mv) = self.option_metavar(opt) {
                        left.push(' ');
                        left.push('<');
                        left.push_str(&mv);
                        left.push('>');
                    }
                    let help = self.build_option_help_text(opt).plain_text().to_string();
                    (left, help)
                })
                .collect::<Vec<_>>();
            self.render_slim_list(console, "Options:", &rows)?;
            printed_any = true;
        }
        if !commands.is_empty() {
            if printed_any {
                self.print_slim_blank(console)?;
            }
            let rows = commands
                .iter()
                .map(|cmd| (cmd.name.clone(), cmd.help.clone()))
                .collect::<Vec<_>>();
            self.render_slim_list(console, "Commands:", &rows)?;
            printed_any = true;
        }
        if printed_any {
            self.print_slim_blank(console)?;
        }
        Ok(())
    }

    fn render_slim_list<W: io::Write>(
        &self,
        console: &mut Console<W>,
        title: &str,
        rows: &[(String, String)],
    ) -> io::Result<()> {
        let width = self.line_width();
        console.print_text(&self.pad_line(title, width))?;
        let max_left = rows
            .iter()
            .map(|(left, _)| left.chars().count())
            .max()
            .unwrap_or(0);
        for (left, right) in rows {
            if right.is_empty() {
                let line = format!("  {}", left);
                console.print_text(&self.pad_line(&line, width))?;
            } else {
                let pad = if max_left > left.chars().count() {
                    max_left - left.chars().count()
                } else {
                    0
                };
                let line = format!("  {}{}  {}", left, " ".repeat(pad), right);
                console.print_text(&self.pad_line(&line, width))?;
            }
        }
        Ok(())
    }

    fn line_width(&self) -> usize {
        self.config.width.or(self.config.max_width).unwrap_or(80)
    }

    fn pad_line(&self, line: &str, width: usize) -> String {
        let len = line.chars().count();
        if len >= width {
            line.to_string()
        } else {
            format!("{}{}", line, " ".repeat(width - len))
        }
    }

    fn print_slim_blank<W: io::Write>(&self, console: &mut Console<W>) -> io::Result<()> {
        let width = self.line_width();
        console.print_text(&" ".repeat(width))
    }

    fn slim_option_name(&self, opt: &click::option::ClickOption) -> String {
        let mut names = Vec::new();
        for s in &opt.short {
            names.push(s.as_str());
        }
        for l in &opt.long {
            names.push(l.as_str());
        }
        if opt.is_bool_flag && names.len() == 2 && opt.short.is_empty() {
            return format!("{}{}{}", names[0], self.config.delimiter_slash, names[1]);
        }
        names.join(&format!("{} ", self.config.delimiter_comma))
    }

    fn partition_options(
        &self,
        options: &[click::option::ClickOption],
        names: &[String],
    ) -> (
        Vec<click::option::ClickOption>,
        Vec<click::option::ClickOption>,
    ) {
        let mut selected = Vec::new();
        let mut remaining: Vec<click::option::ClickOption> = options.to_vec();

        for name in names {
            if let Some(pos) = remaining.iter().position(|opt| {
                opt.name() == name
                    || opt.long.iter().any(|l| l == name)
                    || opt.short.iter().any(|s| s == name)
                    || opt.long.iter().any(|l| l.trim_start_matches('-') == name)
            }) {
                selected.push(remaining.remove(pos));
            }
        }

        (selected, remaining)
    }

    fn partition_commands(
        &self,
        commands: &[CommandEntry],
        names: &[String],
    ) -> (Vec<CommandEntry>, Vec<CommandEntry>) {
        let mut selected = Vec::new();
        let mut remaining: Vec<CommandEntry> = commands.to_vec();

        for name in names {
            if let Some(pos) = remaining.iter().position(|entry| &entry.name == name) {
                selected.push(remaining.remove(pos));
            }
        }

        (selected, remaining)
    }
}

/// Extension trait for getting rich help output.
pub trait RichHelp {
    fn get_rich_help(&self, ctx: &Context) -> String;
    fn get_rich_help_with(&self, ctx: &Context, config: &RichHelpConfig) -> String;
}

impl RichHelp for Command {
    fn get_rich_help(&self, ctx: &Context) -> String {
        RichHelpRenderer::new(RichHelpConfig::default()).render_command_help(self, ctx)
    }

    fn get_rich_help_with(&self, ctx: &Context, config: &RichHelpConfig) -> String {
        RichHelpRenderer::new(config.clone()).render_command_help(self, ctx)
    }
}

impl RichHelp for Group {
    fn get_rich_help(&self, ctx: &Context) -> String {
        RichHelpRenderer::new(RichHelpConfig::default()).render_group_help(self, ctx)
    }

    fn get_rich_help_with(&self, ctx: &Context, config: &RichHelpConfig) -> String {
        RichHelpRenderer::new(config.clone()).render_group_help(self, ctx)
    }
}

/// Run a command with rich help output when --help is requested.
pub fn main_rich_command(
    command: &Command,
    args: Vec<String>,
    config: &RichHelpConfig,
) -> Result<(), ClickError> {
    let prog_name = CommandLike::name(command)
        .map(|s| s.to_string())
        .unwrap_or_else(|| {
            std::env::args()
                .next()
                .unwrap_or_else(|| "program".to_string())
        });

    let args_for_eager = args.clone();
    let ctx_result = command.make_context(&prog_name, args, None);

    match ctx_result {
        Ok(ctx) => {
            let ctx = Arc::new(ctx);
            push_context(Arc::clone(&ctx));
            let result = command.invoke(&ctx);
            pop_context();
            ctx.close();
            result
        }
        Err(ClickError::Exit { code: 0 }) => {
            if let Some(version_output) = get_version_output_from_args(command, &args_for_eager) {
                println!("{}", version_output);
                return Ok(());
            }
            let ctx = ContextBuilder::new().info_name(&prog_name).build();
            let help = RichHelpRenderer::new(config.clone()).render_command_help(command, &ctx);
            print!("{}", help);
            Ok(())
        }
        Err(e) => Err(e),
    }
}

/// Run a command with rich help output and rich error rendering.
pub fn main_rich_command_with_errors(
    command: &Command,
    args: Vec<String>,
    config: &RichHelpConfig,
) -> Result<(), ClickError> {
    let prog_name = CommandLike::name(command)
        .map(|s| s.to_string())
        .unwrap_or_else(|| {
            std::env::args()
                .next()
                .unwrap_or_else(|| "program".to_string())
        });

    let args_for_eager = args.clone();
    let ctx_result = command.make_context(&prog_name, args, None);

    match ctx_result {
        Ok(ctx) => {
            let ctx = Arc::new(ctx);
            push_context(Arc::clone(&ctx));
            let result = command.invoke(&ctx);
            if let Err(ref err) = result {
                let renderer = RichHelpRenderer::new(config.clone());
                eprint!("{}", renderer.render_error(err));
            }
            pop_context();
            ctx.close();
            result
        }
        Err(ClickError::Exit { code: 0 }) => {
            if let Some(version_output) = get_version_output_from_args(command, &args_for_eager) {
                println!("{}", version_output);
                return Ok(());
            }
            let ctx = ContextBuilder::new().info_name(&prog_name).build();
            let help = RichHelpRenderer::new(config.clone()).render_command_help(command, &ctx);
            print!("{}", help);
            Ok(())
        }
        Err(err) => {
            let renderer = RichHelpRenderer::new(config.clone());
            eprint!("{}", renderer.render_error(&err));
            Err(err)
        }
    }
}

/// Build a `HelpRenderer` that dispatches to the rich renderer by downcasting
/// the `&dyn CommandLike` to `&Group` or `&Command`.
///
/// This is installed into the root context by [`make_rich_group_context`] so that
/// `Group::invoke` will call it for any subcommand's `--help` (i.e. `Exit{0}`
/// from `make_context`), producing rich-formatted output instead of the plain
/// `cmd.get_help()` fallback.
fn build_rich_help_renderer(config: RichHelpConfig) -> HelpRenderer {
    Arc::new(move |cmd: &dyn CommandLike, ctx: &Context| {
        let rich = RichHelpRenderer::new(config.clone());
        if let Some(group) = cmd.as_any().downcast_ref::<Group>() {
            rich.render_group_help(group, ctx)
        } else if let Some(command) = cmd.as_any().downcast_ref::<Command>() {
            rich.render_command_help(command, ctx)
        } else {
            // Fallback: use the CommandLike's own plain help
            cmd.get_help(ctx)
        }
    })
}

/// Build the root context for a `Group`, mirroring `Group::make_context` but
/// also installing the rich help renderer so subcommand `--help` requests are
/// rendered richly.
///
/// Returns `Err(Exit{0})` if the group's own `--help` is triggered (caller
/// handles that by printing rich group help directly).
fn make_rich_group_context(
    group: &Group,
    prog_name: &str,
    args: Vec<String>,
    renderer: HelpRenderer,
) -> Result<Context, ClickError> {
    let builder = ContextBuilder::new()
        .info_name(prog_name)
        .allow_extra_args(true)
        .allow_interspersed_args(false)
        .help_renderer(renderer);

    // Groups do not have a parent at root level.
    let mut ctx = builder.build();
    group.command.parse_args(&mut ctx, args)?;
    Ok(ctx)
}

/// Run a group with rich help output when --help is requested.
pub fn main_rich_group(
    group: &Group,
    args: Vec<String>,
    config: &RichHelpConfig,
) -> Result<(), ClickError> {
    let prog_name = CommandLike::name(group)
        .map(|s| s.to_string())
        .unwrap_or_else(|| {
            std::env::args()
                .next()
                .unwrap_or_else(|| "program".to_string())
        });

    let args_for_eager = args.clone();
    let renderer = build_rich_help_renderer(config.clone());
    let ctx_result = make_rich_group_context(group, &prog_name, args, renderer);

    match ctx_result {
        Ok(ctx) => {
            let ctx = Arc::new(ctx);
            push_context(Arc::clone(&ctx));
            let result = group.invoke(&ctx);
            pop_context();
            ctx.close();
            result
        }
        Err(ClickError::Exit { code: 0 }) => {
            if let Some(version_output) =
                get_version_output_from_args(&group.command, &args_for_eager)
            {
                println!("{}", version_output);
                return Ok(());
            }
            let ctx = ContextBuilder::new().info_name(&prog_name).build();
            let help = RichHelpRenderer::new(config.clone()).render_group_help(group, &ctx);
            print!("{}", help);
            Ok(())
        }
        Err(e) => Err(e),
    }
}

/// Run a group with rich help output and rich error rendering.
pub fn main_rich_group_with_errors(
    group: &Group,
    args: Vec<String>,
    config: &RichHelpConfig,
) -> Result<(), ClickError> {
    let prog_name = CommandLike::name(group)
        .map(|s| s.to_string())
        .unwrap_or_else(|| {
            std::env::args()
                .next()
                .unwrap_or_else(|| "program".to_string())
        });

    let args_for_eager = args.clone();
    let renderer = build_rich_help_renderer(config.clone());
    let ctx_result = make_rich_group_context(group, &prog_name, args, renderer);

    match ctx_result {
        Ok(ctx) => {
            let ctx = Arc::new(ctx);
            push_context(Arc::clone(&ctx));
            let result = group.invoke(&ctx);
            if let Err(ref err) = result {
                let renderer = RichHelpRenderer::new(config.clone());
                eprint!("{}", renderer.render_error(err));
            }
            pop_context();
            ctx.close();
            result
        }
        Err(ClickError::Exit { code: 0 }) => {
            if let Some(version_output) =
                get_version_output_from_args(&group.command, &args_for_eager)
            {
                println!("{}", version_output);
                return Ok(());
            }
            let ctx = ContextBuilder::new().info_name(&prog_name).build();
            let help = RichHelpRenderer::new(config.clone()).render_group_help(group, &ctx);
            print!("{}", help);
            Ok(())
        }
        Err(err) => {
            let renderer = RichHelpRenderer::new(config.clone());
            eprint!("{}", renderer.render_error(&err));
            Err(err)
        }
    }
}

fn get_version_output_from_args(command: &Command, args: &[String]) -> Option<String> {
    for opt in &command.options {
        let meta = opt.config.metavar.as_deref()?;
        let output = meta.strip_prefix(VERSION_METAVAR_PREFIX)?;

        let mut names = opt.long.iter().chain(opt.short.iter());
        if names.any(|n| args.iter().any(|a| arg_matches_opt(a, n))) {
            return Some(output.to_string());
        }
    }
    None
}

fn arg_matches_opt(arg: &str, opt: &str) -> bool {
    if arg == opt {
        return true;
    }
    if opt.starts_with("--")
        && arg.starts_with(opt)
        && arg.get(opt.len()..opt.len() + 1) == Some("=")
    {
        return true;
    }
    if opt.starts_with('-') && opt.len() == 2 && !opt.starts_with("--") {
        let needle = opt.chars().nth(1).unwrap_or('\0');
        if arg.starts_with('-') && !arg.starts_with("--") {
            return arg.chars().skip(1).any(|c| c == needle);
        }
    }
    false
}

fn append_suffix_to_first_line(message: &str, suffix: &str) -> String {
    if let Some(idx) = message.find('\n') {
        let (first, rest) = message.split_at(idx);
        format!("{}{}{}", first, suffix, rest)
    } else {
        format!("{}{}", message, suffix)
    }
}

fn collect_param_candidates(value: &str) -> Vec<String> {
    let mut long = Vec::new();
    let mut short = Vec::new();
    for raw in value.split(|c| c == ',' || c == '/') {
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            continue;
        }
        if trimmed.starts_with("--") {
            long.push(trimmed.to_string());
        } else {
            short.push(trimmed.to_string());
        }
    }
    long.extend(short);
    long
}

fn normalize_param_name(value: &str) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return None;
    }
    if trimmed.starts_with("--no-") {
        return Some(trimmed.trim_start_matches("--no-").replace('-', "_"));
    }
    if trimmed.starts_with("--") {
        return Some(trimmed.trim_start_matches("--").replace('-', "_"));
    }
    if trimmed.starts_with('-') {
        let name = trimmed.trim_start_matches('-');
        if name.is_empty() {
            return None;
        }
        return Some(name.to_string());
    }
    Some(trimmed.replace('-', "_").to_ascii_lowercase())
}
