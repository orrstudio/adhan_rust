use eframe::egui;
use crate::config::{
    WINDOW_TITLE, WINDOW_WIDTH, WINDOW_HEIGHT, 
    MIN_WINDOW_WIDTH, MIN_WINDOW_HEIGHT, 
    BACKGROUND_COLOR, MARGIN_ZERO
};
use egui::ViewportBuilder;
use crate::gui::app;

/// Настройки темы приложения
pub fn setup_theme(ctx: &egui::Context) {
    // Устанавливаем темную тему
    let mut visuals = egui::Visuals::dark();
    
    // Настраиваем цвета из конфигурации
    let bg_color = egui::Color32::from_rgba_premultiplied(
        BACKGROUND_COLOR[0], 
        BACKGROUND_COLOR[1], 
        BACKGROUND_COLOR[2], 
        BACKGROUND_COLOR[3]
    );
    
    visuals.window_fill = bg_color;
    visuals.panel_fill = bg_color;
    visuals.faint_bg_color = bg_color;
    
    ctx.set_visuals(visuals);
}

/// Получить настройки фрейма для панели
pub fn get_panel_frame() -> egui::Frame {
    let bg_color = egui::Color32::from_rgba_premultiplied(
        BACKGROUND_COLOR[0], 
        BACKGROUND_COLOR[1], 
        BACKGROUND_COLOR[2], 
        BACKGROUND_COLOR[3]
    );
    
    egui::Frame::NONE
        .fill(bg_color)
        .outer_margin(MARGIN_ZERO)
        .inner_margin(MARGIN_ZERO)
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
            // Настраиваем тему при инициализации
            setup_theme(&cc.egui_ctx);
            Ok(Box::new(app::App::default()))
        }),
    )
}