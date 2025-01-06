# Instructions

Install the rust std for nightly compiler by running:

```bash
rustup component add rust-std --toolchain nightly
```

## Android

Install the android rust toolchain and the nightly version by running:

```bash
rustup target add aarch64-linux-android 
rustup target add aaarch64-linux-android --toolchain nightly
```

Then follow the following the [tauri android setup](https://tauri.app/start/prerequisites/#android)
