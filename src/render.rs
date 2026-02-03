use std::io;
use std::sync::Arc;

use click::command::Command;
use click::context::{pop_context, push_context, Context, ContextBuilder};
use click::error::ClickError;
use click::group::{CommandLike, Group};
use click::parameter::Parameter;
use rich_rs::markdown::Markdown;
use rich_rs::{Column, Console, ConsoleOptions, Panel, Row, Style, Table, Text};

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
        console.print(&usage_text, None, None, None, false, "\n")?;

        self.render_help_text(console, command.help.as_deref(), command.deprecated.as_deref())?;

        let mut sections_printed = false;

        if self.config.show_arguments.unwrap_or(true) {
            let arg_records = command
                .arguments
                .iter()
                .filter_map(|arg| arg.get_help_record())
                .filter(|(_, help)| !help.is_empty())
                .collect::<Vec<_>>();
            if !arg_records.is_empty() {
                self.print_section_spacing(console, &mut sections_printed)?;
                self.print_table_panel(
                    console,
                    &self.config.arguments_panel_title,
                    &arg_records,
                    self.config.style_argument,
                    self.config.style_helptext,
                    &self.config.panel_arguments,
                    &self.config.table_arguments,
                )?;
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
                console.print(&Text::styled(epilog, self.config.style_helptext), None, None, None, false, "\n")?;
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
        console.print(&usage_text, None, None, None, false, "\n")?;

        self.render_help_text(console, group.command.help.as_deref(), group.command.deprecated.as_deref())?;

        let mut sections_printed = false;

        let commands = self.collect_commands(group);

        let options = self.collect_options(&group.command, ctx);

        let mut arguments = Vec::new();
        if self.config.show_arguments.unwrap_or(true) {
            arguments = group
                .command
                .arguments
                .iter()
                .filter_map(|arg| arg.get_help_record())
                .filter(|(_, help)| !help.is_empty())
                .collect::<Vec<_>>();
        }

        if self.config.commands_before_options {
            if !commands.is_empty() {
            self.print_section_spacing(console, &mut sections_printed)?;
            self.print_command_panels(console, &commands)?;
            }
        }

        if !arguments.is_empty() {
            self.print_section_spacing(console, &mut sections_printed)?;
            self.print_table_panel(
                console,
                &self.config.arguments_panel_title,
                &arguments,
                self.config.style_argument,
                self.config.style_helptext,
                &self.config.panel_arguments,
                &self.config.table_arguments,
            )?;
        }

        if !options.is_empty() {
            self.print_section_spacing(console, &mut sections_printed)?;
            self.print_option_panels(console, &options)?;
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
                console.print(&Text::styled(epilog, self.config.style_helptext), None, None, None, false, "\n")?;
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
                console.print_text("")?;
                let mut decorated = self.apply_paragraph_linebreaks(help_text);
                if let Some(dep) = deprecated {
                    if dep.is_empty() {
                        decorated = format!("{} {}", decorated, self.config.deprecated_string);
                    } else {
                        decorated = format!(
                            "{} {}",
                            decorated,
                            self.config
                                .deprecated_with_reason_string
                                .replace("{}", dep)
                        );
                    }
                }
                if self.config.text_markup == TextMarkup::Markdown {
                    let md = Markdown::new(&decorated);
                    console.print(&md, None, None, None, false, "\n")?;
                } else {
                    let help_text = self.render_help_text_block(&decorated, deprecated.is_some());
                    console.print(&help_text, None, None, None, false, "\n")?;
                }
                rendered = true;
            }
        }
        if !rendered {
            if let Some(dep) = deprecated {
                console.print_text("")?;
                let dep_msg = if dep.is_empty() {
                    self.config.deprecated_string.clone()
                } else {
                    self.config
                        .deprecated_with_reason_string
                        .replace("{}", dep)
                };
                let dep_text = Text::styled(&dep_msg, self.config.style_deprecated);
                console.print(&dep_text, None, None, None, false, "\n")?;
            }
        }
        Ok(())
    }

    fn render_error_into<W: io::Write>(
        &self,
        console: &mut Console<W>,
        error: &ClickError,
    ) -> io::Result<()> {
        let message = if matches!(error, ClickError::Abort) {
            self.config.aborted_text.clone()
        } else {
            error.format_full()
        };
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
        console.print(&panel, None, None, None, false, "\n")?;

        if let Some(ref suggestion) = self.config.errors_suggestion {
            let text = Text::styled(suggestion, self.config.style_errors_suggestion.unwrap_or(self.config.style_option_help));
            console.print(&text, None, None, None, false, "\n")?;
        }
        if let Some(ref epilogue) = self.config.errors_epilogue {
            let text = Text::styled(epilogue, self.config.style_padding_errors);
            console.print(&text, None, None, None, false, "\n")?;
        }
        Ok(())
    }

    fn print_section_spacing<W: io::Write>(
        &self,
        console: &mut Console<W>,
        printed_any: &mut bool,
    ) -> io::Result<()> {
        if *printed_any {
            console.print_text("")?;
        } else {
            console.print_text("")?;
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

        if !self.config.option_groups.is_empty() {
            for group in &self.config.option_groups {
                let (group_items, rest) = self.partition_options(&remaining, &group.items);
                remaining = rest;
                if group_items.is_empty() {
                    continue;
                }
                let rows = self.build_option_rows(&group_items);
                let table = self.build_table_from_rows(rows, &self.config.table_options, None);
                let title_style = group.title_style.unwrap_or(self.config.panel_options.title_style);
                let help_style = group.help_style.unwrap_or(self.config.style_options_panel_help_style);
                let inline = group.inline_help_in_title.unwrap_or(self.config.panel_inline_help_in_title);
                let panel = self.build_panel(
                    &group.name,
                    table,
                    &self.config.panel_options,
                    group.help.as_deref(),
                    help_style,
                    Some(title_style),
                    inline,
                );
                panels.push(panel);
            }
        }

        if !remaining.is_empty() {
            let rows = self.build_option_rows(&remaining);
            let table = self.build_table_from_rows(rows, &self.config.table_options, None);
            let panel = self.build_panel(
                &self.config.options_panel_title,
                table,
                &self.config.panel_options,
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

        for panel in panels {
            console.print(&panel, None, None, None, false, "\n")?;
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

        if !self.config.command_groups.is_empty() {
            for group in &self.config.command_groups {
                let (group_items, rest) = self.partition_commands(&remaining, &group.items);
                remaining = rest;
                if group_items.is_empty() {
                    continue;
                }
                let rows = self.build_command_rows(&group_items);
                let table = self.build_table_from_rows(
                    rows,
                    &self.config.table_commands,
                    self.config.style_commands_table_column_width_ratio,
                );
                let title_style = group.title_style.unwrap_or(self.config.panel_commands.title_style);
                let help_style = group.help_style.unwrap_or(self.config.style_commands_panel_help_style);
                let inline = group.inline_help_in_title.unwrap_or(self.config.panel_inline_help_in_title);
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

        if !remaining.is_empty() {
            let rows = self.build_command_rows(&remaining);
            let table = self.build_table_from_rows(
                rows,
                &self.config.table_commands,
                self.config.style_commands_table_column_width_ratio,
            );
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

        for panel in panels {
            console.print(&panel, None, None, None, false, "\n")?;
        }
        Ok(())
    }

    fn print_table_panel<W: io::Write>(
        &self,
        console: &mut Console<W>,
        title: &str,
        records: &[(String, String)],
        key_style: Style,
        value_style: Style,
        panel_cfg: &PanelConfig,
        table_cfg: &TableConfig,
    ) -> io::Result<()> {
        let rows = records
            .iter()
            .map(|(left, right)| {
                vec![
                    Box::new(Text::styled(left, key_style)) as Box<dyn rich_rs::Renderable + Send + Sync>,
                    Box::new(Text::styled(right, value_style)) as Box<dyn rich_rs::Renderable + Send + Sync>,
                ]
            })
            .collect::<Vec<_>>();
        let table = self.build_table_from_rows(rows, table_cfg, None);
        let panel = self.build_panel(
            title,
            table,
            panel_cfg,
            None,
            self.config.style_options_panel_help_style,
            None,
            self.config.panel_inline_help_in_title,
        );
        console.print(&panel, None, None, None, false, "\n")
    }

    fn build_table_from_rows(
        &self,
        rows: Vec<Vec<Box<dyn rich_rs::Renderable + Send + Sync>>>,
        table_cfg: &TableConfig,
        ratios: Option<(Option<usize>, Option<usize>)>,
    ) -> Table {
        let mut table = Table::grid()
            .with_padding(table_cfg.padding.0, table_cfg.padding.1)
            .with_pad_edge(table_cfg.pad_edge)
            .with_show_lines(table_cfg.show_lines)
            .with_leading(table_cfg.leading)
            .with_expand(table_cfg.expand)
            .with_border_style(table_cfg.border_style);

        if let Some(box_type) = table_cfg.box_type {
            table = table.with_box(Some(box_type));
        }

        if !table_cfg.row_styles.is_empty() {
            table = table.with_row_styles(table_cfg.row_styles.clone());
        }

        if let Some(first_row) = rows.first() {
            for idx in 0..first_row.len() {
                let mut col = Column::default();
                if let Some((left, right)) = ratios {
                    if idx == 0 {
                        col.ratio = left;
                    } else if idx == 1 {
                        col.ratio = right;
                    }
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
        let title_text = self.build_panel_title(title, title_style, help_text, help_style, inline_help);
        Panel::new(Box::new(table))
            .with_box(panel_cfg.box_type)
            .with_title_text(title_text)
            .with_title_align(panel_cfg.align)
            .with_border_style(panel_cfg.border_style)
            .with_style(panel_cfg.panel_style)
            .with_padding(panel_cfg.padding)
            .with_expand(panel_cfg.expand)
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
        options.color_system = self.config.color_system.to_color_system(options.color_system);
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
            TextMarkup::Ansi => Text::from_ansi(&source),
            TextMarkup::None => {
                let mut rendered = Text::plain(&source);
                self.apply_help_styles(&mut rendered, &source, has_deprecated, true);
                rendered
            }
        }
    }

    fn apply_help_styles(&self, rendered: &mut Text, raw: &str, has_deprecated: bool, style_base: bool) {
        if style_base {
            rendered.stylize(0, raw.len(), self.config.style_helptext);
        }
        if let Some(first_end) = raw.find('\n') {
            rendered.stylize(0, first_end, self.config.style_helptext_first_line);
        } else {
            rendered.stylize(0, raw.len(), self.config.style_helptext_first_line);
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
        if let Some(ref breaks) = self.config.text_paragraph_linebreaks {
            if breaks == "\n" {
                return input.replace("\n\n", "\n");
            }
            if breaks != "\n\n" {
                return input.replace("\n\n", breaks);
            }
        }
        input.to_string()
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
                text.append(&self.config.panel_inline_help_delimiter, Some(self.config.style_option_help));
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
        if self.config.show_commands.unwrap_or(true) {
            for (registered, cmd) in group.list_command_entries() {
                if cmd.is_hidden() {
                    continue;
                }
                let mut aliases = group.list_command_aliases(&registered);
                if let Some(extra) = self.config.command_aliases.get(&registered) {
                    aliases.extend(extra.iter().cloned());
                }
                aliases.sort();
                aliases.dedup();

                let mut help = cmd.get_short_help();
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
                    name: registered,
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
        let column_types = self.config.options_table_column_types.clone();

        for opt in options {
            let mut row = Vec::new();
            for col in &column_types {
                let cell = self.build_option_column(opt, col);
                if let Some(renderable) = cell {
                    row.push(renderable);
                } else {
                    row.push(Box::new(Text::plain("")) as Box<dyn rich_rs::Renderable + Send + Sync>);
                }
            }
            rows.push(row);
        }

        rows
    }

    fn build_command_rows(
        &self,
        commands: &[CommandEntry],
    ) -> Vec<Vec<Box<dyn rich_rs::Renderable + Send + Sync>>> {
        let mut rows = Vec::new();
        let column_types = self.config.commands_table_column_types.clone();

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
                    "aliases" => Box::new(Text::styled(&alias_str, self.config.style_command_aliases)),
                    "name_with_aliases" => {
                        if alias_str.is_empty() {
                            Box::new(Text::styled(&entry.name, self.config.style_command))
                        } else {
                            let mut t = Text::styled(&entry.name, self.config.style_command);
                            t.append(&self.config.delimiter_slash, Some(self.config.style_option_help));
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
            "opt_long" => self.build_option_list(&opt.long, self.config.style_option),
            "opt_short" => self.build_option_list(&opt.short, self.config.style_switch),
            "opt_all" => {
                Some(Box::new(self.build_option_all_text(opt)) as Box<_>)
            }
            "opt_long_metavar" => self.build_option_with_metavar(&opt.long, opt),
            "opt_all_metavar" => {
                let mut text = self.build_option_all_text(opt);
                if let Some(mv) = opt.get_metavar() {
                    text.append(" ", Some(self.config.style_option_help));
                    text.append(mv, Some(self.config.style_metavar));
                }
                Some(Box::new(text))
            }
            "metavar" | "metavar_short" => {
                opt.get_metavar()
                    .map(|mv| Box::new(Text::styled(mv, self.config.style_metavar)) as Box<_>)
            }
            "help" => Some(Box::new(self.build_option_help_text(opt)) as Box<_>),
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
                text.append(&self.config.delimiter_comma, Some(self.config.style_option_help));
            }
            text.append(item, Some(style));
        }
        Some(Box::new(text))
    }

    fn build_option_all_text(&self, opt: &click::option::ClickOption) -> Text {
        let mut text = Text::styled("", self.config.style_option);
        let mut first = true;
        for item in &opt.short {
            if !first {
                text.append(&self.config.delimiter_comma, Some(self.config.style_option_help));
            }
            text.append(item, Some(self.config.style_switch));
            first = false;
        }
        for item in &opt.long {
            if !first {
                text.append(&self.config.delimiter_comma, Some(self.config.style_option_help));
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
                text.append(&self.config.delimiter_comma, Some(self.config.style_option_help));
            }
            text.append(item, Some(self.config.style_option));
        }
        if let Some(mv) = opt.get_metavar() {
            text.append(" ", Some(self.config.style_option_help));
            text.append(mv, Some(self.config.style_metavar));
        }
        Some(Box::new(text))
    }

    fn build_option_help_text(&self, opt: &click::option::ClickOption) -> Text {
        let mut text = Text::styled("", self.config.style_option_help);
        let mut first = true;
        for section in &self.config.options_table_help_sections {
            let piece = match section.as_str() {
                "help" => opt.help().map(|v| v.to_string()),
                "envvar" => {
                    if opt.show_envvar {
                        opt.envvar()
                            .map(|vars| self.config.envvar_string.replace("{}", &vars.join(&self.config.delimiter_comma)))
                    } else {
                        None
                    }
                }
                "default" => {
                    if opt.show_default {
                        opt.default
                            .as_ref()
                            .map(|v| self.config.default_string.replace("{}", v))
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
                "metavar" => opt.get_metavar().map(|mv| self.config.append_metavars_help_string.replace("{}", &mv)),
                "range" => opt.get_metavar().map(|mv| self.config.append_range_help_string.replace("{}", &mv)),
                "deprecated" => None,
                _ => None,
            };
            if let Some(piece) = piece {
                if !first {
                    text.append(" ", Some(self.config.style_option_help));
                }
                let style = match section.as_str() {
                    "envvar" => self.config.style_option_envvar,
                    "default" => self.config.style_option_default,
                    "required" => self.config.style_required_long,
                    "metavar" | "range" => self.config.style_metavar_append,
                    _ => self.config.style_option_help,
                };
                text.append(piece, Some(style));
                first = false;
            }
        }
        text
    }

    fn partition_options(
        &self,
        options: &[click::option::ClickOption],
        names: &[String],
    ) -> (Vec<click::option::ClickOption>, Vec<click::option::ClickOption>) {
        let mut selected = Vec::new();
        let mut remaining = Vec::new();
        for opt in options {
            let mut matched = false;
            for name in names {
                if opt.name() == name {
                    matched = true;
                    break;
                }
                if opt.long.iter().any(|l| l == name) || opt.short.iter().any(|s| s == name) {
                    matched = true;
                    break;
                }
                if opt.long.iter().any(|l| l.trim_start_matches('-') == name) {
                    matched = true;
                    break;
                }
            }
            if matched {
                selected.push(opt.clone());
            } else {
                remaining.push(opt.clone());
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
        let mut remaining = Vec::new();
        for entry in commands {
            if names.iter().any(|n| n == &entry.name) {
                selected.push(entry.clone());
            } else {
                remaining.push(entry.clone());
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
            println!("{}", help);
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
    match main_rich_command(command, args, config) {
        Ok(()) => Ok(()),
        Err(err) => {
            let renderer = RichHelpRenderer::new(config.clone());
            eprint!("{}", renderer.render_error(&err));
            Err(err)
        }
    }
}

/// Run a group with rich help output when --help is requested.
pub fn main_rich_group(group: &Group, args: Vec<String>, config: &RichHelpConfig) -> Result<(), ClickError> {
    let prog_name = CommandLike::name(group)
        .map(|s| s.to_string())
        .unwrap_or_else(|| {
            std::env::args()
                .next()
                .unwrap_or_else(|| "program".to_string())
        });

    let args_for_eager = args.clone();
    let ctx_result = group.make_context(&prog_name, args, None);

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
            if let Some(version_output) = get_version_output_from_args(&group.command, &args_for_eager) {
                println!("{}", version_output);
                return Ok(());
            }
            let ctx = ContextBuilder::new().info_name(&prog_name).build();
            let help = RichHelpRenderer::new(config.clone()).render_group_help(group, &ctx);
            println!("{}", help);
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
    match main_rich_group(group, args, config) {
        Ok(()) => Ok(()),
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
    if opt.starts_with("--") && arg.starts_with(opt) && arg.get(opt.len()..opt.len() + 1) == Some("=") {
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
