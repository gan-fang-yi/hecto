use std::io::stdout;

use crossterm::{cursor::MoveTo, execute, terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType}};

pub struct Terminal {}

impl Terminal {
    pub fn initialize() -> std::io::Result<()> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_cursor_to(0, 0)?;
        Ok(())
    }

    pub fn terminate() -> std::io::Result<()> {
        disable_raw_mode()?;
        Ok(())
    }

    pub fn clear_screen() -> std::io::Result<()> {
        execute!(stdout(), Clear(ClearType::All))?;
        Ok(())
    }

    pub fn move_cursor_to(x: u16, y: u16) -> std::io::Result<()> {
        execute!(stdout(), MoveTo(x, y))?;
        Ok(())
    }

    pub fn size() -> std::io::Result<(u16, u16)> {
        crossterm::terminal::size()
    }
}