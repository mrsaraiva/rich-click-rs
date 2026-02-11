use click::command::Command;
use click::context::ContextBuilder;
use click::option::OptionBuilder;
use click::types::PathType;
use rich_click_rs::{RichHelp, RichHelpConfig, TextMarkup};

fn main() {
    let cli = Command::new("05_markdown")
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
        .callback(|_ctx| Ok(()))
        .build();

    let ctx = ContextBuilder::new().info_name("05_markdown").build();
    let mut cfg = RichHelpConfig::default();
    cfg.text_markup = TextMarkup::Markdown;
    println!("{}", cli.get_rich_help_with(&ctx, &cfg));
}
