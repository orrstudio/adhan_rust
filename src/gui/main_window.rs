use eframe::{egui, App, Frame};
use log::info;
use std::sync::mpsc::Sender;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WindowCommand {
    OpenSettings,
    OpenMain,
    Exit,
}

pub struct MainWindow {
    command_sender: Option<Sender<WindowCommand>>,
}

impl MainWindow {
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

impl App for MainWindow {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Главное окно");

            if ui.button("Настройки").clicked() {
                info!("Кнопка 'Настройки' нажата");
                if let Some(sender) = &self.command_sender {
                    if let Err(e) = sender.send(WindowCommand::OpenSettings) {
                        log::error!("Не удалось отправить команду: {}", e);
                    }
                }
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
        });
    }
}
