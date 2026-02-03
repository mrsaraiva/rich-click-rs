import os
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


def run_case(title, cli, args=None, env=None):
    if args is None:
        args = ["--help"]
    runner = CliRunner()
    with temp_env(env or {}):
        result = runner.invoke(cli, args)
    print(f"=== {title} ===")
    print(result.output, end="")


@click.command()
@click.option("--count", default=1, show_default=True, help="Number of greetings")
@click.option("--verbose/--no-verbose", default=False, help="Verbose output")
@click.argument("name")
def cli_basic(count, verbose, name):
    """Say hello"""
    pass


@click.command()
@click.option("--env", envvar="MY_ENV", show_envvar=True, help="Uses envvar")
@click.option("--req", required=True, help="Required value")
@click.option("--mode", default="fast", show_default=True, help="Mode")
@click.argument("path")
def cli_meta(env, req, mode, path):
    """Meta test"""
    pass


@click.group()
def group():
    """Group help"""
    pass


@group.command(aliases=["run"])
@click.option("--speed", default=1, show_default=True, help="Speed")
def start(speed):
    """Start command"""
    pass


if __name__ == "__main__":
    run_case("basic", cli_basic)
    run_case("slim-theme", cli_basic, env={"RICH_CLICK_THEME": "slim"})
    run_case("metadata", cli_meta)
    run_case("group-alias", group)
