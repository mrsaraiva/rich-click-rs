use click::command::Command;
use click::context::Context;
use click::option::OptionBuilder;
use click::types::Choice;
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
    let cli = Command::new("08_metavars_default.py")
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
        .callback(|ctx| {
            let debug = get_bool_param(ctx, "debug");
            println!("Debug mode is {}", if debug { "on" } else { "off" });
            Ok(())
        })
        .build();

    let args: Vec<String> = std::env::args().skip(1).collect();
    let cfg = RichHelpConfig::default();
    if let Err(err) = main_rich_command(&cli, args, &cfg) {
        eprintln!("{}", err.format_full());
        std::process::exit(err.exit_code());
    }
}
