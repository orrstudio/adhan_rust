//! Основной модуль графического интерфейса приложения

mod main_window;
mod settings_window;

use std::sync::mpsc::{self, Sender};
use log::info;

pub use main_window::{MainWindow, WindowCommand};
pub use settings_window::SettingsWindow;

use eframe::egui;
use crate::config::{WINDOW_TITLE, WINDOW_WIDTH, WINDOW_HEIGHT, MIN_WINDOW_WIDTH, MIN_WINDOW_HEIGHT};

/// Запуск главного окна приложения
pub fn run() -> Result<(), eframe::Error> {
    info!("Запуск приложения");
    
    // Создаем канал для передачи команд между окнами
    let (tx, rx) = mpsc::channel();
    
    // Начинаем с главного окна
    let mut current_window = WindowCommand::OpenMain;
    
    loop {
        match current_window {
            WindowCommand::OpenMain => {
                info!("Открываем главное окно");
                
                // Настройки окна
                let window_options = eframe::NativeOptions {
                    viewport: egui::ViewportBuilder::default()
                        .with_inner_size([WINDOW_WIDTH, WINDOW_HEIGHT])
                        .with_min_inner_size([MIN_WINDOW_WIDTH, MIN_WINDOW_HEIGHT])
                        .with_transparent(true)
                        .with_decorations(true), 
                    ..Default::default()
                };
                
                let tx_clone = tx.clone();
                
                // Запускаем главное окно
                let result = eframe::run_native(
                    WINDOW_TITLE,
                    window_options,
                    Box::new(move |_cc| {
                        Ok(Box::new(MainWindow::new().with_command_sender(tx_clone)) as Box<dyn eframe::App>)
                    }),
                );
                
                // Проверяем, какое окно нужно открыть следующим
                if let Ok(cmd) = rx.try_recv() {
                    current_window = cmd;
                    continue;
                }
                
                // Если окно было закрыто и нет команд, завершаем приложение
                if result.is_ok() {
                    break;
                }
            }
            WindowCommand::OpenSettings => {
                info!("Открываем окно настроек");
                
                // Настройки окна
                let window_options = eframe::NativeOptions {
                    viewport: egui::ViewportBuilder::default()
                        .with_inner_size([WINDOW_WIDTH, WINDOW_HEIGHT])
                        .with_min_inner_size([MIN_WINDOW_WIDTH, MIN_WINDOW_HEIGHT])
                        .with_transparent(true)
                        .with_decorations(true), 
                    ..Default::default()
                };
                
                let tx_clone = tx.clone();
                
                // Запускаем окно настроек
                let result = eframe::run_native(
                    "Настройки",
                    window_options,
                    Box::new(move |_cc| {
                        Ok(Box::new(SettingsWindow::new().with_command_sender(tx_clone)) as Box<dyn eframe::App>)
                    }),
                );
                
                // Проверяем, какое окно нужно открыть следующим
                if let Ok(cmd) = rx.try_recv() {
                    current_window = cmd;
                    continue;
                }
                
                // Если окно было закрыто и нет команд, завершаем приложение
                if result.is_ok() {
                    break;
                }
            }
            WindowCommand::Exit => {
                info!("Завершение работы приложения");
                break;
            }
        }
    }
    
    Ok(())
}