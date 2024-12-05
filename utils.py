import subprocess
import sys
from typing import Callable, Any


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


def use_run_command(cwd: str) -> Callable[[tuple[str, ...], Any], None]:
    def run_command(command: tuple[str, ...], **kwargs) -> None:
        try:
            master_fd = None
            if cwd == "." or cwd.startswith(".."):
                subprocess.run(command, **kwargs, check=True, cwd=cwd)
            else:
                import pty
                import os

                master_fd, slave_fd = pty.openpty()
                proc = subprocess.Popen(
                    command,
                    **kwargs,
                    cwd=cwd,
                    stdout=slave_fd,
                    stderr=slave_fd,
                    text=True,
                )
                os.close(slave_fd)

                while True:
                    try:
                        output = os.read(master_fd, 1024).decode()
                        if output == "" and proc.poll() is not None:
                            break
                        if output:
                            for line in output.splitlines():
                                print(f"[{cwd.upper()}]: {line}")
                    except OSError as e:
                        if e.errno == 5:
                            break
                        raise
                proc.wait()

        except (FileNotFoundError, subprocess.CalledProcessError) as e:
            if isinstance(e, subprocess.CalledProcessError) and (
                e.returncode == 2 or e.returncode == 130 or e.returncode == -2
            ):  # do not print error on SIGINT
                return
            Colors.error(f"{e}\nFailed to run command: {command}")
            sys.exit(1)
        finally:
            if master_fd:
                os.close(master_fd)

    return run_command
