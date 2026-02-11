use click::command::Command;
use click::context::ContextBuilder;
use click::group::Group;
use click::option::OptionBuilder;
use rich_click_rs::{RichHelp, RichHelpConfig};

fn main() {
    let sync = Command::new("sync")
        .short_help("Synchronise all your files between two places. Example command that doesn't do much except print to the terminal.")
        .help("Synchronise all your files between two places.\nExample command that doesn't do much except print to the terminal.")
        .option(
            OptionBuilder::new(&["--type"])
                .required()
                .default("files")
                .show_default()
                .help("Type of file to sync")
                .build(),
        )
        .option(OptionBuilder::new(&["--all"]).flag("true").build())
        .callback(|_ctx| {
            println!("Syncing");
            Ok(())
        })
        .build();

    let download_help = concat!(
        "Pretend to download some files from\n",
        "somewhere. Multi-line help strings are unwrapped\n",
        "until you use a double newline.\n\n",
        "Only the first paragraph is used in group help texts.\n",
        "Don't forget you can opt-in to rich and markdown formatting!\n\n",
        "\u{0008}\n",
        "Click escape markers should still work.\n",
        "  * So you\n",
        "  * Can keep\n",
        "  * Your newlines\n\n",
        "And this is a paragraph\n",
        "that will be rewrapped again.\n\n",
        "\u{000c}\n",
        "Also if you want to write function help text that won't\n",
        "be rendered to the terminal.\n",
    );

    let download = Command::new("download")
        .short_help("Optionally use short-help for the group help text")
        .help(download_help)
        .option(
            OptionBuilder::new(&["--all"])
                .flag("true")
                .help("Get everything")
                .build(),
        )
        .callback(|_ctx| {
            println!("Downloading");
            Ok(())
        })
        .build();

    let cli = Group::new("01_simple")
        .help(
            "My amazing tool does all the things.\n\n\
This is a minimal example based on documentation\n\
from the 'click' package.\n\n\
You can try using --help at the top level and also for\n\
specific subcommands.",
        )
        .option(
            OptionBuilder::new(&["--debug", "--no-debug", "-d", "-n"])
                .bool_flag()
                .default("false")
                .help(
                    "Enable debug mode.\n\
    Newlines are removed by default.\n\n\
    Double newlines are preserved.",
                )
                .build(),
        )
        .callback(|_ctx| Ok(()))
        .command(sync)
        .command(download)
        .build();

    let ctx = ContextBuilder::new().info_name("01_simple").build();
    let config = RichHelpConfig::default();
    println!("{}", cli.get_rich_help_with(&ctx, &config));
}
