use crossterm::event::{read, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers, Event};
use std::io::stdout;

mod terminal;
use terminal::Terminal;

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub const fn default() -> Self {
        Self { should_quit: false }
    }

    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    fn draw_rows() -> Result<(), std::io::Error> {
        let (_, row) = Terminal::size()?;
        for current_row in 0..row {
            crossterm::cursor::MoveTo(0, current_row);
            println!("~");
        }

        Ok(())
    }


    fn repl(&mut self) -> Result<(), std::io::Error> {
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
            let event = read()?;
            self.evalute_event(&event);
        }
        Ok(())
    }

    fn evalute_event(&mut self, event: &Event) {
        if let Key(KeyEvent { code, modifiers, ..}) = event {
            match code {
                Char('x') if *modifiers == KeyModifiers::CONTROL => self.should_quit = true,
                _ => (),
            }
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        if self.should_quit {
            Terminal::clear_screen()?;
            println!("Goodbye.");
        } else {
            Self::draw_rows()?;
            Terminal::move_cursor_to(0, 0)?;
        }

        Ok(())
    }
}
