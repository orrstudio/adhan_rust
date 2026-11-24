// Логгер приложения на базе tracing
// Письмо в папку logs/ с ротацией

use tracing_subscriber::{fmt, EnvFilter, prelude::*};
use std::fs;

pub fn init(log_dir: &str, file_logging: bool) {
    // Создаём папку логов, если её нет
    let _ = fs::create_dir_all(log_dir);

    let fmt_layer = fmt::layer()
        .with_file(true)
        .with_line_number(true)
        .with_target(false);

    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    let subscriber = tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt_layer);

    if file_logging {
        // В будущем здесь будет запись в файл с ротацией
        tracing::info!("Логирование в файл включено, директория: {}", log_dir);
    } else {
        tracing::info!("Логирование в файл выключено, только консоль");
    }

    let _ = subscriber.init();
}
