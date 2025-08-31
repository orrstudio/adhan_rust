//! Основной модуль графического интерфейса приложения

mod main_window;
mod settings_window;

use std::sync::mpsc::{self, Sender};
use log::info;

pub use main_window::{MainWindow, WindowCommand};
pub use settings_window::SettingsWindow;

use eframe::egui;
use crate::config::{
    MAIN_WINDOW_TITLE, MAIN_WINDOW_WIDTH, MAIN_WINDOW_HEIGHT, 
    MIN_MAIN_WINDOW_WIDTH, MIN_MAIN_WINDOW_HEIGHT,
    SETTINGS_WINDOW_TITLE, SETTINGS_WINDOW_WIDTH, SETTINGS_WINDOW_HEIGHT,
    MIN_SETTINGS_WINDOW_WIDTH, MIN_SETTINGS_WINDOW_HEIGHT,
    DEFAULT_OPACITY
};

/// Запуск главного окна приложения
pub fn run() -> eframe::Result<()> {
    info!("Запуск приложения");
    
    // Создаем канал для передачи команд между окнами
    let (tx, rx) = mpsc::channel();
    
    // Начинаем с главного окна
    let mut current_window = WindowCommand::OpenMain;
    let mut opacity = DEFAULT_OPACITY;
    
    loop {
        match current_window {
            WindowCommand::OpenMain => {
                info!("Открываем главное окно");
                
                // Настройки окна
                let mut window_options = eframe::NativeOptions::default();
                window_options.viewport = egui::ViewportBuilder::default()
                    .with_inner_size([MAIN_WINDOW_WIDTH, MAIN_WINDOW_HEIGHT])
                    .with_min_inner_size([MIN_MAIN_WINDOW_WIDTH, MIN_MAIN_WINDOW_HEIGHT])
                    .with_transparent(true)
                    .with_decorations(true)
                    .with_window_level(egui::viewport::WindowLevel::AlwaysOnBottom);
                
                // Клонируем отправитель для использования в замыкании
                let tx_clone = tx.clone();
                
                // Создаем главное окно
                let main_window = MainWindow::new()
                    .with_command_sender(tx_clone);
                
                // Запускаем главное окно
                let result = eframe::run_native(
                    MAIN_WINDOW_TITLE,
                    window_options,
                    Box::new(move |_cc| {
                        Ok(Box::new(main_window) as Box<dyn eframe::App>)
                    }),
                );
                
                if let Err(e) = result {
                    log::error!("Ошибка при запуске главного окна: {}", e);
                    break;
                }
                
                // Проверяем, не пришла ли команда переключения окна
                if let Ok(cmd) = rx.try_recv() {
                    current_window = cmd;
                    continue;
                }
                
                break;
            }
            WindowCommand::OpenSettings => {
                info!("Открываем окно настроек");
                
                // Настройки окна
                let mut window_options = eframe::NativeOptions::default();
                window_options.viewport = egui::ViewportBuilder::default()
                    .with_inner_size([SETTINGS_WINDOW_WIDTH, SETTINGS_WINDOW_HEIGHT])
                    .with_min_inner_size([MIN_SETTINGS_WINDOW_WIDTH, MIN_SETTINGS_WINDOW_HEIGHT])
                    .with_transparent(true)
                    .with_decorations(true);
                
                // Клонируем отправитель для использования в замыкании
                let tx_clone = tx.clone();
                
                // Создаем окно настроек с текущей прозрачностью
                let settings_window = SettingsWindow::new()
                    .with_command_sender(tx_clone)
                    .with_opacity(opacity);
                
                // Запускаем окно настроек
                let result = eframe::run_native(
                    SETTINGS_WINDOW_TITLE,
                    window_options,
                    Box::new(move |_cc| {
                        Ok(Box::new(settings_window) as Box<dyn eframe::App>)
                    }),
                );
                
                if let Err(e) = result {
                    log::error!("Ошибка при запуске окна настроек: {}", e);
                    break;
                }
                
                // Проверяем, не пришла ли команда переключения окна или изменения прозрачности
                if let Ok(cmd) = rx.try_recv() {
                    match cmd {
                        _ => current_window = cmd,
                    }
                    continue;
                }
                
                break;
            }
            WindowCommand::Exit => {
                info!("Завершение работы приложения");
                break;
            }
        }
    }
    
    Ok(())
}