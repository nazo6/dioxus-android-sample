use anyhow::Result;
use dioxus::prelude::*;
#[cfg(target_os = "android")]
use wry::android_binding;
use wry::application::event_loop::EventLoop;

#[cfg(target_os = "android")]
fn init_logging() {
    android_logger::init_once(
        android_logger::Config::default()
            .with_min_level(log::Level::Trace)
            .with_tag("dioxus-android-sample"),
    );
}

#[cfg(not(target_os = "android"))]
fn init_logging() {
    env_logger::init();
}

fn stop_unwind<F: FnOnce() -> T, T>(f: F) -> T {
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)) {
        Ok(t) => t,
        Err(err) => {
            eprintln!("attempt to unwind out of `rust` with err: {:?}", err);
            std::process::abort()
        }
    }
}

fn _start_app() {
    stop_unwind(|| main().unwrap());
}

#[no_mangle]
#[inline(never)]
pub extern "C" fn start_app() {
    #[cfg(target_os = "android")]
    android_binding!(com_github_noreply_users, dioxus_1mobile_1test, _start_app);
    _start_app()
}

fn main() -> Result<()> {
    init_logging();
    let event_loop = EventLoop::new();

    dioxus_desktop::launch(app);

    Ok(())
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            "hello world!"
        }
    })
}
