# dioxus-android-sample

This is a sample project to show how to use the Dioxus in an Android
application.

Basically, we are just using the wry template for cargo-mobile. However, we had
to use an unreleased version of wry, so forked dioxus is used.

## Run

First, you need to install Android Studio and cargo-mobile forked by tauri
(https://github.com/tauri-apps/cargo-mobile).

Then, run these commands:

```sh
git clone https://github.com/nazo6/dioxus-android-sample-project
cd dioxus-android-sample-project
cargo mobile init

# To run in device (you need to run emulator or connect device)
cargo android run

# To build apk
cargo android apk build
```