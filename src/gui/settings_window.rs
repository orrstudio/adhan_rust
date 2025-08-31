use eframe::{egui, App, Frame};
use log::info;
use std::sync::mpsc::Sender;

use super::main_window::WindowCommand;
use crate::config::{MIN_OPACITY, MAX_OPACITY, DEFAULT_OPACITY};

pub struct SettingsWindow {
    command_sender: Option<Sender<WindowCommand>>,
    opacity: f32,
    initial_opacity: f32,
}

impl SettingsWindow {
    pub fn new() -> Self {
        Self {
            command_sender: None,
            opacity: DEFAULT_OPACITY,
            initial_opacity: DEFAULT_OPACITY,
        }
    }
    
    pub fn with_command_sender(mut self, sender: Sender<WindowCommand>) -> Self {
        self.command_sender = Some(sender);
        self
    }
    
    pub fn with_opacity(mut self, opacity: f32) -> Self {
        self.opacity = opacity;
        self.initial_opacity = opacity;
        self
    }

    fn show_settings_ui(&mut self, ui: &mut egui::Ui) -> bool {
        let mut settings_changed = false;
        
        egui::Grid::new("settings_grid")
            .num_columns(2)
            .spacing([20.0, 10.0])
            .striped(true)
            .show(ui, |ui| {
                // Настройка прозрачности
                ui.label("Прозрачность окна:");
                let response = ui.add(
                    egui::Slider::new(&mut self.opacity, MIN_OPACITY..=MAX_OPACITY)
                        .suffix("%")
                        .fixed_decimals(1)
                );
                
                if response.drag_stopped() || response.lost_focus() {
                    settings_changed = true;
                }
                
                ui.end_row();
            });
            
        settings_changed
    }
}

impl App for SettingsWindow {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Кнопка возврата в главное меню
            if ui.button("← Вернуться в главное меню").clicked() {
                info!("Кнопка 'Вернуться в главное меню' нажата");
                if let Some(sender) = &self.command_sender {
                    if let Err(e) = sender.send(WindowCommand::OpenMain) {
                        log::error!("Не удалось отправить команду: {}", e);
                    }
                }
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                return;
            }

            ui.separator();
            ui.heading("Настройки");
            
            // Прокручиваемая область для настроек
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.vertical_centered_justified(|ui| {
                    self.show_settings_ui(ui);
                });
            });
            
            // Кнопки управления
            ui.separator();
            
            ui.horizontal(|ui| {
                if ui.button("Отмена").clicked() {
                    // Восстанавливаем исходную прозрачность
                    self.opacity = self.initial_opacity;
                    // Закрываем окно настроек
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
                
                if ui.button("Применить").clicked() {
                    info!("Применена прозрачность: {}%", self.opacity * 100.0);
                    // Закрываем окно настроек
                    if let Some(sender) = &self.command_sender {
                        if let Err(e) = sender.send(WindowCommand::OpenMain) {
                            log::error!("Не удалось отправить команду: {}", e);
                        }
                    }
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });
        });
    }
}
