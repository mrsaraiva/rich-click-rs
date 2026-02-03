use click::command::Command;
use click::context::Context;
use click::group::Group;
use click::option::OptionBuilder;
use rich_click_rs::{main_rich_group, RichHelpConfig};

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
    let sync = Command::new("sync")
        .help("Synchronise all your files between two places.")
        .callback(|_ctx| {
            println!("Syncing");
            Ok(())
        })
        .build();

    let cli = Group::new("02_declarative.py")
        .help(
            "My amazing tool does all the things.\n\n\
This is a minimal example based on documentation\n\
from the 'click' package.\n\n\
You can try using --help at the top level and also for\n\
specific group subcommands.",
        )
        .option(
            OptionBuilder::new(&["--debug", "--no-debug"])
                .bool_flag()
                .default("false")
                .build(),
        )
        .callback(|ctx| {
            let debug = get_bool_param(ctx, "debug");
            println!("Debug mode is {}", if debug { "on" } else { "off" });
            Ok(())
        })
        .command(sync)
        .build();

    let args: Vec<String> = std::env::args().skip(1).collect();
    let config = RichHelpConfig::default();
    if let Err(err) = main_rich_group(&cli, args, &config) {
        eprintln!("{}", err.format_full());
        std::process::exit(err.exit_code());
    }
}
