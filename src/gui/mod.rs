mod app;
pub mod theme;
pub mod window;

pub use window::create_window;
pub use theme::{apply_theme, Theme};

pub fn run() -> eframe::Result<()> {
    create_window()
}