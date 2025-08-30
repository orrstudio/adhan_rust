//! Основной модуль графического интерфейса приложения

mod theme;
mod window;

use eframe::egui;
use crate::config::*;
use crate::config::{DEFAULT_OPACITY, MIN_OPACITY, MAX_OPACITY};

/// Главное приложение
pub struct AdhanApp {
    /// Текущая тема (темная/светлая)
    theme: theme::Theme,
    /// Текущая прозрачность (0.0 - прозрачное, 1.0 - непрозрачное)
    opacity: f32,
}

impl Default for AdhanApp {
    fn default() -> Self {
        Self {
            theme: theme::Theme::Dark, // По умолчанию темная тема
            opacity: DEFAULT_OPACITY,  // По умолчанию полная непрозрачность
        }
    }
}

impl eframe::App for AdhanApp {
    /// Обновление интерфейса
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Применяем выбранную тему и прозрачность
        theme::apply_theme(ctx, self.theme, self.opacity);

        // Верхняя панель с кнопкой смены темы
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading(WINDOW_TITLE);
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let theme_text = match self.theme {
                        theme::Theme::Dark => "🌞 Light",
                        theme::Theme::Light => "🌙 Dark",
                    };
                    
                    if ui.button(theme_text).clicked() {
                        self.theme = match self.theme {
                            theme::Theme::Dark => theme::Theme::Light,
                            theme::Theme::Light => theme::Theme::Dark,
                        };
                    }
                    
                    // Ползунок для настройки прозрачности
                    ui.separator();
                    ui.horizontal(|ui| {
                        ui.label("Прозрачность:");
                        ui.add(
                            egui::Slider::new(&mut self.opacity, MIN_OPACITY..=MAX_OPACITY)
                                .step_by(0.05)
                                .text("")
                                .min_decimals(1)
                                .max_decimals(2)
                        );
                        ui.label(format!("{:.0}%", self.opacity * 100.0));
                    });
                });
            });
        });

        // Основная область отрисовки
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.separator();
                
                // Здесь будет основной контент
                ui.label("Время намазов будет отображаться здесь");
            });
        });
    }
}

/// Запуск приложения
pub fn run() -> Result<(), eframe::Error> {
    // Настройки окна
    let window_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([WINDOW_WIDTH, WINDOW_HEIGHT])
            .with_min_inner_size([MIN_WINDOW_WIDTH, MIN_WINDOW_HEIGHT])
            .with_transparent(true)
            .with_decorations(true), 
        ..Default::default()
    };

    // Запуск приложения
    eframe::run_native(
        WINDOW_TITLE,
        window_options,
        Box::new(|_cc| Ok(Box::<AdhanApp>::default())),
    )
}