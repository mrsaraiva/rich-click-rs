# rich-click-rs

Rich help formatting for [`click-rs`](../click-rs), powered by [`rich-rs`](../rich-rs).

This crate provides rich-styled help output (usage, panels, option tables, and command tables)
that mirrors the Python rich-click experience, but for Rust.

## Quick Start

```toml
[dependencies]
click = { path = "../click-rs" }
rich-rs = { path = "../rich-rs" }
rich-click-rs = { path = "../rich-click-rs" }
```

```rust
use click::Command;
use rich_click_rs::{RichHelp, RichHelpConfig, RichMainExt};

fn main() {
    let cmd = Command::new("demo")
        .help("A demo CLI")
        .option(
            click::OptionBuilder::new(&["-v", "--verbose"])
                .help("Enable verbose output")
                .build(),
        )
        .build();

    let ctx = click::ContextBuilder::new().info_name("demo").build();
    let help = cmd.get_rich_help_with(&ctx, &RichHelpConfig::default());
    println!("{}", help);

    // Or run directly with rich help (and rich errors):
    // cmd.main_rich_with_errors().unwrap();
}
```

## Macros

For a tiny `main.rs`, use macros:

```rust
use rich_click_rs::rich_main_with_errors;

fn main() {
    let cmd = click::Command::new("demo").build();
    rich_main_with_errors!(cmd).unwrap();
}
```

## Themes

List available themes:

```bash
cargo run --bin rich-click -- --themes
```

Set a theme via env var (matches Python rich-click):

```bash
RICH_CLICK_THEME=solarized cargo run --example basic
```

## Optional Enhancements (Off by Default)

These are optional helpers that go beyond Python rich-click parity. All are disabled by default.

- Prompt metadata in help output:
  - Enable `show_prompt = true`.
  - Customize `prompt_string`, `prompt_confirm_string`, `prompt_hidden_string`, `prompt_confirm_hidden_string`.
  - If you want full ordering control, add `"prompt"` to `options_table_help_sections`.
- Parameter source in errors:
  - Enable `errors_show_param_source = true`.
  - Customize `errors_param_source_format` (use `{}` for the source label).
- Rich test runner:
  - Use `RichCliRunner` to capture rich help and rich errors when testing.

```rust
use click::Command;
use rich_click_rs::{RichCliRunner, RichHelpConfig};

let cmd = Command::new("demo")
    .option(
        click::OptionBuilder::new(&["--token"])
            .prompt("Token")
            .hide_input(true)
            .build(),
    )
    .build();

let config = RichHelpConfig::builder()
    .build();

let result = RichCliRunner::new()
    .config(RichHelpConfig { show_prompt: true, ..config })
    .invoke(&cmd, &["--help"]);
```

## Status

Feature-parity with Python rich-click for core help rendering, with parity tests and example suite
comparisons in place. Optional enhancements (off by default) are available for prompt metadata in
help output, error parameter-source hints, and a rich-aware test runner.
