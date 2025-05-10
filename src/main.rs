mod app;
mod action;
mod config;
mod input;
mod tag;
mod timer;
mod ui;
mod storage;
mod plugin;
mod secure;

fn main() {
    if let Err(e) = app::run() {
        eprintln!("Application error: {}", e);
    }
}
