use click::command::Command;
use click::context::Context;
use click::option::OptionBuilder;
use click::types::INT;
use rich_click_rs::{main_rich_command, RichHelpConfig, TextMarkup};

fn get_string_param(ctx: &Context, name: &str) -> Option<String> {
    if let Some(value) = ctx.get_param::<String>(name) {
        return Some(value.clone());
    }
    None
}

fn get_count_param(ctx: &Context, name: &str) -> i64 {
    if let Some(value) = ctx.get_param::<i64>(name) {
        return *value;
    }
    if let Some(value) = ctx.get_param::<String>(name) {
        return value.parse::<i64>().unwrap_or(1);
    }
    1
}

fn main() {
    let hello = Command::new("11_hello.py")
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
        .callback(|ctx| {
            let count = get_count_param(ctx, "count");
            let name = get_string_param(ctx, "name").unwrap_or_else(|| "World".to_string());
            for _ in 0..count {
                println!("Hello, {}!", name);
            }
            Ok(())
        })
        .build();

    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut config = RichHelpConfig::default();
    config.text_markup = TextMarkup::Markdown;
    config.width = Some(60);
    if let Err(err) = main_rich_command(&hello, args, &config) {
        eprintln!("{}", err.format_full());
        std::process::exit(err.exit_code());
    }
}
