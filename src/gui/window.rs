use eframe::egui;
use crate::config::{WINDOW_TITLE, WINDOW_WIDTH, WINDOW_HEIGHT, MIN_WINDOW_WIDTH, MIN_WINDOW_HEIGHT, WINDOW_OPACITY};
use egui::ViewportBuilder;
use egui::epaint::Margin;

pub fn create_window() -> eframe::Result<()> {
    let viewport = ViewportBuilder::default()
        .with_inner_size(egui::vec2(WINDOW_WIDTH, WINDOW_HEIGHT))
        .with_min_inner_size(egui::vec2(MIN_WINDOW_WIDTH, MIN_WINDOW_HEIGHT))
        .with_transparent(true)
        .with_decorations(true);

    let options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };

    eframe::run_native(
        WINDOW_TITLE,
        options,
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
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
        // Настраиваем стиль с полупрозрачным черным фоном
        let mut visuals = egui::Visuals::dark();
        // Используем значение прозрачности из конфига
        let semi_transparent_black = egui::Color32::from_rgba_premultiplied(0, 0, 0, WINDOW_OPACITY);
        visuals.window_fill = semi_transparent_black;
        visuals.panel_fill = semi_transparent_black;
        visuals.faint_bg_color = semi_transparent_black;
        ctx.set_visuals(visuals);

        // Устанавливаем полупрозрачный черный фон для центральной панели
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE
                .fill(egui::Color32::from_rgba_premultiplied(0, 0, 0, WINDOW_OPACITY))
                .outer_margin(Margin::ZERO)
                .inner_margin(Margin::ZERO))
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