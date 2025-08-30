//! Конфигурация приложения

use egui;

/// Заголовок окна
pub const WINDOW_TITLE: &str = "Adhan";

// ===== РАЗМЕРЫ ОКНА =====
/// Ширина окна по умолчанию
pub const WINDOW_WIDTH: f32 = 400.0;
/// Высота окна по умолчанию
pub const WINDOW_HEIGHT: f32 = 300.0;

/// Минимальная ширина окна
pub const MIN_WINDOW_WIDTH: f32 = 300.0;
/// Минимальная высота окна
pub const MIN_WINDOW_HEIGHT: f32 = 200.0;

// ===== ПРОЗРАЧНОСТЬ =====
/// Прозрачность окна (0 - полностью прозрачное, 255 - полностью непрозрачное)
pub const WINDOW_OPACITY: u8 = 200;

// ===== ЦВЕТА =====
/// Основной цвет фона (черный с прозрачностью)
pub const BACKGROUND_COLOR: [u8; 4] = [0, 0, 0, WINDOW_OPACITY];

// ===== ОТСТУПЫ =====
/// Внутренние отступы (padding) элементов интерфейса
pub const PADDING: f32 = 8.0;

/// Внешние отступы (margin) элементов интерфейса
pub const MARGIN: f32 = 0.0;

/// Отступы по умолчанию (все стороны одинаковые)
pub const MARGIN_ZERO: egui::Margin = egui::Margin::ZERO;