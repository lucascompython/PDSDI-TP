#!/usr/bin/env python3

from app import make as app_make
from backend import make as backend_make
from frontend import make as frontend_make
from model import make as model_make

import concurrent.futures


def make_app() -> None:
    frontend_make.main()
    app_make.main(build_frontend=False)


def main() -> None:
    print("Hello from make.py!")

    with concurrent.futures.ThreadPoolExecutor() as executor:
        executor.submit(backend_make.main)
        executor.submit(model_make.main)
        executor.submit(make_app)
    print("teste")


if __name__ == "__main__":
    main()
