use click::{Command, ContextBuilder, OptionBuilder};
use rich_click_rs::{RichHelp, RichHelpConfig};

fn main() {
    let cmd = Command::new("demo")
        .help("A demo CLI")
        .option(
            OptionBuilder::new(&["-v", "--verbose"]) 
                .help("Enable verbose output")
                .build(),
        )
        .build();

    let ctx = ContextBuilder::new().info_name("demo").build();
    let help = cmd.get_rich_help_with(&ctx, &RichHelpConfig::default());
    println!("{}", help);
}
