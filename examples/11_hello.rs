use click::command::Command;
use click::context::ContextBuilder;
use click::option::OptionBuilder;
use click::types::INT;
use rich_click_rs::{RichHelp, RichHelpConfig, TextMarkup};

fn main() {
    let hello = Command::new("11_hello")
        .help("Simple program that greets `NAME` for a total of `COUNT` times.")
        .option(
            OptionBuilder::new(&["--count"])
                .default("1")
                .help("Number of greetings.")
                .type_any(INT)
                .build(),
        )
        .option(
            OptionBuilder::new(&["--name"])
                .prompt("Your name")
                .help("The person to greet.")
                .build(),
        )
        .callback(|_ctx| Ok(()))
        .build();

    let ctx = ContextBuilder::new().info_name("11_hello").build();
    let mut config = RichHelpConfig::default();
    config.text_markup = TextMarkup::Markdown;
    config.width = Some(60);
    println!("{}", hello.get_rich_help_with(&ctx, &config));
}
