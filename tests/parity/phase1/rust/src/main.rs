use click::{Argument, Command, ContextBuilder, OptionBuilder};
use click::group::CommandLike;
use click::group::Group;
use std::sync::Arc;
use rich_click_rs::{RichHelp, RichHelpConfig};

fn main() {
    let basic = Command::new("cli-basic")
        .help("Say hello")
        .option(
            OptionBuilder::new(&["--count"])
                .help("Number of greetings")
                .default("1")
                .show_default()
                .build(),
        )
        .option(
            OptionBuilder::new(&["--verbose", "--no-verbose"])
                .help("Verbose output")
                .bool_flag()
                .build(),
        )
        .argument(Argument::new("name").build())
        .build();

    let meta = Command::new("cli-meta")
        .help("Meta test")
        .option(
            OptionBuilder::new(&["--env"])
                .help("Uses envvar")
                .envvar("MY_ENV")
                .show_envvar()
                .build(),
        )
        .option(
            OptionBuilder::new(&["--req"])
                .help("Required value")
                .required()
                .build(),
        )
        .option(
            OptionBuilder::new(&["--mode"])
                .help("Mode")
                .default("fast")
                .show_default()
                .build(),
        )
        .argument(Argument::new("path").build())
        .build();

    let start = Command::new("start")
        .help("Start command")
        .option(
            OptionBuilder::new(&["--speed"])
                .help("Speed")
                .default("1")
                .show_default()
                .build(),
        )
        .build();
    let start_shared: Arc<dyn CommandLike> = Arc::new(start);
    let group = Group::new("group")
        .help("Group help")
        .command_shared(Arc::clone(&start_shared))
        .command_shared_with_name("run", Arc::clone(&start_shared))
        .build();

    println!("=== basic ===");
    let ctx = ContextBuilder::new().info_name("cli-basic").build();
    let help = basic.get_rich_help_with(&ctx, &RichHelpConfig::default());
    print!("{}", help);

    println!("=== slim-theme ===");
    let ctx = ContextBuilder::new().info_name("cli-basic").build();
    let cfg = RichHelpConfig::builder().theme("slim").build();
    let help = basic.get_rich_help_with(&ctx, &cfg);
    print!("{}", help);

    println!("=== metadata ===");
    let ctx = ContextBuilder::new().info_name("cli-meta").build();
    let help = meta.get_rich_help_with(&ctx, &RichHelpConfig::default());
    print!("{}", help);

    println!("=== group-alias ===");
    let ctx = ContextBuilder::new().info_name("group").build();
    let help = group.get_rich_help_with(&ctx, &RichHelpConfig::default());
    print!("{}", help);
}
