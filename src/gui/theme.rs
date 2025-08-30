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

/// Применяет выбранную тему к контексту
pub fn apply_theme(ctx: &egui::Context, theme: Theme) {
    let visuals = match theme {
        Theme::Dark => {
            let mut visuals = egui::Visuals::dark();
            let bg = crate::config::DARK_BACKGROUND;
            
            // Основные цвета для темной темы
            visuals.panel_fill = Color32::from_rgba_premultiplied(40, 40, 45, 255);   // Чуть светлее для панелей
            visuals.faint_bg_color = Color32::from_rgba_premultiplied(25, 25, 30, 255); // Еще темнее для фона
            
            // Цвета текста для темной темы
            visuals.override_text_color = Some(Color32::from_rgb(240, 240, 240)); // Почти белый текст
            
            // Дополнительные настройки
            visuals.code_bg_color = Color32::from_rgba_premultiplied(45, 45, 50, 255); // Цвет фона для кода
            
            visuals
        },
        Theme::Light => {
            let mut visuals = egui::Visuals::light();
            let bg = crate::config::LIGHT_BACKGROUND;
            
            // Основные цвета для светлой темы
            visuals.panel_fill = Color32::from_rgb(250, 250, 250);   // Почти белый для панелей
            visuals.faint_bg_color = Color32::from_rgb(240, 240, 240); // Светло-серый для фона
            
            // Цвета текста для светлой темы
            visuals.override_text_color = Some(Color32::from_rgb(0, 0, 0)); // Черный текст
           
            // Дополнительные настройки
            visuals.code_bg_color = Color32::from_rgb(245, 245, 245);
            
            visuals
        },
    };
    
    ctx.set_visuals(visuals);
}

