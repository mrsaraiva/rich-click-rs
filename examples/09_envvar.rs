use click::command::Command;
use click::context::Context;
use click::group::Group;
use click::option::OptionBuilder;
use rich_click_rs::{main_rich_group, RichHelpConfig};

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
    let greet = Command::new("greet")
        .option(
            OptionBuilder::new(&["--username"])
                .envvar("GREETER_GREET_USERNAME")
                .show_envvar()
                .help("This can be set via env var GREETER_GREET_USERNAME")
                .build(),
        )
        .option(
            OptionBuilder::new(&["--nickname"])
                .envvar("NICKNAME")
                .show_envvar()
                .show_default()
                .help("This can be set via env var NICKNAME")
                .build(),
        )
        .option(
            OptionBuilder::new(&["--email"])
                .envvars(["EMAIL", "EMAIL_ADDRESS"])
                .show_envvar()
                .default("foo@bar.com")
                .show_default()
                .help("This can be set via env var EMAIL or EMAIL_ADDRESS")
                .build(),
        )
        .callback(|ctx| {
            let username = ctx.get_param::<String>("username").map(|v| v.as_str()).unwrap_or("");
            let nickname = ctx.get_param::<String>("nickname").map(|v| v.as_str()).unwrap_or("");
            let email = ctx.get_param::<String>("email").map(|v| v.as_str()).unwrap_or("");
            println!("Hello {} ({}) with email {}!", username, nickname, email);
            Ok(())
        })
        .build();

    let cli = Group::new("09_envvar.py")
        .option(
            OptionBuilder::new(&["--debug", "--no-debug"])
                .bool_flag()
                .build(),
        )
        .callback(|ctx| {
            let debug = get_bool_param(ctx, "debug");
            println!("Debug mode is {}", if debug { "on" } else { "off" });
            Ok(())
        })
        .command(greet)
        .build();

    let args: Vec<String> = std::env::args().skip(1).collect();
    let config = RichHelpConfig::default();
    if let Err(err) = main_rich_group(&cli, args, &config) {
        eprintln!("{}", err.format_full());
        std::process::exit(err.exit_code());
    }
}
