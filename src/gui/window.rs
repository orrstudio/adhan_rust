use eframe::egui;
use egui::ViewportBuilder;
use crate::config::{
    WINDOW_TITLE, WINDOW_WIDTH, WINDOW_HEIGHT, 
    MIN_WINDOW_WIDTH, MIN_WINDOW_HEIGHT, 
    PADDING
};
use crate::gui::{app, apply_theme, Theme};

/// Получить настройки фрейма для панели
pub fn get_panel_frame() -> egui::Frame {
    // Используем прозрачный фон, чтобы тема управляла цветом
    let bg_color = egui::Color32::TRANSPARENT;
    
    // Устанавливаем только внутренние отступы (приводим f32 к i8)
    let padding = egui::Margin::symmetric(PADDING as i8, PADDING as i8);
    
    egui::Frame::NONE
        .fill(bg_color)
        .inner_margin(padding)
        .corner_radius(8.0)  // Закругляем углы
}

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
        Box::new(|cc| {
            // Создаем приложение
            let mut app = app::App::default();
            // Применяем тему по умолчанию
            apply_theme(&cc.egui_ctx, app.current_theme);
            Ok(Box::new(app))
        }),
    )
}