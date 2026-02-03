use click::{Command, ContextBuilder, OptionBuilder};
use click::group::Group;
use rich_click_rs::{GroupConfig, RichHelp, RichHelpConfig};

fn main() {
    let run = Command::new("run")
        .help("Run command.")
        .option(OptionBuilder::new(&["--alpha"]).metavar("TEXT").help("Alpha option").build())
        .option(OptionBuilder::new(&["--beta"]).metavar("TEXT").help("Beta option").build())
        .build();

    let stop = Command::new("stop")
        .help("Stop command.")
        .option(OptionBuilder::new(&["--gamma"]).metavar("TEXT").help("Gamma option").build())
        .build();

    let group = Group::new("cli-panels")
        .help("Panel and table styling tests.")
        .command(run)
        .command(stop)
        .build();

    println!("=== panels ===");
    let ctx = ContextBuilder::new().info_name("cli-panels").build();
    let mut cfg = RichHelpConfig::default();
    cfg.panel_inline_help_in_title = true;
    cfg.panel_inline_help_delimiter = " — ".to_string();
    cfg.option_groups = vec![GroupConfig {
        name: "Core".to_string(),
        items: vec!["--alpha".to_string(), "--beta".to_string()],
        help: Some("Core options".to_string()),
        help_style: None,
        title_style: None,
        inline_help_in_title: Some(true),
    }];
    cfg.command_groups = vec![GroupConfig {
        name: "Subcommands".to_string(),
        items: vec!["run".to_string(), "stop".to_string()],
        help: Some("Task controls".to_string()),
        help_style: None,
        title_style: None,
        inline_help_in_title: Some(true),
    }];
    cfg.options_table_column_types = vec!["opt_long".to_string(), "metavar".to_string(), "help".to_string()];
    cfg.commands_table_column_types = vec!["name".to_string(), "help".to_string()];
    cfg.style_options_table_show_lines = true;
    cfg.style_commands_table_show_lines = true;
    let help = group.get_rich_help_with(&ctx, &cfg);
    print!("{}", help);
}
