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
