use egui::Color32;

/// Доступные темы приложения
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Theme {
    Dark,
    Light,
}

impl Default for Theme {
    fn default() -> Self {
        Self::Dark
    }
}

/// Применяет выбранную тему к контексту с учетом прозрачности
pub fn apply_theme(ctx: &egui::Context, theme: Theme, opacity: f32) {
    // Применяем тему с учетом прозрачности
    let alpha = (opacity * 255.0) as u8;
    let visuals = match theme {
        Theme::Dark => {
            let mut visuals = egui::Visuals::dark();
            
            // Основные цвета для темной темы с прозрачностью
            visuals.panel_fill = Color32::from_rgba_premultiplied(0, 0, 0, alpha);   // Черный фон с прозрачностью
            visuals.faint_bg_color = Color32::from_rgba_premultiplied(0, 0, 0, alpha); // Черный фон с прозрачностью
            visuals.window_fill = Color32::from_rgba_premultiplied(0, 0, 0, alpha);   // Черный фон для окна с прозрачностью
            
            // Цвета текста для темной темы
            visuals.override_text_color = Some(Color32::from_rgb(240, 240, 240)); // Почти белый текст
            
            // Дополнительные настройки
            visuals.code_bg_color = Color32::from_rgba_premultiplied(20, 20, 25, 255); // Темно-серый для кода
            
            visuals
        },
        Theme::Light => {
            let mut visuals = egui::Visuals::light();
            
            // Основные цвета для светлой темы с прозрачностью
            visuals.panel_fill = Color32::from_rgba_premultiplied(255, 255, 255, alpha);   // Белый фон с прозрачностью
            visuals.faint_bg_color = Color32::from_rgba_premultiplied(255, 255, 255, alpha); // Белый фон с прозрачностью
            visuals.window_fill = Color32::from_rgba_premultiplied(255, 255, 255, alpha);   // Белый фон для окна с прозрачностью
            
            // Цвета текста для светлой темы
            visuals.override_text_color = Some(Color32::from_rgb(0, 0, 0)); // Черный текст
           
            // Дополнительные настройки
            visuals.code_bg_color = Color32::from_rgb(245, 245, 245); // Светло-серый для кода
            
            visuals
        },
    };
    
    ctx.set_visuals(visuals);
}

