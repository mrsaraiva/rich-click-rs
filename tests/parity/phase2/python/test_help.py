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


@click.command()
@click.option("--count", type=click.INT, default=1, show_default=True, help="Number of greetings")
@click.argument("name")
def cli_markdown(count, name):
    """Markdown **bold** and `code`."""
    pass


@click.command()
@click.option("--spark", is_flag=True, help="Emit :sparkles:")
def cli_markup(spark):
    """Markup [bold]bold[/] and :sparkles:."""
    pass


@click.command()
@click.option("--mode", default="fast", show_default=True, help="Select the execution mode for the operation")
def cli_width(mode):
    """A very long description that should wrap when constrained by width."""
    pass


@click.group()
def cli_alias():
    """Alias test."""
    pass


@cli_alias.command(aliases=["run"])
def start():
    """Start command"""
    pass


if __name__ == "__main__":
    run_case(
        "markdown",
        cli_markdown,
        env={"RICH_CLICK_THEME": json.dumps({"text_markup": "markdown"})},
    )
    run_case(
        "markup",
        cli_markup,
        env={"RICH_CLICK_THEME": json.dumps({"text_markup": "rich", "text_emojis": False})},
    )
    run_case(
        "width",
        cli_width,
        env={"RICH_CLICK_THEME": json.dumps({"width": 40, "max_width": 40})},
    )
    run_case(
        "aliases",
        cli_alias,
        env={"RICH_CLICK_THEME": json.dumps({"helptext_show_aliases": False})},
    )
