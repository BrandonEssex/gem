mod app;
mod action;
mod config;
mod input;
mod tag;
mod timer;
mod ui;
mod storage;

fn main() {
    if let Err(e) = gemx::run() {
        eprintln!("Application error: {}", e);
    }
}
