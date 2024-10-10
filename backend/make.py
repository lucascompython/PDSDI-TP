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

from utils import Colors, run_command


@dataclass
class Args:
    dev: bool
    release: bool
    clean: bool
    run: bool


def _clean() -> None:
    Colors.info(f"Cleaning {Colors.UNDERLINE}target{Colors.RESET} directory")

    start = perf_counter()
    run_command(("cargo", "clean"), cwd=CWD)
    elapsed = perf_counter() - start

    Colors.success(
        f"Cleaned {Colors.UNDERLINE}target{Colors.RESET} directory in {elapsed:.2f} seconds"
    )


def _dev() -> None:
    Colors.info("Running the development server")

    run_command(("cargo", "watch", "-x", "run"), cwd=CWD)


def _release() -> None:

    Colors.info("Building the release version")

    start = perf_counter()

    run_command(("cargo", "build", "--release"), cwd=CWD)

    elapsed = perf_counter() - start

    Colors.success(f"Built the release version in {elapsed:.2f} seconds")


def main(args: Args) -> None:
    print("Hello, World from backend!")

    if args.clean:
        _clean()

    if args.dev:
        _dev()
    elif args.release:
        _release()

    if args.run:
        run_command(("./target/release/backend"), cwd=CWD)


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
        "--run",
        action="store_true",
        help="Run the release application",
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
