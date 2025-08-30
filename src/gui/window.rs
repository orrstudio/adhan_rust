use eframe::egui;
use crate::config::{WINDOW_TITLE, WINDOW_WIDTH, WINDOW_HEIGHT, MIN_WINDOW_WIDTH, MIN_WINDOW_HEIGHT};

pub fn create_window() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(WINDOW_WIDTH, WINDOW_HEIGHT)),
        min_window_size: Some(egui::vec2(MIN_WINDOW_WIDTH, MIN_WINDOW_HEIGHT)),
        ..Default::default()
    };

    eframe::run_native(
        WINDOW_TITLE,
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}

struct MyApp {
    name: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Мир".to_owned(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Устанавливаем темную тему
        ctx.set_visuals(egui::Visuals::dark());

        // Устанавливаем черный фон для всего окна
        egui::CentralPanel::default()
            .frame(egui::Frame::none()
                .fill(egui::Color32::from_rgb(0, 0, 0))
                .outer_margin(egui::style::Margin::same(0.0)))
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