use crossterm::event::{read, Event::{self, Key}, KeyCode::{self, Char}, KeyEvent, KeyEventKind, KeyModifiers};
use std::io::Error;
use core::cmp::min;

mod terminal;
use terminal::{Terminal, Position, Size};

mod view;
use view::View;

#[derive(Clone, Copy, Default)]
struct Location {
    x: usize,
    y: usize,
}

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    location: Location,
    view: View,
}

impl Editor {
    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        self.handle_args();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    fn handle_args(&mut self) {
        let args: Vec<String> = std::env::args().collect();
        if let Some(file_name) = args.get(1) {
            self.view.load(file_name);
        }
    }

    fn repl(&mut self) -> Result<(), std::io::Error> {
        //self.view.render()?;
        Terminal::move_caret_to(Position {col: 0, row: 0})?;
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
            let event = read()?;
            self.evalute_event(&event)?;
        }
        Ok(())
    }

    fn move_point(&mut self, key_code: KeyCode) -> Result<(), Error> {
        let Location { mut x, mut y} = self.location;
        let Size {height, width} = Terminal::size()?;
        match key_code {
            KeyCode::Up => y = y.saturating_sub(1),
            KeyCode::Down => y = min(y.saturating_add(1), height.saturating_sub(1)),
            KeyCode::Left => x = x.saturating_sub(1),
            KeyCode::Right => x = min(x.saturating_add(1), width.saturating_sub(1)),
            KeyCode::PageUp => y = 0,
            KeyCode::PageDown => y = height.saturating_sub(1),
            KeyCode::Home => x = 0,
            KeyCode::End => x = width.saturating_sub(1),
            _ => (),
        }
        self.location = Location {x, y};
        Ok(())
    }

    fn evalute_event(&mut self, event: &Event) -> Result<(), Error>{
        if let Key(KeyEvent { 
            code,
            modifiers,
            kind: KeyEventKind::Press,
            ..
        }) = event {
            match code {
                Char('x') if *modifiers == KeyModifiers::CONTROL => self.should_quit = true,
                KeyCode::Up
                | KeyCode::Down
                | KeyCode::Left
                | KeyCode::Right
                | KeyCode::PageUp
                | KeyCode::PageDown
                | KeyCode::Home
                | KeyCode::End => self.move_point(*code)?,
                _ => (),
            }
        }
        Ok(())
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::hide_caret()?;
        Terminal::move_caret_to(Position::default())?;
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::move_caret_to(Position {row: 0, col: 0})?;
            Terminal::print("Goodbye.\r\n")?;
        } else {
            self.view.render()?;
            Terminal::move_caret_to(Position {
                col: self.location.x,
                row: self.location.y
            })?;       
        }
        Terminal::show_caret()?;
        Terminal::execute()?;

        Ok(())
    }

}
