mod window;

use anyhow::Result;

pub fn run() -> Result<()> {
    window::create_window()?;
    Ok(())
}