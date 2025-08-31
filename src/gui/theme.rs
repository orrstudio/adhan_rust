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
    
    // Создаем копию стандартной темы
    let mut visuals = match theme {
        Theme::Dark => egui::Visuals::dark(),
        Theme::Light => egui::Visuals::light(),
    };
    
    // Применяем прозрачность только к фону
    match theme {
        Theme::Dark => {
            // Для темной темы - черный фон с прозрачностью
            visuals.panel_fill = Color32::from_rgba_premultiplied(0, 0, 0, alpha);
            visuals.window_fill = Color32::from_rgba_premultiplied(0, 0, 0, alpha);
            visuals.faint_bg_color = Color32::from_rgba_premultiplied(0, 0, 0, alpha);
            visuals.widgets.noninteractive.bg_fill = Color32::from_rgba_premultiplied(0, 0, 0, alpha);
            
            // Текст остается непрозрачным
            visuals.override_text_color = Some(Color32::from_rgb(240, 240, 240));
        },
        Theme::Light => {
            // Для светлой темы - светло-серый фон с прозрачностью
            let light_gray = Color32::from_rgb(240, 240, 240);
            visuals.panel_fill = light_gray.gamma_multiply(alpha as f32 / 255.0);
            visuals.window_fill = light_gray.gamma_multiply(alpha as f32 / 255.0);
            visuals.faint_bg_color = light_gray.gamma_multiply(alpha as f32 / 255.0);
            visuals.widgets.noninteractive.bg_fill = light_gray.gamma_multiply(alpha as f32 / 255.0);
            
            // Инвертируем цвет текста в зависимости от прозрачности
            let inverted_alpha = 255 - alpha;
            visuals.override_text_color = Some(Color32::from_rgba_premultiplied(inverted_alpha, inverted_alpha, inverted_alpha, 255));
        },
    };
    
    // Убираем тень у окна для лучшего визуального восприятия
    visuals.window_shadow.blur = 0;
    visuals.window_shadow.spread = 0;
    
    // Применяем визуальные настройки
    ctx.set_visuals(visuals);
}