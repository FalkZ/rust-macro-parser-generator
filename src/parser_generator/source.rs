use std::iter::Peekable;

use super::position::Position;



pub struct Source {
   position: Position,
   chars: Peekable<std::vec::IntoIter<char>>
}



impl Source {
    pub fn new(source: &str) -> Self {
        let v: Vec<char> = source.chars().collect();
        let v: std::vec::IntoIter<char> = v.into_iter();
     
        Self { chars: v.into_iter().peekable(), position: Position::default() }
    }

    pub fn peek(&mut self) -> Option<char> {
        self.chars.peek().copied()
    }

    pub fn get_position(&self) -> Position {
        self.position.clone()
    }
}

impl Iterator for Source {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.chars.next()?;

        if n == '\n' {
            self.position.line += 1;
            self.position.column = 0;
        } else {
            self.position.column += 1;
        }

        Some(n)
    }
}

