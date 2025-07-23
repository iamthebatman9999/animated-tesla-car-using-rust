# Egui Tesla App

Tesla app implementation using Rust and [`egui`](https://github.com/emilk/egui).  
This project is an experiment to replicate the animations and interactions of the Tesla app originally built with Flutter (see references below).

⚠️ **Note:** The code is still in an early stage and contains several bugs. The main purpose of this project is to explore and learn about Rust + egui across different platforms, **especially as an exploration of using egui for animation needs**.

✅ Currently, the app runs fine on **Linux (Ubuntu)** and **Android**.  
⚠️ On **WASM**, animations are not working properly due to limitations in using `std::time` with WebAssembly.

# Demo

![Preview](/tesla-app.gif)

# Android

Built using [`xbuild`](https://github.com/rust-mobile/xbuild)

## Building

    x build --arch arm64 --platform android --release

## Running

    x run --device <DEVICE>

### Example

    $ x devices
    host                                              Linux               linux x64           Arch Linux 6.6.2-arch1-1
    adb:d535946                                       OnePlus5T           android arm64       Android 10 (API 29)
    $ x run --device adb:d535946


# WASM

## Building

    cargo build --target=wasm32-unknown-unknown

## Running

    trunk serve --port 9999 --release

⚠️ Note: Animations are currently broken on WASM because `std::time` is not fully supported in WebAssembly. Further investigation is needed (e.g., using wasm-bindgen or gloo_timers).

# Native

## Building

    cargo build

## Running

    cargo run --release


# References & Acknowledgements

This project was inspired by the work of [Abu Anwar](https://github.com/abuanwar072).

📹 Flutter Tesla App video tutorials:
- [Episode 1 - Flutter Pro Animation - Speed Code](https://www.youtube.com/watch?v=P629-Z3py1Y)
- [Episode 2 - Flutter Pro Animation - Speed Code](https://www.youtube.com/watch?v=0BSRmp1zE1Y)
- [Episode 3 - Flutter Pro Animation - Speed Code](https://www.youtube.com/watch?v=pJD1qmRjmBo)
- [Episode 4 - Flutter Pro Animation - Speed Code](https://www.youtube.com/watch?v=gTC4FMXRRFM)

💻 Flutter source code: [Animated-Tesla-Car-App-using-Flutter](https://github.com/abuanwar072/Animated-Tesla-Car-App-using-Flutter)

Some of the .svg files in this project were taken (with minor modifications) from the above repository.
Special thanks to Abu Anwar for providing such a great learning resource 🙏.

# Status

- ✅ Multi-platform build (Android, WASM, Native)
- ✅ Runs fine on Linux (Ubuntu) and Android
- ⚠️ WASM: animations are not working properly (std::time issue)
- ⚠️ Animations and interactions are not yet complete
- 🐞 Several bugs remain to be fixed