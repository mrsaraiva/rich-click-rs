use click::command::Command;
use click::context::ContextBuilder;
use click::group::Group;
use click::option::OptionBuilder;
use click::types::PathType;
use rich_click_rs::{RichHelp, RichHelpConfig};
use rich_rs::r#box::{DOUBLE, SIMPLE};
use rich_rs::Style;

fn main() {
    let sync = Command::new("sync")
        .help(
            "Synchronise all your files between two places.\n\
Curabitur congue eget lorem in lacinia.\n\
Praesent tempus nunc nec nulla dignissim, et lacinia ipsum accumsan.\n\
Duis sodales, sapien at fermentum condimentum, diam metus porttitor lacus, nec gravida mi diam eget ligula.\n\
Pellentesque elementum at justo a luctus.\n\
Mauris a interdum odio.\n\
Maecenas in consectetur velit.\n\
Ut tristique congue felis at tempus.\n\
Donec pulvinar tortor ut odio posuere imperdiet.\n\
Fusce lacinia iaculis diam in scelerisque.\n\
Pellentesque in lorem est.\n\
Nulla efficitur luctus lacus, auctor auctor dui hendrerit a.\n\
Ut nec iaculis dolor.\n\
Morbi metus lectus, aliquet et sapien nec, congue euismod lorem.\n\
Pellentesque tristique tempus augue at convallis.",
        )
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
        .option(OptionBuilder::new(&["--all"]).flag("true").help("Sync all the things?").build())
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
        .help(
            "Pretend to download some files from somewhere.\n\
Integer bibendum libero nunc, sed aliquet ex tincidunt vel.\n\
Duis vitae sem vel odio luctus suscipit nec vitae enim.\n\
Curabitur vel lectus nec quam maximus dapibus.\n\
Phasellus eros velit, maximus non hendrerit nec, tempor fringilla urna.\n\
Vivamus vel nibh quis sapien consectetur fermentum.\n\
Curabitur at ultrices quam, vel molestie justo.\n\
Nunc lobortis orci vel nibh sagittis pretium.\n\
Morbi rhoncus sapien luctus, ultrices urna vel, convallis tortor.",
        )
        .option(OptionBuilder::new(&["--all"]).flag("true").help("Get everything").build())
        .callback(|_ctx| {
            println!("Downloading");
            Ok(())
        })
        .build();

    let auth = Command::new("auth")
        .help(
            "Authenticate the app.\n\
Duis lacus nibh, feugiat a nibh a, commodo dictum libero.\n\
Ut ac nulla tincidunt, bibendum nisi vitae, sodales ex.\n\
Vestibulum efficitur, lectus quis venenatis porta, dolor elit varius mauris, consequat interdum lectus est quis mi.\n\
Vestibulum imperdiet sed dolor eget semper.\n\
Cras ut mauris ac libero hendrerit congue.\n\
Vivamus pretium nunc turpis, eget imperdiet sapien tempor auctor.\n\
Phasellus risus nisi, laoreet in posuere sit amet, sodales non diam.\n\
Aliquam non malesuada urna, a faucibus risus.",
        )
        .callback(|_ctx| {
            println!("Downloading");
            Ok(())
        })
        .build();

    let config_cmd = Command::new("config")
        .help(
            "Set up the configuration.\n\
Sed accumsan ornare odio dictum aliquam.\n\
Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas.\n\
Curabitur in pellentesque mauris.\n\
Nulla mollis dui finibus, dictum neque id, suscipit nisl.\n\
Nunc mauris ex, laoreet nec tincidunt ut, pellentesque ut tortor.\n\
Mauris fermentum diam at porttitor tempor.\n\
Aliquam euismod nisi massa, nec placerat ante euismod quis.",
        )
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

    let cli = Group::new("10_table_styles")
        .help(
            "My amazing tool does all the things.\n\n\
This is a minimal example based on documentation\n\
from the 'click' package.\n\n\
You can try using --help at the top level and also for\n\
specific group subcommands.",
        )
        .option(
            OptionBuilder::new(&["--type"])
                .default("files")
                .help(
                    "Type of file to sync.\n\
Lorem ipsum dolor sit amet, consectetur adipiscing elit.\n\
Sed sed mauris euismod, semper leo quis, sodales augue.\n\
Donec posuere nulla quis egestas ornare.\n\
Nam efficitur ex quis diam tempus, nec euismod diam consectetur.\n\
Etiam vitae nisi at odio hendrerit dictum in at dui.\n\
Aliquam nulla lacus, pellentesque id ultricies sit amet, mollis nec tellus.\n\
Aenean arcu justo, pellentesque viverra justo eget, tempus tincidunt lectus.\n\
Maecenas porttitor risus vitae libero dapibus ullamcorper.\n\
Cras faucibus euismod erat in porta.\n\
Phasellus cursus gravida ante vel aliquet.\n\
In accumsan enim nec ullamcorper gravida.\n\
Donec malesuada dui ac metus tristique cursus.\n\
Sed gravida condimentum fermentum.\n\
Ut sit amet nulla commodo, iaculis tellus vitae, accumsan enim.\n\
Curabitur mollis semper velit a suscipit.",
                )
                .build(),
        )
        .option(
            OptionBuilder::new(&["--debug", "--no-debug", "-d", "-n"])
                .bool_flag()
                .default("false")
                .help(
                    "Show the debug log messages.\n\
Suspendisse dictum hendrerit turpis eu rutrum.\n\
Vivamus magna ex, elementum sit amet sapien laoreet, tempor consequat eros.\n\
Morbi semper feugiat nisi eget sodales.\n\
Pellentesque et turpis erat.\n\
Donec ac aliquam risus.\n\
Nam leo tellus, rutrum et scelerisque vitae, ultrices sed metus.\n\
Ut sollicitudin convallis turpis, sit amet sollicitudin felis semper feugiat.\n\
In sapien dui, aliquam eget dui quis, auctor maximus nibh.\n\
Suspendisse maximus sem arcu.\n\
Pellentesque sit amet semper est.\n\
Cras pulvinar ut tellus a semper.\n\
In facilisis tellus odio, non porta nisl accumsan nec.\n\
Pellentesque sollicitudin quam ac felis congue, ac congue enim tempor.",
                )
                .build(),
        )
        .option(version_opt)
        .callback(|_ctx| Ok(()))
        .command(sync)
        .command(download)
        .command(auth)
        .command(config_cmd)
        .build();

    let mut cfg = RichHelpConfig::default();
    cfg.table_options.leading = 1;
    cfg.table_options.box_type = Some(SIMPLE);
    cfg.table_options.padding = (1, 1);
    cfg.table_options.row_styles = vec![Style::parse("bold").unwrap_or_default(), Style::default()];

    cfg.table_commands.show_lines = true;
    cfg.table_commands.pad_edge = true;
    cfg.table_commands.box_type = Some(DOUBLE);
    cfg.table_commands.border_style = Style::parse("red").unwrap_or_default();
    cfg.table_commands.row_styles = vec![
        Style::parse("magenta").unwrap_or_default(),
        Style::parse("yellow").unwrap_or_default(),
        Style::parse("cyan").unwrap_or_default(),
        Style::parse("green").unwrap_or_default(),
    ];

    let ctx = ContextBuilder::new().info_name("10_table_styles").build();
    println!("{}", cli.get_rich_help_with(&ctx, &cfg));
}
