#!/usr/bin/env python3

import argparse
import concurrent.futures
import sys
from time import perf_counter

from app import make as app_make
from backend import make as backend_make
from frontend import make as frontend_make
from model import make as model_make
from utils import Colors


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Build the entire project")

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
        help="Clean the project",
    )

    parser.add_argument(
        "-R", "--run", action="store_true", help="Run the production builds"
    )

    parser.add_argument(
        "projects",
        nargs="+",
        choices=("frontend", "backend", "app", "model"),
        help="The projects to build",
    )

    args = parser.parse_args()

    if len(sys.argv) == 1:
        parser.print_help()
        sys.exit(1)

    return args


def dev(
    projects: tuple[str, ...],
    frontend_args: frontend_make.Args,
    backend_args: backend_make.Args,
    app_args: app_make.Args,
) -> None:
    with concurrent.futures.ThreadPoolExecutor() as executor:
        if "app" in projects:
            app_args.smallest = False
            executor.submit(app_make.main, app_args)
        elif "frontend" in projects:
            executor.submit(frontend_make.main, frontend_args)

        if "backend" in projects:
            executor.submit(backend_make.main, backend_args)

        Colors.info("Starting development builds...")


def make_app_release(
    app_args: app_make.Args, frontend_args: frontend_make.Args
) -> None:
    frontend_make.main(frontend_args)
    app_make.main(app_args)


def release(
    projects: tuple[str, ...],
    frontend_args: frontend_make.Args,
    backend_args: backend_make.Args,
    app_args: app_make.Args,
) -> None:
    start = perf_counter()
    with concurrent.futures.ThreadPoolExecutor() as executor:
        if "app" in projects and "frontend" in projects:
            executor.submit(make_app_release, app_args, frontend_args)
        elif "frontend" in projects:
            executor.submit(frontend_make.main, frontend_args)
        elif "app" in projects:
            executor.submit(app_make.main, app_args)

        if "backend" in projects:
            executor.submit(backend_make.main, backend_args)

        Colors.info("Starting release builds...")

    elapsed = perf_counter() - start
    Colors.success(
        f"Built the release version of {Colors.UNDERLINE}{', '.join(projects)}{Colors.RESET} in {elapsed:.2f} seconds"
    )


def run(
    projects: tuple[str, ...],
    frontend_args: frontend_make.Args,
    backend_args: backend_make.Args,
    app_args: app_make.Args,
) -> None:
    with concurrent.futures.ThreadPoolExecutor() as executor:
        if "app" in projects:
            app_args.smallest = False
            executor.submit(app_make.main, app_args)

        if "frontend" in projects:
            executor.submit(frontend_make.main, frontend_args)

        if "backend" in projects:
            executor.submit(backend_make.main, backend_args)

        Colors.info("Running projects...")


def clean(
    projects: tuple[str, ...],
    frontend_args: frontend_make.Args,
    backend_args: backend_make.Args,
    app_args: app_make.Args,
) -> None:
    start = perf_counter()
    with concurrent.futures.ThreadPoolExecutor() as executor:
        if "app" in projects:
            app_args.smallest = False
            executor.submit(app_make.main, app_args)
        if "frontend" in projects:
            executor.submit(frontend_make.main, frontend_args)
        if "backend" in projects:
            executor.submit(backend_make.main, backend_args)

    elapsed = perf_counter() - start

    Colors.success(
        f"Cleaned {Colors.UNDERLINE}{', '.join(projects)}{Colors.RESET} in {elapsed:.2f} seconds"
    )


def main() -> None:
    args = parse_args()
    frontend_args = frontend_make.Args(
        dev=args.dev, release=args.release, clean=args.clean, run=args.run
    )
    backend_args = backend_make.Args(
        dev=args.dev, release=args.release, clean=args.clean, run=args.run
    )
    app_args = app_make.Args(
        dev=args.dev,
        release=args.release,
        clean=args.clean,
        smallest=True,
        build_frontend=False,
        native=False,
        run=args.run,
        mobile=False,
        upx=False,
        nightly=False,
    )

    if args.clean:
        clean(args.projects, frontend_args, backend_args, app_args)

    if args.dev:
        dev(args.projects, frontend_args, backend_args, app_args)
    elif args.release:
        release(args.projects, frontend_args, backend_args, app_args)

    if args.run:
        run(args.projects, frontend_args, backend_args, app_args)


if __name__ == "__main__":
    try:
        main()
    except KeyboardInterrupt:
        print("\nExiting...")
