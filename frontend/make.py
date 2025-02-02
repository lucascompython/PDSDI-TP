#!/usr/bin/env python3

import os
import sys
from dataclasses import dataclass
from shutil import rmtree
from time import perf_counter
from shutil import copyfile

cwd = os.getcwd()

if "frontend" in cwd:  # Make utils module always available
    sys.path.append("..")
    CWD = "."
elif "app" in cwd:
    sys.path.append("..")
    CWD = "../frontend"
else:
    CWD = "frontend"

from utils import Colors, use_run_command  # noqa: E402

run_command = use_run_command(CWD)


@dataclass
class Args:
    dev: bool
    release: bool
    clean: bool
    run: bool


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


def _copy_files() -> None:
    """This function copies all the pages to the /pt folder"""

    Colors.info("Copying files")
    start = perf_counter()

    entries = os.listdir(f"{CWD}/src/pages")
    for entry in entries:
        if os.path.isfile(f"{CWD}/src/pages/{entry}"):
            copyfile(f"{CWD}/src/pages/{entry}", f"{CWD}/src/pages/pt/{entry}")

    elapsed = perf_counter() - start
    Colors.success(f"Copied files in {elapsed:.2f} seconds")


def _dev() -> None:
    Colors.info("Running the development server")
    run_command(("wasm-pack", "build", "--target", "web"), cwd="../cbf")
    run_command(("bunx", "--bun", "astro", "dev"))


def _release() -> None:
    Colors.info("Building the release version")

    start = perf_counter()

    run_command(("wasm-pack", "build", "--target", "web"), cwd="../cbf")
    run_command(("bunx", "--bun", "astro", "build"))

    elapsed = perf_counter() - start

    Colors.success(
        f"Built the release version to {Colors.UNDERLINE}dist{Colors.RESET} in {elapsed:.2f} seconds"
    )


def _run() -> None:
    Colors.info("Running the production build")
    run_command(("bunx", "--bun", "astro", "preview", "--host"))


def main(args: Args) -> None:
    if args.clean:
        _clean()

    if args.dev:
        run_command(("bun", "install"))
        _copy_files()
        _dev()
    elif args.release:
        run_command(("bun", "install"))
        _copy_files()
        _release()

    if args.run:
        _run()


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
        help="Build in release mode",
    )
    parser.add_argument(
        "-c",
        "--clean",
        action="store_true",
        help="Clean the dist and node_modules directories",
    )
    parser.add_argument(
        "-R",
        "--run",
        action="store_true",
        help="Run the production build",
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
