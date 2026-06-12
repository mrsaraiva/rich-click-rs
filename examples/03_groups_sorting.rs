use click::command::Command;
use click::context::ContextBuilder;
use click::group::Group;
use click::option::OptionBuilder;
use click::types::PathType;
use rich_click_rs::{GroupConfig, RichHelp, RichHelpConfig};

fn main() {
    let sync = Command::new("sync")
        .help("Synchronise all your files between two places.")
        .option(
            OptionBuilder::new(&["--input", "-i"])
                .required()
                .help("Input path")
                .type_any(PathType::new())
                .build(),
        )
        .option(
            OptionBuilder::new(&["--output", "-o"])
                .help("Output path")
                .type_any(PathType::new())
                .build(),
        )
        .option(
            OptionBuilder::new(&["--all"])
                .flag("true")
                .help("Sync all the things?")
                .build(),
        )
        .option(
            OptionBuilder::new(&["--overwrite"])
                .flag("true")
                .help("Overwrite local files")
                .build(),
        )
        .callback(|_ctx| {
            println!("Syncing");
            Ok(())
        })
        .build();

    let download = Command::new("download")
        .help("Pretend to download some files from somewhere.")
        .option(
            OptionBuilder::new(&["--all"])
                .flag("true")
                .help("Get everything")
                .build(),
        )
        .callback(|_ctx| {
            println!("Downloading");
            Ok(())
        })
        .build();

    let auth = Command::new("auth")
        .help("Authenticate the app.")
        .callback(|_ctx| {
            println!("Downloading");
            Ok(())
        })
        .build();

    let config_cmd = Command::new("config")
        .help("Set up the configuration.")
        .callback(|_ctx| {
            println!("Downloading");
            Ok(())
        })
        .build();

    let version_opt = OptionBuilder::new(&["--version"])
        .flag("true")
        .eager()
        .metavar("__click_version__:mytool 1.23")
        .help("Show the version and exit.")
        .build();

    let help_opt = OptionBuilder::new(&["-h", "--help"])
        .flag("true")
        .eager()
        .help("Show this message and exit.")
        .build();

    let cli = Group::new("03_groups_sorting")
        .help(
            "My amazing tool does all the things.\n\n\
This is a minimal example based on documentation\n\
from the 'click' package.\n\n\
You can try using --help at the top level and also for\n\
specific subcommands.",
        )
        .option(
            OptionBuilder::new(&["--type"])
                .required()
                .default("files")
                .show_default()
                .help("Type of file to sync")
                .build(),
        )
        .option(
            OptionBuilder::new(&["--debug", "--no-debug", "-d", "-n"])
                .bool_flag()
                .default("false")
                .show_default()
                .help("Show the debug log messages")
                .build(),
        )
        .option(version_opt)
        .help_option(help_opt)
        .callback(|_ctx| Ok(()))
        .command(sync)
        .command(download)
        .command(auth)
        .command(config_cmd)
        .build();

    let mut cfg = RichHelpConfig::default();
    cfg.theme = Some("magenta2-modern".to_string());
    let _ = cfg.apply_theme_name("magenta2-modern");
    cfg.option_groups = vec![
        GroupConfig {
            name: "Basic usage".to_string(),
            items: vec!["--type".to_string(), "--output".to_string()],
            help: None,
            inline_help_in_title: None,
            title_style: None,
            help_style: None,
        },
        GroupConfig {
            name: "Advanced options".to_string(),
            items: vec![
                "--help".to_string(),
                "--version".to_string(),
                "--debug".to_string(),
            ],
            help: None,
            inline_help_in_title: None,
            title_style: None,
            help_style: None,
        },
    ];
    cfg.command_groups = vec![
        GroupConfig {
            name: "Main usage".to_string(),
            items: vec!["sync".to_string(), "download".to_string()],
            help: None,
            inline_help_in_title: None,
            title_style: None,
            help_style: None,
        },
        GroupConfig {
            name: "Configuration".to_string(),
            items: vec!["config".to_string(), "auth".to_string()],
            help: None,
            inline_help_in_title: None,
            title_style: None,
            help_style: None,
        },
    ];

    let ctx = ContextBuilder::new().info_name("03_groups_sorting").build();
    println!("{}", cli.get_rich_help_with(&ctx, &cfg));
}
