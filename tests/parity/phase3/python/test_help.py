import os
import json
import rich_click as click
from click.testing import CliRunner
from contextlib import contextmanager


@contextmanager
def temp_env(vars):
    old = {}
    for k, v in vars.items():
        old[k] = os.environ.get(k)
        os.environ[k] = v
    try:
        yield
    finally:
        for k in vars:
            if old[k] is None:
                os.environ.pop(k, None)
            else:
                os.environ[k] = old[k]


def run_case(title, cli, env=None):
    runner = CliRunner()
    with temp_env(env or {}):
        result = runner.invoke(cli, ["--help"])
    print(f"=== {title} ===")
    print(result.output, end="")


@click.group()
def cli_panels():
    """Panel and table styling tests."""
    pass


@cli_panels.command()
@click.option("--alpha", metavar="TEXT", help="Alpha option")
@click.option("--beta", metavar="TEXT", help="Beta option")
def run(alpha, beta):
    """Run command"""
    pass


@cli_panels.command()
@click.option("--gamma", metavar="TEXT", help="Gamma option")
def stop(gamma):
    """Stop command"""
    pass


if __name__ == "__main__":
    cfg = {
        "panel_inline_help_in_title": True,
        "panel_inline_help_delimiter": " — ",
        "option_groups": {
            "*": [
                {
                    "name": "Core",
                    "options": ["--alpha", "--beta"],
                    "help": "Core options",
                    "inline_help_in_title": True,
                }
            ]
        },
        "command_groups": {
            "*": [
                {
                    "name": "Subcommands",
                    "commands": ["run", "stop"],
                    "help": "Task controls",
                    "inline_help_in_title": True,
                }
            ]
        },
        "options_table_column_types": ["opt_long", "metavar", "help"],
        "commands_table_column_types": ["name", "help"],
        "style_options_table_show_lines": True,
        "style_commands_table_show_lines": True,
    }
    run_case("panels", cli_panels, env={"RICH_CLICK_THEME": json.dumps(cfg)})
