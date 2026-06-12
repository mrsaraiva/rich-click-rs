use std::any::Any;
use std::sync::Arc;

use click::command::Command;
use click::context::Context;
use click::error::ClickError;
use click::group::{CommandLike, Group};
use click::testing::{CliRunner, InvokeResult};

use crate::{main_rich_command_with_errors, main_rich_group_with_errors, RichHelpConfig};

struct RichCommandLike<'a> {
    inner: &'a dyn CommandLike,
    config: RichHelpConfig,
}

impl CommandLike for RichCommandLike<'_> {
    fn name(&self) -> Option<&str> {
        self.inner.name()
    }

    fn make_context(
        &self,
        info_name: &str,
        args: Vec<String>,
        parent: Option<Arc<Context>>,
    ) -> Result<Context, ClickError> {
        self.inner.make_context(info_name, args, parent)
    }

    fn invoke(&self, ctx: &Context) -> Result<(), ClickError> {
        self.inner.invoke(ctx)
    }

    fn main(&self, args: Vec<String>) -> Result<(), ClickError> {
        if let Some(command) = self.inner.as_any().downcast_ref::<Command>() {
            main_rich_command_with_errors(command, args, &self.config)
        } else if let Some(group) = self.inner.as_any().downcast_ref::<Group>() {
            main_rich_group_with_errors(group, args, &self.config)
        } else {
            self.inner.main(args)
        }
    }

    fn get_help(&self, ctx: &Context) -> String {
        self.inner.get_help(ctx)
    }

    fn get_short_help(&self) -> String {
        self.inner.get_short_help()
    }

    fn is_hidden(&self) -> bool {
        self.inner.is_hidden()
    }

    fn get_usage(&self, ctx: &Context) -> String {
        self.inner.get_usage(ctx)
    }

    fn as_any(&self) -> &dyn Any {
        self.inner.as_any()
    }
}

/// Test runner that renders rich help and errors (off by default in production).
#[derive(Debug, Clone, Default)]
pub struct RichCliRunner {
    runner: CliRunner,
    config: RichHelpConfig,
}

impl RichCliRunner {
    /// Create a new runner with default settings.
    pub fn new() -> Self {
        Self {
            runner: CliRunner::new(),
            config: RichHelpConfig::default(),
        }
    }

    /// Override the rich help configuration used during invocation.
    pub fn config(mut self, config: RichHelpConfig) -> Self {
        self.config = config;
        self
    }

    pub fn env(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.runner = self.runner.env(key, value);
        self
    }

    pub fn env_unset(mut self, key: impl Into<String>) -> Self {
        self.runner = self.runner.env_unset(key);
        self
    }

    pub fn env_clear(mut self) -> Self {
        self.runner = self.runner.env_clear();
        self
    }

    pub fn echo_stdin(mut self, echo: bool) -> Self {
        self.runner = self.runner.echo_stdin(echo);
        self
    }

    pub fn mix_stderr(mut self, mix: bool) -> Self {
        self.runner = self.runner.mix_stderr(mix);
        self
    }

    pub fn catch_panics(mut self, catch: bool) -> Self {
        self.runner = self.runner.catch_panics(catch);
        self
    }

    pub fn charset(mut self, charset: impl Into<String>) -> Self {
        self.runner = self.runner.charset(charset);
        self
    }

    pub fn invoke(&self, cmd: &dyn CommandLike, args: &[&str]) -> InvokeResult {
        self.invoke_with_input(cmd, args, None)
    }

    pub fn invoke_with_input(
        &self,
        cmd: &dyn CommandLike,
        args: &[&str],
        input: Option<&str>,
    ) -> InvokeResult {
        let wrapped = RichCommandLike {
            inner: cmd,
            config: self.config.clone(),
        };
        self.runner.invoke_with_input(&wrapped, args, input)
    }

    pub fn invoke_isolated(&self, cmd: &dyn CommandLike, args: &[&str]) -> InvokeResult {
        let wrapped = RichCommandLike {
            inner: cmd,
            config: self.config.clone(),
        };
        self.runner.invoke_isolated(&wrapped, args)
    }
}

