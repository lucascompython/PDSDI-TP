#!/usr/bin/env python3

import os
import sys
from dataclasses import dataclass
from shutil import rmtree
from time import perf_counter

if "frontend" in os.getcwd():  # Make utils module always available
    sys.path.append("..")
    CWD = "."
else:
    CWD = "frontend"

from utils import Colors, run_command


@dataclass
class Args:
    dev: bool
    release: bool
    clean: bool


def _clean() -> None:
    Colors.info(
        f"Cleaning {Colors.UNDERLINE}dist{Colors.RESET} and {Colors.UNDERLINE}node_modules{Colors.RESET} directories"
    )

    start = perf_counter()
    rmtree("dist", ignore_errors=True)
    rmtree("node_modules", ignore_errors=True)
    elapsed = perf_counter() - start

    Colors.success(
        f"Cleaned {Colors.UNDERLINE}dist{Colors.RESET} and {Colors.UNDERLINE}node_modules{Colors.RESET} directories in {elapsed:.2f} seconds"
    )


def _dev() -> None:
    Colors.info("Running the development server")

    run_command(("bunx", "--bun", "astro", "dev"), cwd=CWD)


def _release() -> None:
    Colors.info("Building the release version")

    start = perf_counter()

    run_command(("bunx", "--bun", "astro", "build"), cwd=CWD)

    elapsed = perf_counter() - start

    Colors.success(
        f"Built the release version to {Colors.UNDERLINE}dist{Colors.RESET} in {elapsed:.2f} seconds"
    )


def main(args: Args) -> None:
    if args.clean:
        _clean()

    run_command(("bun", "install"), cwd=CWD)
    if args.dev:
        _dev()
    elif args.release:
        _release()


def parse_args() -> Args:
    import argparse

    parser = argparse.ArgumentParser(description="Frontend build script")

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
        help="Run in release mode",
    )
    parser.add_argument(
        "-c",
        "--clean",
        action="store_true",
        help="Clean the dist and node_modules directories",
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
