#!/usr/bin/env python3
import os
import sys
from dataclasses import dataclass
from time import perf_counter

if "app" in os.getcwd():  # Make utils module always available
    sys.path.append("..")
    CWD = "."
else:
    CWD = "app"

from utils import Colors, run_command

OS = sys.platform
match OS:
    case "win32":
        APP_NAME = "clothe_match.exe"
        TARGET = "x86_64-pc-windows-msvc"
    case "linux":
        APP_NAME = "clothe_match"
        TARGET = "x86_64-unknown-linux-gnu"
    case _:
        Colors.error(f"Unsupported OS: {OS}")
        sys.exit(1)


android_home = os.path.expanduser("~/Android/Sdk")
ndk_directories = os.listdir(os.path.join(android_home, "ndk"))
ndk_home = (
    os.path.join(android_home, "ndk", ndk_directories[0]) if ndk_directories else ""
)
os.environ["ANDROID_HOME"] = android_home
os.environ["NDK_HOME"] = ndk_home


@dataclass
class Args:
    dev: bool
    release: bool
    mobile: bool
    clean: bool
    upx: bool
    nightly: bool
    native: bool
    build_frontend: bool
    run: bool
    smallest: bool


def _clean() -> None:
    Colors.info(
        f"Cleaning {Colors.UNDERLINE}dist{Colors.RESET} and {Colors.UNDERLINE}node_modules{Colors.RESET} directories"
    )
    start = perf_counter()
    run_command(("cargo", "clean"), cwd=CWD)
    elapsed = perf_counter() - start

    Colors.success(
        f"Cleaned {Colors.UNDERLINE}target{Colors.RESET} directory in {elapsed:.2f} seconds"
    )


def _dev(mobile: bool) -> None:
    Colors.info("Running the development build")
    if mobile:
        run_command(("cargo", "tauri", "android", "dev"), cwd=CWD)
    else:
        run_command(("cargo", "tauri", "dev"), cwd=CWD)


def _get_size() -> str:
    try:
        size = os.path.getsize(f"target/{TARGET}/release/{APP_NAME}")
        return f"{size / 1024:.2f} KB"
    except FileNotFoundError:
        Colors.warning(f"Binary not found at target/{TARGET}/release/{APP_NAME}")
        return "N/A"


def _release(args: Args) -> None:
    Colors.info("Building in release mode")
    start = perf_counter()

    command = [
        "cargo",
        "tauri",
        "build",
        "--target",
        TARGET[:7] if args.mobile else TARGET,
    ]
    rustflags = []

    if args.mobile:
        command.insert(2, "android")

    if args.nightly:
        command.insert(1, "+nightly")
        rustflags.extend(["-Zlocation-detail=none", "-Zfmt-debug=none"])

        command.extend(
            [
                "--",
                "-Z",
                "build-std=std,panic_abort",
                "-Z",
                "build-std-features='optimize-for_size'",
                "-Z",
                "build-std-features=panic_immediate_abort",
                "-Z",
                "trim-paths",
            ]
        )

        if args.native:
            rustflags.append("-C target-cpu=native")

        env = os.environ.copy()
        run_command(
            command,
            cwd=CWD,
            env={**env, "RUSTFLAGS": " ".join(rustflags)},
        )

        if args.upx:
            upx_start = perf_counter()
            Colors.info("Compressing the binary with UPX")

            run_command(("upx", "--ultra-brute", f"target/{TARGET}/release/{APP_NAME}"))

            upx_elapsed = perf_counter() - upx_start

            Colors.success(f"Compressed the binary in {upx_elapsed:.2f} seconds")

    elapsed = perf_counter() - start
    size = _get_size()
    Colors.success(f"Built the release version in {elapsed:.2f} seconds")
    Colors.success(f"Binary size: {size}")


def main(args: Args) -> None:
    global TARGET
    if args.mobile:
        TARGET = "aarch64-linux-android"

    if args.smallest:
        args.release = True
        args.upx = True
        args.nightly = True
        args.dev = False

    if args.clean:
        _clean()

    if args.dev:
        _dev(args.mobile)

    if args.build_frontend:
        from frontend import make as frontend_make

        frontend_make.main()

    if args.release:
        _release(args)

    if args.run:
        run_command((f"target/{TARGET}/release/{APP_NAME}",), cwd=CWD)


def parse_args() -> Args:
    import argparse

    parser = argparse.ArgumentParser(description="App build script")

    group = parser.add_mutually_exclusive_group()

    group.add_argument(
        "-d",
        "--dev",
        action="store_true",
        help="Run in development mode",
    )
    group.add_argument(
        "-r", "--release", action="store_true", help="Build in release mode"
    )
    parser.add_argument(
        "-m",
        "--mobile",
        action="store_true",
        help="Build the mobile version (requires Android NDK and SDK)",
    )
    parser.add_argument(
        "-bf",
        "--build-frontend",
        action="store_true",
        help="Also build the frontend on release",
    )
    parser.add_argument(
        "-c",
        "--clean",
        action="store_true",
        help="Clean the target directory",
    )
    parser.add_argument(
        "-u",
        "--upx",
        action="store_true",
        help="Compress the binary with UPX",
    )
    parser.add_argument(
        "--run",
        action="store_true",
        help="Run the release application",
    )
    parser.add_argument(
        "-n",
        "--nightly",
        action="store_true",
        help="Build with nightly compiler for further optimizations",
    )
    parser.add_argument(
        "--native",
        action="store_true",
        help="Build with native CPU target (not recommended for distribution)",
    )
    parser.add_argument(
        "-s",
        "--smallest",
        action="store_true",
        help="Build with optimizations for size (enables release, nightly and upx)",
    )

    if len(sys.argv) == 1:
        parser.print_help(sys.stderr)
        sys.exit(1)

    return Args(**vars(parser.parse_args()))


if __name__ == "__main__":
    try:
        main(parse_args())
    except KeyboardInterrupt:
        print("Exiting...")
