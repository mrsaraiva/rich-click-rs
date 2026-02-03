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
use rich_click_rs::{RichHelp, RichHelpConfig};

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

## Status

This is an early scaffold focused on help rendering (usage, panels, option/command tables).
More parity features will be added as we iterate.