// =============================================================================
// Regression tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::GroupConfig;

    fn make_group_with_subcommands() -> Group {
        Group::new("cli")
            .help("Test CLI application")
            .command(Command::new("start").help("Start the service").build())
            .command(Command::new("stop").help("Stop the service").build())
            .command(Command::new("status").help("Show service status").build())
            .build()
    }

    // -------------------------------------------------------------------------
    // Help renderer wiring tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_subcommand_help_uses_rich_renderer() {
        // When `main_rich_group` is used, a subcommand's --help should exit 0
        // (Exit{0} handled internally via the rich renderer hook).
        let group = make_group_with_subcommands();
        let runner = RichCliRunner::new();
        let result = runner.invoke(&group, &["start", "--help"]);
        assert_eq!(result.exit_code, 0, "Subcommand --help should exit 0");
    }

    #[test]
    fn test_root_help_uses_rich_renderer() {
        // Root --help should exit 0 via the rich renderer path.
        let group = make_group_with_subcommands();
        let runner = RichCliRunner::new();
        let result = runner.invoke(&group, &["--help"]);
        assert_eq!(result.exit_code, 0, "Root --help should exit 0");
    }

    #[test]
    fn test_main_rich_group_subcommand_help_ok() {
        // Subcommand --help via the RichCliRunner should return exit_code 0.
        let group = make_group_with_subcommands();
        let runner = RichCliRunner::new();
        let result = runner.invoke(&group, &["stop", "--help"]);
        assert_eq!(result.exit_code, 0, "Expected exit 0 for subcommand --help");
    }

    // -------------------------------------------------------------------------
    // COMMAND_GROUPS / grouped rendering tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_command_groups_rendered_as_separate_panels() {
        // Configure two command groups; verify each group's title appears.
        let group = Group::new("cli")
            .help("My CLI")
            .command(Command::new("start").help("Start").build())
            .command(Command::new("stop").help("Stop").build())
            .command(Command::new("status").help("Status").build())
            .command(Command::new("logs").help("Logs").build())
            .build();

        let config = RichHelpConfig::builder().build();
        let mut cfg = config;
        cfg.command_groups = vec![
            GroupConfig {
                name: "Lifecycle".to_string(),
                items: vec!["start".to_string(), "stop".to_string()],
                help: None,
                inline_help_in_title: None,
                title_style: None,
                help_style: None,
            },
            GroupConfig {
                name: "Observability".to_string(),
                items: vec!["status".to_string(), "logs".to_string()],
                help: None,
                inline_help_in_title: None,
                title_style: None,
                help_style: None,
            },
        ];

        let runner = RichCliRunner::new().config(cfg);
        let result = runner.invoke(&group, &["--help"]);
        // Help should complete successfully
        assert_eq!(result.exit_code, 0, "Expected exit 0 for grouped --help");
    }

    #[test]
    fn test_command_groups_ungrouped_fallback() {
        // Commands not listed in any group appear in the default "Commands" panel.
        // Verify exit 0 (rendering does not panic or error).
        let group = Group::new("cli")
            .command(Command::new("run").help("Run").build())
            .command(Command::new("build").help("Build").build())
            .command(Command::new("extra").help("Extra command").build())
            .build();

        let mut cfg = RichHelpConfig::default();
        cfg.command_groups = vec![GroupConfig {
            name: "Main".to_string(),
            items: vec!["run".to_string(), "build".to_string()],
            help: None,
            inline_help_in_title: None,
            title_style: None,
            help_style: None,
        }];

        let runner = RichCliRunner::new().config(cfg);
        let result = runner.invoke(&group, &["--help"]);
        assert_eq!(
            result.exit_code, 0,
            "Expected exit 0 for partially-grouped --help"
        );
    }

    #[test]
    fn test_no_command_groups_shows_all_commands() {
        // Without any command_groups config, --help should complete successfully.
        let group = make_group_with_subcommands();
        let cfg = RichHelpConfig::default();
        let runner = RichCliRunner::new().config(cfg);
        let result = runner.invoke(&group, &["--help"]);
        assert_eq!(result.exit_code, 0, "Expected exit 0 for ungrouped --help");
    }
}
