use click::command::Command;
use click::context::Context;
use click::option::OptionBuilder;
use click::types::PathType;
use rich_click_rs::{main_rich_command, RichHelpConfig, TextMarkup};

fn get_bool_param(ctx: &Context, name: &str) -> bool {
    if let Some(value) = ctx.get_param::<bool>(name) {
        return *value;
    }
    if let Some(value) = ctx.get_param::<String>(name) {
        return value == "true";
    }
    false
}

fn main() {
    let cli = Command::new("05_markdown.py")
        .help(
            "My amazing tool does _**all the things**_.\n\n\
This is a `minimal example` based on documentation from the [_click_ package](https://click.palletsprojects.com/).\n\n\
> Remember:\n\
>  - You can try using --help at the top level\n\
>  - Also for specific group subcommands.\n\n",
        )
        .option(
            OptionBuilder::new(&["--input"])
                .help("Input **file**. _[default: a custom default]_")
                .type_any(PathType::new())
                .build(),
        )
        .option(
            OptionBuilder::new(&["--type"])
                .default("files")
                .show_default()
                .help("Type of file to sync")
                .build(),
        )
        .option(
            OptionBuilder::new(&["--all"])
                .flag("true")
                .help("Sync\n 1. all\n 2. the\n 3. things?")
                .build(),
        )
        .option(
            OptionBuilder::new(&["--debug"])
                .flag("true")
                .help("# Enable `debug mode`")
                .build(),
        )
        .callback(|ctx| {
            let debug = get_bool_param(ctx, "debug");
            println!("Debug mode is {}", if debug { "on" } else { "off" });
            Ok(())
        })
        .build();

    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut cfg = RichHelpConfig::default();
    cfg.text_markup = TextMarkup::Markdown;
    if let Err(err) = main_rich_command(&cli, args, &cfg) {
        eprintln!("{}", err.format_full());
        std::process::exit(err.exit_code());
    }
}
