use eframe::egui;

pub fn create_window() -> anyhow::Result<()> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(400.0, 300.0)),
        min_window_size: Some(egui::vec2(300.0, 200.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Adhan Rust",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )?;

    Ok(())
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
        egui::CentralPanel::default().show(ctx, |ui| {
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