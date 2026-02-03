import rich_click as click
from click.testing import CliRunner

@click.command()
@click.option("--count", default=1, show_default=True, help="Number of greetings")
@click.option("--verbose/--no-verbose", default=False, help="Verbose output")
@click.argument("name")
def cli(count, verbose, name):
    """Say hello"""
    pass

if __name__ == "__main__":
    runner = CliRunner()
    result = runner.invoke(cli, ["--help"])
    print(result.output, end="")
