use click::command::Command;
use click::context::ContextBuilder;
use click::group::Group;
use click::option::OptionBuilder;
use rich_click_rs::{RichHelp, RichHelpConfig};

fn main() {
    let sync = Command::new("sync")
        .help("Synchronise all your files between two places.")
        .callback(|_ctx| {
            println!("Syncing");
            Ok(())
        })
        .build();

    let cli = Group::new("02_declarative")
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
        .callback(|_ctx| Ok(()))
        .command(sync)
        .build();

    let ctx = ContextBuilder::new().info_name("02_declarative").build();
    let config = RichHelpConfig::default();
    println!("{}", cli.get_rich_help_with(&ctx, &config));
}
