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
    let cli = Command::new("04_rich_markup.py")
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
        .callback(|ctx| {
            let debug = get_bool_param(ctx, "debug");
            println!("Debug mode is {}", if debug { "on" } else { "off" });
            Ok(())
        })
        .build();

    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut cfg = RichHelpConfig::default();
    cfg.text_markup = TextMarkup::Rich;
    cfg.text_emojis = Some(true);
    if let Err(err) = main_rich_command(&cli, args, &cfg) {
        eprintln!("{}", err.format_full());
        std::process::exit(err.exit_code());
    }
}
