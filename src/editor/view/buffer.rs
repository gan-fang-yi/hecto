use std::io::Error;
use std::fs::read_to_string;

#[derive(Default)]
pub struct Buffer {
    pub lines: Vec<String>,
}

impl Buffer {
    pub fn load(file_name: &str) -> Result<Self, Error> {
        let contents = read_to_string(file_name)?;
        let mut lines = Vec::new();
        for line in contents.split("\n") {
            lines.push(String::from(line));
        }

        Ok(Self { lines })
    }

    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }
}