use click::argument::Argument;
use click::command::Command;
use click::context::ContextBuilder;
use click::option::OptionBuilder;
use click::types::{Choice, PathType};
use rich_click_rs::{RichHelp, RichHelpConfig};

fn main() {
    let cli = Command::new("06_arguments")
        .help(
            "My amazing tool does all the things.\n\n\
This is a minimal example based on documentation\n\
from the 'click' package.\n\n\
You can try using --help at the top level and also for\n\
specific group subcommands.",
        )
        .argument(
            Argument::new("input")
                .type_(PathType::new())
                .required(true)
                .build(),
        )
        .argument(
            Argument::new("output")
                .type_(PathType::new())
                .required(true)
                .build(),
        )
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
        .option(
            OptionBuilder::new(&["--all"])
                .flag("true")
                .help("Sync all the things?")
                .build(),
        )
        .option(
            OptionBuilder::new(&["--debug"])
                .flag("true")
                .help("Enable debug mode")
                .build(),
        )
        .callback(|_ctx| Ok(()))
        .build();

    let ctx = ContextBuilder::new().info_name("06_arguments").build();
    let mut cfg = RichHelpConfig::default();
    cfg.show_arguments = Some(true);
    println!("{}", cli.get_rich_help_with(&ctx, &cfg));
}
