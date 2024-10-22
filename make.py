#!/usr/bin/env python3

import argparse
import concurrent.futures
import sys

from app import make as app_make
from backend import make as backend_make
from frontend import make as frontend_make
from model import make as model_make


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Build the entire project")
    parser.add_argument(
        "-c",
        "--clean",
        action="store_true",
        help="Clean the project",
    )

    parser.add_argument(
        "-d",
        "--dev",
        action="store_true",
        help="Run in development mode",
    )

    parser.add_argument(
        "-r",
        "--release",
        action="store_true",
        help="Build in release mode",
    )

    args = parser.parse_args()

    if len(sys.argv) == 1:
        parser.print_help()
        sys.exit(1)

    return args


def make_app(app_args: app_make.Args, frontend_args: frontend_make.Args) -> None:
    frontend_make.main(frontend_args)
    app_make.main(build_frontend=False)


def main() -> None:
    args = parse_args()
    frontend_args = frontend_make.Args(
        dev=args.dev, release=args.release, clean=args.clean
    )
    backend_args = backend_make.Args(
        dev=args.dev, release=args.release, clean=args.clean, run=False
    )
    app_args = app_make.Args(
        dev=args.dev,
        release=args.release,
        clean=args.clean,
        smallest=True,
        build_frontend=False,
        native=False,
    )

    with concurrent.futures.ThreadPoolExecutor() as executor:
        executor.submit(backend_make.main, backend_args)
        executor.submit(model_make.main)
        executor.submit(make_app, app_args, frontend_args)


if __name__ == "__main__":
    try:
        main()
    except KeyboardInterrupt:
        print("\nExiting...")
