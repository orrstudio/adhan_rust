mod app;
pub mod window;

pub use window::create_window;

pub fn run() -> eframe::Result<()> {
    create_window()
}