use std::{
    cell::RefCell,
    fmt::{Debug, Display},
    rc::Rc,
};

use super::position::{GetPosition, Position};

#[derive(Debug, Clone)]

pub struct Pin<'a, T> {
    pinned_index: usize,
    tokens: &'a Tokens<T>,
}

impl<'a, T> Pin<'a, T> {
    pub fn get_pinned(&'a self) -> &'a Tokens<T> {
        let mut r = self.tokens.index.borrow_mut();

        *r = self.pinned_index;

        self.tokens
    }
}

#[derive(Default, Clone, Debug)]
pub struct Token {
    pub raw: String,
    pub position: Position,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.raw)
    }
}

impl Token {
    pub fn new(raw: String, position: Position) -> Self {
        Self { raw, position }
    }
    pub fn as_str<'a>(&'a self) -> &'a str {
        &self.raw
    }
}

#[derive(Debug, Clone)]
pub struct Tokens<T> {
    index: Rc<RefCell<usize>>,
    tokens: Vec<T>,
}

impl<T: GetPosition> Tokens<T> {
    pub fn new(tokens: Vec<T>) -> Self {
        Self {
            index: Rc::new(RefCell::new(0)),
            tokens,
        }
    }

    pub fn pin(&self) -> Pin<T> {
        Pin {
            pinned_index: self.index(),
            tokens: self,
        }
    }

    fn index(&self) -> usize {
        let reference = self.index.borrow();
        *reference
    }

    pub fn next(&self) -> Option<&T> {
        let next = self.tokens.get(self.index()).clone();

        let mut reference = self.index.borrow_mut();
        *reference += 1;

        return next;
    }

    pub fn peek(&self) -> Option<&T> {
        let next = self.tokens.get(self.index()).clone();

        return next;
    }

    pub fn position(&self) -> Option<Position> {
        let t = &self.peek()?;

        Some(t.position().clone())
    }
}

impl<T: GetPosition + Display> Display for Tokens<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: String = self
            .tokens
            .iter()
            .enumerate()
            .map(|(i, t)| {
                if i == self.index() {
                    format!("|> {} <|", t)
                } else {
                    format!("{}", t)
                }
            })
            .collect::<Vec<String>>()
            .join(", ");

        write!(f, "{}", s)
    }
}

pub trait RawToken: Clone {
    fn raw_token(&self) -> Token;
    fn as_str(&self) -> String {
        let t = self.raw_token();

        t.raw
    }
}

impl<T: Into<Token> + Clone> RawToken for T {
    fn raw_token(&self) -> Token {
        self.clone().into()
    }
}
