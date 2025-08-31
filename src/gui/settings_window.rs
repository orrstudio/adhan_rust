use eframe::{egui, App, Frame};
use log::info;
use std::sync::mpsc::Sender;

use super::main_window::WindowCommand;

#[derive(Debug, Default)]
pub struct Settings {
    // Настройки прозрачности
    pub opacity: f32,
    // Пример других настроек (можно добавить больше)
    pub show_notifications: bool,
    pub enable_sound: bool,
    pub dark_mode: bool,
}

pub struct SettingsWindow {
    command_sender: Option<Sender<WindowCommand>>,
    settings: Settings,
}

impl SettingsWindow {
    pub fn new() -> Self {
        Self {
            command_sender: None,
            settings: Settings {
                opacity: 1.0,
                show_notifications: true,
                enable_sound: true,
                dark_mode: true,
            },
        }
    }
    
    pub fn with_command_sender(mut self, sender: Sender<WindowCommand>) -> Self {
        self.command_sender = Some(sender);
        self
    }

    fn show_settings_ui(&mut self, ui: &mut egui::Ui) {
        egui::Grid::new("settings_grid")
            .num_columns(2)
            .spacing([20.0, 10.0])
            .striped(true)
            .show(ui, |ui| {
                // Прозрачность
                ui.label("Прозрачность:");
                ui.add(egui::Slider::new(&mut self.settings.opacity, 0.1..=1.0).suffix(" %"));
                ui.end_row();

                // Уведомления
                ui.label("Показывать уведомления:");
                ui.checkbox(&mut self.settings.show_notifications, "");
                ui.end_row();

                // Звук
                ui.label("Включить звук:");
                ui.checkbox(&mut self.settings.enable_sound, "");
                ui.end_row();

                // Тёмная тема
                ui.label("Тёмная тема:");
                ui.checkbox(&mut self.settings.dark_mode, "");
                ui.end_row();
            });
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
            
            // Кнопка сохранения настроек
            ui.separator();
            if ui.button("Сохранить настройки").clicked() {
                // TODO: Реализовать сохранение настроек
                info!("Настройки сохранены: {:?}", self.settings);
            }
        });
    }
}
