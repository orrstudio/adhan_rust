use eframe::egui;
use crate::gui::{window, Theme};

#[derive(Default)]
pub struct App {
    name: String,
    pub current_theme: Theme,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Меню с выбором темы
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                ui.heading("Adhan Rust");
                
                ui.separator();
                
                ui.label("Тема:");
                for theme in [Theme::Dark, Theme::Light] {
                    let theme_name = match theme {
                        Theme::Dark => "Темная",
                        Theme::Light => "Светлая",
                    };
                    
                    if ui.selectable_label(self.current_theme == theme, theme_name).clicked() {
                        self.current_theme = theme;
                        crate::gui::apply_theme(ctx, theme);
                    }
                }
            });
        });

        // Основное содержимое
        egui::CentralPanel::default()
            .frame(window::get_panel_frame())
            .show(ctx, |ui| {
                ui.heading("Привет из Adhan Rust!");
                
                ui.horizontal(|ui| {
                    ui.label("Введите ваше имя: ");
                    ui.text_edit_singleline(&mut self.name);
                });
                
                if ui.button("Поприветствовать").clicked() {
                    println!("Привет, {}!", self.name);
                }
                
                ui.label(format!("Привет, {}! 👋", self.name));
            });
    }
}
