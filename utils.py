import subprocess
import sys
from typing import Callable


class Colors:
    """ANSI color codes"""

    RED = "\033[91m"
    GREEN = "\033[92m"
    YELLOW = "\033[93m"
    BLUE = "\033[94m"
    MAGENTA = "\033[95m"
    CYAN = "\033[96m"
    WHITE = "\033[97m"
    BOLD = "\033[1m"
    UNDERLINE = "\033[4m"
    ITALIC = "\033[3m"
    RESET = "\033[0m"

    @staticmethod
    def info(string: str) -> None:
        print(f"{Colors.CYAN}INFO:{Colors.RESET} {string}")

    @staticmethod
    def warning(string: str) -> None:
        print(f"{Colors.YELLOW}WARNING:{Colors.RESET} {string}")

    @staticmethod
    def error(string: str) -> None:
        sys.stderr.write(f"{Colors.RED}ERROR:{Colors.RESET} {string}\n")

    @staticmethod
    def success(string: str) -> None:
        print(f"{Colors.GREEN}SUCCESS:{Colors.RESET} {string}")


def use_run_command(cwd: str) -> Callable[[tuple[str]], None]:
    def run_command(command: tuple[str], **kwargs) -> None:
        try:
            subprocess.run(command, **kwargs, check=True, cwd=cwd)
        except (FileNotFoundError, subprocess.CalledProcessError) as e:
            Colors.error(e)
            sys.exit(1)

    return run_command
