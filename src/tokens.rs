use std::{rc::Rc, cell::RefCell};



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
    index:  Rc<RefCell<usize>>,
    tokens: Vec<T>,
}

impl<T> Tokens<T> {
    pub fn new(tokens: Vec<T>) -> Self {
        Self { index: Rc::new(RefCell::new(0)), 
            tokens }
    }

    pub fn pin(&self) -> Pin<T> {  
       Pin {
        pinned_index: self.index(),
        tokens: self
       }
    }

    fn index(&self) -> usize {
        let  reference = self.index.borrow();
        *reference
    }

    pub fn next(&self) -> Option<&T> {
        let next = self.tokens.get(self.index()).clone();
        
        let mut reference = self.index.borrow_mut();
        *reference += 1;
 
        return next
    }

    pub fn peek(&self) -> Option<&T> {
        let next = self.tokens.get(self.index()).clone();
 
        return next
    }
}

