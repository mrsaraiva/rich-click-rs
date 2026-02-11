use click::command::Command;
use click::context::ContextBuilder;
use click::option::OptionBuilder;
use click::types::PathType;
use rich_click_rs::{RichHelp, RichHelpConfig, TextMarkup};

fn main() {
    let cli = Command::new("04_rich_markup")
        .help(
            "My amazing tool does [black on blue]all the things[/].\n\n\
This is a [u]minimal example[/] based on documentation\n\
from the [link=https://click.palletsprojects.com/]'click' package[/].\n\n\
[i]You can try using --help at the top level and also for\n\
specific group subcommands.[/]",
        )
        .option(
            OptionBuilder::new(&["--input"])
                .help(r"Input [magenta bold]file[/]. [dim]\[default: a custom default][/]")
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
        .option(OptionBuilder::new(&["--all"]).flag("true").help("Sync all the things?").build())
        .option(
            OptionBuilder::new(&["--debug"])
                .flag("true")
                .help("Enable :point_right: [yellow]debug mode[/] :point_left:")
                .build(),
        )
        .callback(|_ctx| Ok(()))
        .build();

    let ctx = ContextBuilder::new().info_name("04_rich_markup").build();
    let mut cfg = RichHelpConfig::default();
    cfg.text_markup = TextMarkup::Rich;
    cfg.text_emojis = Some(true);
    println!("{}", cli.get_rich_help_with(&ctx, &cfg));
}
