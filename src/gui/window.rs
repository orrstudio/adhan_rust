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
        // Настройка визуального стиля с максимально черным фоном (RGBA 0, 0, 0, 255)
        let mut visuals = egui::Visuals::dark();
        visuals.window_fill = egui::Color32::from_rgba_premultiplied(0, 0, 0, 200);
        ctx.set_visuals(visuals);

        egui::CentralPanel::default()
            .frame(egui::Frame::none())
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