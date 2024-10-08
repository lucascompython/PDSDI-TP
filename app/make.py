#!/usr/bin/env python3
import sys
import os

if "app" in os.getcwd():  # Make frontend module always available
    sys.path.append("..")

from frontend import make as frontend_make


def main() -> None:
    print("Hello, World from app!")
    frontend_make.main()


if __name__ == "__main__":
    main()
