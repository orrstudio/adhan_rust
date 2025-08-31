use eframe::{egui, App, Frame};
use log::info;
use std::sync::mpsc::Sender;

use super::main_window::WindowCommand;

pub struct SettingsWindow {
    command_sender: Option<Sender<WindowCommand>>,
}

impl SettingsWindow {
    pub fn new() -> Self {
        Self {
            command_sender: None,
        }
    }
    
    pub fn with_command_sender(mut self, sender: Sender<WindowCommand>) -> Self {
        self.command_sender = Some(sender);
        self
    }
}

impl App for SettingsWindow {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Окно настроек");

            if ui.button("Вернуться в главное меню").clicked() {
                info!("Кнопка 'Вернуться в главное меню' нажата");
                if let Some(sender) = &self.command_sender {
                    if let Err(e) = sender.send(WindowCommand::OpenMain) {
                        log::error!("Не удалось отправить команду: {}", e);
                    }
                }
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
        });
    }
}
