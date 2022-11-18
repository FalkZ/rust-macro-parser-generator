use std::{cell::RefCell, fmt::Display, rc::Rc};
use itertools::Itertools;

use crate::sourcemap::Pos;

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

#[derive(Debug, Clone)]
pub struct Tokens<T> {
    index: Rc<RefCell<usize>>,
    tokens: Vec<T>,
}

impl<T: Pos> Tokens<T> {
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

    pub fn position(&self)-> Option<crate::sourcemap::Position> {
        let t = &self.peek()?;
        
        Some(t.position().clone())
    }
}

impl<T: std::fmt::Debug + Pos> Display for Tokens<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: String = self.tokens.iter().enumerate().map(|(i, t)| {
            if i == self.index() {
                format!("|> {:?} <|", t)
            } else {
                format!("{:?}", t)
            }
        }).intersperse(", ".to_string()).collect();
        
        write!(f, "{}", s)
    }
}