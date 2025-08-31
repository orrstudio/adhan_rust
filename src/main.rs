mod gui;
mod utils;
mod config;

fn main() {
    // Инициализируем логгер
    env_logger::init();
    log::info!("Запуск приложения");

    if let Err(e) = gui::run() {
        log::error!("Ошибка при выполнении приложения: {}", e);
        eprintln!("Ошибка: {}", e);
        std::process::exit(1);
    }
}