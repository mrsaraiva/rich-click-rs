use click::{Argument, Command, ContextBuilder, OptionBuilder};
use click::group::{CommandLike, Group};
use click::types::INT;
use std::sync::Arc;
use rich_click_rs::{RichHelp, RichHelpConfig, TextMarkup};

fn main() {
    let markdown = Command::new("cli-markdown")
        .help("Markdown **bold** and `code`.")
        .option(
            OptionBuilder::new(&["--count"])
                .help("Number of greetings")
                .type_any(INT)
                .default("1")
                .show_default()
                .build(),
        )
        .argument(Argument::new("name").build())
        .build();

    let markup = Command::new("cli-markup")
        .help("Markup [bold]bold[/] and :sparkles:.")
        .option(OptionBuilder::new(&["--spark"]).help("Emit :sparkles:").bool_flag().build())
        .build();

    let width = Command::new("cli-width")
        .help("A very long description that should wrap when constrained by width.")
        .option(
            OptionBuilder::new(&["--mode"])
                .help("Select the execution mode for the operation")
                .default("fast")
                .show_default()
                .build(),
        )
        .build();

    let start = Command::new("start").help("Start command").build();
    let start_shared: Arc<dyn CommandLike> = Arc::new(start);
    let alias = Group::new("cli-alias")
        .help("Alias test.")
        .command_shared(Arc::clone(&start_shared))
        .command_shared_with_name("run", Arc::clone(&start_shared))
        .build();

    println!("=== markdown ===");
    let ctx = ContextBuilder::new().info_name("cli-markdown").build();
    let mut cfg = RichHelpConfig::default();
    cfg.text_markup = TextMarkup::Markdown;
    let help = markdown.get_rich_help_with(&ctx, &cfg);
    print!("{}", help);

    println!("=== markup ===");
    let ctx = ContextBuilder::new().info_name("cli-markup").build();
    let mut cfg = RichHelpConfig::default();
    cfg.text_markup = TextMarkup::Rich;
    cfg.text_emojis = Some(false);
    let help = markup.get_rich_help_with(&ctx, &cfg);
    print!("{}", help);

    println!("=== width ===");
    let ctx = ContextBuilder::new().info_name("cli-width").build();
    let mut cfg = RichHelpConfig::default();
    cfg.width = Some(40);
    cfg.max_width = Some(40);
    let help = width.get_rich_help_with(&ctx, &cfg);
    print!("{}", help);

    println!("=== aliases ===");
    let ctx = ContextBuilder::new().info_name("cli-alias").build();
    let mut cfg = RichHelpConfig::default();
    cfg.helptext_show_aliases = false;
    let help = alias.get_rich_help_with(&ctx, &cfg);
    print!("{}", help);
}
