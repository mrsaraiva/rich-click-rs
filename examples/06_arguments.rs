use click::argument::Argument;
use click::command::Command;
use click::context::Context;
use click::option::OptionBuilder;
use click::types::{Choice, PathType};
use rich_click_rs::{main_rich_command, RichHelpConfig};

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
    let cli = Command::new("06_arguments.py")
        .help(
            "My amazing tool does all the things.\n\n\
This is a minimal example based on documentation\n\
from the 'click' package.\n\n\
You can try using --help at the top level and also for\n\
specific group subcommands.",
        )
        .argument(Argument::new("input").type_(PathType::new()).required(true).build())
        .argument(Argument::new("output").type_(PathType::new()).required(true).build())
        .argument(
            Argument::new("format")
                .type_(Choice::new(["yaml", "json"]))
                .required(true)
                .build(),
        )
        .argument(Argument::new("flavour").required(false).build())
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
                .help("Enable debug mode")
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
    cfg.show_arguments = Some(true);
    if let Err(err) = main_rich_command(&cli, args, &cfg) {
        eprintln!("{}", err.format_full());
        std::process::exit(err.exit_code());
    }
}
