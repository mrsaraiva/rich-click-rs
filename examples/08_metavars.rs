use click::command::Command;
use click::context::ContextBuilder;
use click::option::OptionBuilder;
use click::types::Choice;
use rich_click_rs::{RichHelp, RichHelpConfig};

fn main() {
    let cli = Command::new("08_metavars")
        .help(
            "My amazing tool does all the things.\n\n\
This is a minimal example based on documentation\n\
from the 'click' package.\n\n\
You can try using --help at the top level and also for\n\
specific group subcommands.",
        )
        .option(
            OptionBuilder::new(&["--debug"])
                .flag("true")
                .help("Enable debug mode.")
                .build(),
        )
        .option(
            OptionBuilder::new(&["--number"])
                .type_any(Choice::new([
                    "one",
                    "two",
                    "three",
                    "four",
                    "five",
                    "six",
                    "seven",
                    "eight",
                    "nine",
                    "ten",
                    "eleven",
                    "twelve",
                    "thirteen",
                    "fourteen",
                    "fifteen",
                    "sixteen",
                    "seventeen",
                    "eighteen",
                    "nineteen",
                    "twenty",
                    "twenty-one",
                    "twenty-two",
                    "twenty-three",
                    "twenty-four",
                    "twenty-five",
                    "twenty-six",
                    "twenty-seven",
                    "twenty-eight",
                    "twenty-nine",
                    "thirty",
                ]))
                .show_default()
                .help("This click choice has loads of options.")
                .build(),
        )
        .callback(|_ctx| Ok(()))
        .build();

    let ctx = ContextBuilder::new().info_name("08_metavars").build();
    let mut cfg = RichHelpConfig::default();
    cfg.show_metavars_column = Some(false);
    cfg.append_metavars_help = Some(true);
    println!("{}", cli.get_rich_help_with(&ctx, &cfg));
}
