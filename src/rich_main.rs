use click::command::Command;
use click::error::ClickError;
use click::group::Group;

use crate::{
    main_rich_command, main_rich_command_with_errors, main_rich_group, main_rich_group_with_errors,
    RichHelpConfig,
};

pub trait RichMainExt {
    fn main_rich(&self) -> Result<(), ClickError>;
    fn main_rich_with_errors(&self) -> Result<(), ClickError>;
}

impl RichMainExt for Command {
    fn main_rich(&self) -> Result<(), ClickError> {
        let args: Vec<String> = std::env::args().skip(1).collect();
        let cfg = RichHelpConfig::global().clone();
        main_rich_command(self, args, &cfg)
    }

    fn main_rich_with_errors(&self) -> Result<(), ClickError> {
        let args: Vec<String> = std::env::args().skip(1).collect();
        let cfg = RichHelpConfig::global().clone();
        main_rich_command_with_errors(self, args, &cfg)
    }
}

impl RichMainExt for Group {
    fn main_rich(&self) -> Result<(), ClickError> {
        let args: Vec<String> = std::env::args().skip(1).collect();
        let cfg = RichHelpConfig::global().clone();
        main_rich_group(self, args, &cfg)
    }

    fn main_rich_with_errors(&self) -> Result<(), ClickError> {
        let args: Vec<String> = std::env::args().skip(1).collect();
        let cfg = RichHelpConfig::global().clone();
        main_rich_group_with_errors(self, args, &cfg)
    }
}
