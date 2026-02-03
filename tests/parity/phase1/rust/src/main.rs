use click::{Argument, Command, ContextBuilder, OptionBuilder};
use rich_click_rs::{RichHelp, RichHelpConfig};

fn main() {
    let cmd = Command::new("cli")
        .help("Say hello")
        .option(
            OptionBuilder::new(&["--count"])
                .help("Number of greetings")
                .default("1")
                .show_default(true)
                .build(),
        )
        .option(
            OptionBuilder::new(&["--verbose", "--no-verbose"])
                .help("Verbose output")
                .is_flag(true)
                .build(),
        )
        .argument(Argument::new("name").build())
        .build();

    let ctx = ContextBuilder::new().info_name("cli").build();
    let help = cmd.get_rich_help_with(&ctx, &RichHelpConfig::default());
    print!("{}", help);
}
