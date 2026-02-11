use click::argument::Argument;
use click::command::Command;
use click::context::ContextBuilder;
use click::option::OptionBuilder;
use click::types::PathType;
use rich_click_rs::{RichHelp, RichHelpConfig};
use rich_rs::Style;

fn main() {
    let cli = Command::new("07_custom_errors")
        .help(
            "My amazing tool does all the things.\n\n\
This is a minimal example based on documentation\n\
from the 'click' package.\n\n\
You can try using --help at the top level and also for\n\
specific group subcommands.",
        )
        .argument(Argument::new("input").type_(PathType::new()).required(true).build())
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
                .default("false")
                .help("Enable debug mode")
                .build(),
        )
        .callback(|_ctx| Ok(()))
        .build();

    let ctx = ContextBuilder::new().info_name("07_custom_errors").build();
    let mut cfg = RichHelpConfig::default();
    cfg.style_errors_suggestion = Style::parse("magenta italic");
    cfg.errors_suggestion = Some("Try running the '--help' flag for more information.".to_string());
    cfg.errors_epilogue = Some(
        "To find out more, visit [link=https://mytool.com]https://mytool.com[/link]".to_string(),
    );
    println!("{}", cli.get_rich_help_with(&ctx, &cfg));
}
