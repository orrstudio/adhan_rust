mod gui;
mod utils;
mod config;

fn main() {
    if let Err(e) = gui::run() {
        eprintln!("Ошибка: {}", e);
        std::process::exit(1);
    }
}