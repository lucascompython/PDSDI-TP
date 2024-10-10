#!/usr/bin/env python3
import os
import sys


def main(build_frontend: bool = True) -> None:
    print("Hello, World from app!")
    if build_frontend:
        if "app" in os.getcwd():  # Make frontend module always available
            sys.path.append("..")

        from frontend import make as frontend_make

        frontend_make.main()


if __name__ == "__main__":
    try:
        main()
    except KeyboardInterrupt:
        print("Exiting...")
