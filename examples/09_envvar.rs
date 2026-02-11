use click::command::Command;
use click::context::ContextBuilder;
use click::group::Group;
use click::option::OptionBuilder;
use rich_click_rs::{RichHelp, RichHelpConfig};

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
        .callback(|_ctx| Ok(()))
        .build();

    let cli = Group::new("09_envvar")
        .option(
            OptionBuilder::new(&["--debug", "--no-debug"])
                .bool_flag()
                .build(),
        )
        .callback(|_ctx| Ok(()))
        .command(greet)
        .build();

    let ctx = ContextBuilder::new().info_name("09_envvar").build();
    let config = RichHelpConfig::default();
    println!("{}", cli.get_rich_help_with(&ctx, &config));
}
