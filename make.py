#!/usr/bin/env python3

from app import make as app_make
from backend import make as backend_make
from frontend import make as frontend_make
from model import make as model_make


def main() -> None:
    print("Hello from make.py!")
    app_make.main()
    backend_make.main()
    frontend_make.main()
    model_make.main()


if __name__ == "__main__":
    main()
