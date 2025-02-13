#!/usr/bin/env python3

import os
import sys
from dataclasses import dataclass
from time import perf_counter


if "backend" in os.getcwd():  # Make utils module always available
    sys.path.append("..")
    CWD = "."
else:
    CWD = "backend"

from utils import Colors, use_run_command

run_command = use_run_command(CWD)


@dataclass
class Args:
    dev: bool
    release: bool
    clean: bool
    run: bool
    test: bool


def _clean() -> None:
    Colors.info(f"Cleaning {Colors.UNDERLINE}target{Colors.RESET} directory")

    start = perf_counter()
    run_command(("cargo", "clean"))
    elapsed = perf_counter() - start

    Colors.success(
        f"Cleaned {Colors.UNDERLINE}target{Colors.RESET} directory in {elapsed:.2f} seconds"
    )


def _dev() -> None:
    Colors.info("Running the development server")

    run_command(
        ("cargo", "watch", "-x", "run", "--features", "log", "--ignore", "uploads/")
    )


def _release() -> None:

    Colors.info("Building the release version")

    start = perf_counter()

    run_command(("cargo", "build", "--release"))

    elapsed = perf_counter() - start

    Colors.success(f"Built the release version in {elapsed:.2f} seconds")


def _run() -> None:
    Colors.info("Running the release version")

    run_command(("./target/release/backend"))


def main(args: Args) -> None:

    if args.clean:
        _clean()

    if args.test:
        run_command(("cargo", "test"))

    if args.dev:
        _dev()
    elif args.release:
        _release()

    if args.run:
        _run()


def parse_args() -> Args:
    import argparse

    parser = argparse.ArgumentParser(description="Backend build script")

    group = parser.add_mutually_exclusive_group()

    group.add_argument(
        "-d",
        "--dev",
        action="store_true",
        help="Run in development mode",
    )
    group.add_argument(
        "-r",
        "--release",
        action="store_true",
        help="Build in release mode",
    )
    parser.add_argument(
        "-c",
        "--clean",
        action="store_true",
        help="Clean the target directory",
    )
    parser.add_argument(
        "-R",
        "--run",
        action="store_true",
        help="Run the release application",
    )
    parser.add_argument(
        "-t",
        "--test",
        action="store_true",
        help="Run tests",
    )

    if len(sys.argv) == 1:
        parser.print_help(sys.stderr)
        sys.exit(1)

    return Args(**vars(parser.parse_args()))


if __name__ == "__main__":
    try:
        main(parse_args())
    except KeyboardInterrupt:
        print("\nExiting...")
