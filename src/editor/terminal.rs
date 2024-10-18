use std::io::{stdout, Error, Write};
use crossterm::cursor::{Hide, MoveDown, MoveLeft, MoveRight, MoveTo, MoveUp, Show};
use crossterm::{queue, Command};
use core::fmt::Display;
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType, size};

#[derive(Clone, Copy)]
pub struct Size {
    pub height: usize,
    pub width: usize,
}

#[derive(Clone, Copy)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}


/// Represents the terminal
/// Edge Case for platform where `usize` < `u16`:
/// Regardless of the actual size of the Terminal, this presentation only
/// spans over at most `usize::Max` or `u16::Max` rows and columns, whichever is samller.
/// Each size returned truncates to min(`usize::Max`, `u16::Max`) rows and columns.
/// And should you attempt to set the cursor out of these bounds, it will also be truncated.
pub struct Terminal {}

impl Terminal {
    pub fn initialize() -> Result<(), Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_cursor_to(Position { x: 0, y: 0 })?;
        Self::execute()?;
        Ok(())
    }

    pub fn terminate() -> Result<(), Error> {
        disable_raw_mode()?;
        Ok(())
    }

    pub fn clear_screen() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::All))?;
        Ok(())
    }

    pub fn clear_line() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::CurrentLine))?;
        Ok(())
    }

    /// Move the cursor to the specified position
    /// # Arguments
    /// * `position` - The position to move the cursor to. Will be truncated to `u16::MAX` if bigger.
    pub fn move_cursor_to(position: Position) -> Result<(), Error> {
        #[allow(clippy::as_conversions, clippy::cast_possible_truncation)]
        Self::queue_command(MoveTo(position.x as u16, position.y as u16))?;
        Ok(())
    }

    pub fn move_cursor_to_right() -> Result<(), Error> {
        Self::queue_command(MoveRight(1))?;
        Ok(())
    }

    pub fn move_cursor_to_left() -> Result<(), Error> {
        Self::queue_command(MoveLeft(1))?;
        Ok(())
    }

    pub fn move_cursor_to_down() -> Result<(), Error> {
        Self::queue_command(MoveDown(1))?;
        Ok(())
    }

    pub fn move_cursor_to_up() -> Result<(), Error> {
        Self::queue_command(MoveUp(1))?;
        Ok(())
    }

    pub fn hide_cursor() -> Result<(), Error> {
        Self::queue_command(Hide)?;
        Ok(())
    }

    pub fn show_cursor() -> Result<(), Error> {
        Self::queue_command(Show)?;
        Ok(())
    }

    pub fn execute() -> Result<(), Error> {
        stdout().flush()?;
        Ok(())
    }

    /// Returns the size of the terminal
    /// Edge Case for platform where `usize` < `u16`:
    /// * A `Size` representing the terminal size. Any coordinate `z` truncated to `usize` if `usize` < `u16`.
    pub fn size() -> Result<Size, Error> {
        let (width, height) = size()?;
        
        #[allow(clippy::as_conversions)]
        let width = width as usize;

        #[allow(clippy::as_conversions)]
        let height = height as usize;

        Ok(Size { height, width })
    }

    pub fn print<T: Display>(string: T) -> Result<(), Error> {
        Self::queue_command(Print(string))?;
        Ok(())
    }

    fn queue_command<T: Command>(command: T) -> Result<(), Error> {
        queue!(stdout(), command)?;
        Ok(())
    }
}