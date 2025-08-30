//! Конфигурация приложения

/// Заголовок окна
pub const WINDOW_TITLE: &str = "Adhan";

// ===== РАЗМЕРЫ ОКНА =====
/// Ширина окна по умолчанию
pub const WINDOW_WIDTH: f32 = 720.0;
/// Высота окна по умолчанию
pub const WINDOW_HEIGHT: f32 = 1000.0;

/// Минимальная ширина окна
pub const MIN_WINDOW_WIDTH: f32 = 400.0;
/// Минимальная высота окна
pub const MIN_WINDOW_HEIGHT: f32 = 560.0;

// ===== ПРОЗРАЧНОСТЬ =====
/// Прозрачность окна (0 - полностью прозрачное, 255 - полностью непрозрачное)
pub const WINDOW_OPACITY: u8 = 255;

// ===== ЦВЕТА =====
/// Основной цвет фона (черный с прозрачностью)
pub const DARK_BACKGROUND: [u8; 4] = [0, 0, 0, WINDOW_OPACITY];
/// Светлый цвет фона с прозрачностью
pub const LIGHT_BACKGROUND: [u8; 4] = [1, 1, 1, WINDOW_OPACITY];

// ===== ОТСТУПЫ =====
/// Внутренние отступы (padding) элементов интерфейса
pub const PADDING: f32 = 4.0;