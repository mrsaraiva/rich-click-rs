use std::io;
use std::sync::Arc;

use click::command::Command;
use click::context::{pop_context, push_context, Context, ContextBuilder};
use click::error::ClickError;
use click::group::{CommandLike, Group};
use click::parameter::Parameter;
use rich_rs::{Column, Console, ConsoleOptions, Panel, Row, Style, Table, Text};

use crate::config::RichHelpConfig;

const VERSION_METAVAR_PREFIX: &str = "__click_version__:";

/// Rich help renderer.
#[derive(Debug, Clone)]
pub struct RichHelpRenderer {
    config: RichHelpConfig,
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

    fn render_command_help_into<W: io::Write>(
        &self,
        console: &mut Console<W>,
        command: &Command,
        ctx: &Context,
    ) -> io::Result<()> {
        let usage = command.get_usage(ctx);
        console.print_styled(&usage, self.config.style_usage)?;

        self.render_help_text(console, command.help.as_deref(), command.deprecated.as_deref())?;

        let mut sections_printed = false;

        if self.config.show_arguments {
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
                    self.config.style_help,
                )?;
            }
        }

        let mut opt_records: Vec<(String, String)> = command
            .options
            .iter()
            .filter_map(|opt| opt.get_help_record())
            .collect();
        if let Some(help_opt) = command.get_help_option(ctx) {
            if let Some(record) = help_opt.get_help_record() {
                opt_records.push(record);
            }
        }
        if !opt_records.is_empty() {
            self.print_section_spacing(console, &mut sections_printed)?;
            self.print_table_panel(
                console,
                &self.config.options_panel_title,
                &opt_records,
                self.config.style_option,
                self.config.style_option_help,
            )?;
        }

        if let Some(epilog) = command.epilog.as_deref() {
            if !epilog.is_empty() {
                console.print_text("")?;
                console.print(&Text::styled(epilog, self.config.style_help), None, None, None, false, "\n")?;
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
        console.print_styled(&usage, self.config.style_usage)?;

        self.render_help_text(console, group.command.help.as_deref(), group.command.deprecated.as_deref())?;

        let mut sections_printed = false;

        let mut commands = Vec::new();
        if self.config.show_commands {
            for name in group.list_commands() {
                if let Some(cmd) = group.get_command(name) {
                    if cmd.is_hidden() {
                        continue;
                    }
                    let help = cmd.get_short_help();
                    commands.push((name.to_string(), help));
                }
            }
        }

        let mut options: Vec<(String, String)> = group
            .command
            .options
            .iter()
            .filter_map(|opt| opt.get_help_record())
            .collect();
        if let Some(help_opt) = group.command.get_help_option(ctx) {
            if let Some(record) = help_opt.get_help_record() {
                options.push(record);
            }
        }

        let mut arguments = Vec::new();
        if self.config.show_arguments {
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
                self.print_table_panel(
                    console,
                    &self.config.commands_panel_title,
                    &commands,
                    self.config.style_command,
                    self.config.style_command_help,
                )?;
            }
        }

        if !arguments.is_empty() {
            self.print_section_spacing(console, &mut sections_printed)?;
            self.print_table_panel(
                console,
                &self.config.arguments_panel_title,
                &arguments,
                self.config.style_argument,
                self.config.style_help,
            )?;
        }

        if !options.is_empty() {
            self.print_section_spacing(console, &mut sections_printed)?;
            self.print_table_panel(
                console,
                &self.config.options_panel_title,
                &options,
                self.config.style_option,
                self.config.style_option_help,
            )?;
        }

        if !self.config.commands_before_options {
            if !commands.is_empty() {
                self.print_section_spacing(console, &mut sections_printed)?;
                self.print_table_panel(
                    console,
                    &self.config.commands_panel_title,
                    &commands,
                    self.config.style_command,
                    self.config.style_command_help,
                )?;
            }
        }

        if let Some(epilog) = group.command.epilog.as_deref() {
            if !epilog.is_empty() {
                console.print_text("")?;
                console.print(&Text::styled(epilog, self.config.style_help), None, None, None, false, "\n")?;
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
                let decorated = if let Some(dep) = deprecated {
                    if dep.is_empty() {
                        format!("{}  (DEPRECATED)", help_text)
                    } else {
                        format!("{}  (DEPRECATED: {})", help_text, dep)
                    }
                } else {
                    help_text.to_string()
                };
                console.print(&Text::styled(&decorated, self.config.style_help), None, None, None, false, "\n")?;
                rendered = true;
            }
        }
        if !rendered {
            if let Some(dep) = deprecated {
                console.print_text("")?;
                let dep_msg = if dep.is_empty() {
                    "(DEPRECATED)".to_string()
                } else {
                    format!("(DEPRECATED: {})", dep)
                };
                console.print(&Text::styled(&dep_msg, self.config.style_deprecated), None, None, None, false, "\n")?;
            }
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

    fn print_table_panel<W: io::Write>(
        &self,
        console: &mut Console<W>,
        title: &str,
        records: &[(String, String)],
        key_style: Style,
        value_style: Style,
    ) -> io::Result<()> {
        let table = self.build_table(records, key_style, value_style);
        let panel = self.build_panel(title, table);
        console.print(&panel, None, None, None, false, "\n")
    }

    fn build_table(&self, records: &[(String, String)], key_style: Style, value_style: Style) -> Table {
        let mut table = Table::grid()
            .with_padding(self.config.table.padding.0, self.config.table.padding.1)
            .with_pad_edge(self.config.table.pad_edge)
            .with_show_lines(self.config.table.show_lines)
            .with_leading(self.config.table.leading)
            .with_expand(self.config.table.expand)
            .with_border_style(self.config.table.border_style);

        if let Some(box_type) = self.config.table.box_type {
            table = table.with_box(Some(box_type));
        }

        if !self.config.table.row_styles.is_empty() {
            table = table.with_row_styles(self.config.table.row_styles.clone());
        }

        let mut key_column = Column::default();
        key_column.style = key_style;
        let mut value_column = Column::default();
        value_column.style = value_style;
        table.add_column(key_column);
        table.add_column(value_column);

        for (left, right) in records {
            let left_text = Text::styled(left, key_style);
            let right_text = if right.is_empty() {
                Text::plain("")
            } else {
                Text::styled(right, value_style)
            };
            let row = Row::new(vec![
                Box::new(left_text) as Box<dyn rich_rs::Renderable + Send + Sync>,
                Box::new(right_text) as Box<dyn rich_rs::Renderable + Send + Sync>,
            ]);
            table.add_row(row);
        }

        table
    }

    fn build_panel(&self, title: &str, table: Table) -> Panel {
        let title_text = Text::styled(title, self.config.panel.title_style);
        Panel::new(Box::new(table))
            .with_box(self.config.panel.box_type)
            .with_title_text(title_text)
            .with_title_align(self.config.panel.align)
            .with_border_style(self.config.panel.border_style)
            .with_style(self.config.panel.panel_style)
            .with_padding(self.config.panel.padding)
            .with_expand(self.config.panel.expand)
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
        if let Some(color_system) = self.config.color_system {
            options.color_system = Some(color_system);
        }
        if let Some(force_terminal) = self.config.force_terminal {
            options.is_terminal = force_terminal;
            if !force_terminal {
                options.color_system = None;
            }
        }
        Console::capture_with_options(options)
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
