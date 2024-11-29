# Things needed to build the project

- [python](https://www.python.org/downloads/) - used to make a small custom build system, was only tested with python 3.12 or higher
- [rust](https://www.rust-lang.org/tools/install) - was only tested with rust 1.81 or higher
- [bun](https://bun.sh/) - fast JS runtime
- [postgresql](https://www.postgresql.org/) - database, was only tested with postgresql 16
- [uv](https://github.com/astral-sh/uv) - modern python package manager  
- [cargo-watch](https://crates.io/crates/cargo-watch) - reload backend dev server on changes  
- [upx](https://upx.github.io/) - compress the app binary  
- [tauri-cli](https://v2.tauri.app/reference/cli/) - build the app - `cargo install tauri-cli`
- [android studio](https://developer.android.com/studio) - the make script is made assuming Android Studio was installed from [JetBrains Toolbox](https://www.jetbrains.com/toolbox-app/) and it was only tested on Linux. After installing Android Studio, install the following in the SDK Manager:
  - Android SDK Platform
  - Android SDK Platform-Tools
  - NDK (Side by side)
  - Android SDK Build-Tools
  - Android SDK Command-line Tools
- android target - `rustup target add aarch64-linux-android`
