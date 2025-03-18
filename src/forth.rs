use std::{
    collections::HashMap,
    ops::{BitAnd, BitOr},
};

pub enum Error {
    StackUnderflow,
    StackOverflow,
    InvalidWord,
    DivisionByZero,
}

pub struct Forth {
    stack: Vec<i16>,
    stack_size: usize, // En Kb
    words: HashMap<String, String>,
}

impl Forth {
    pub fn new(size: usize) -> Forth {
        Forth {
            stack: Vec::<i16>::new(),
            stack_size: size,
            words: HashMap::<String, String>::new(),
        }
    }

    pub fn push(&mut self, value: i16) -> Result<i16, Error> {
        let size = (self.stack.len() + 1) * 2;

        if size > self.stack_size * 1024 {
            return Err(Error::StackOverflow);
        }

        self.stack.push(value);
        Ok(value)
    }

    pub fn pop(&mut self) -> Result<i16, Error> {
        match self.stack.pop() {
            Some(value) => Ok(value),
            None => Err(Error::StackUnderflow),
        }
    }

    pub fn suma(&mut self) -> Result<i16, Error> {
        let a = self.pop()?;
        let b = self.pop()?;

        self.push(a + b)?;
        Ok(a + b)
    }

    pub fn resta(&mut self) -> Result<i16, Error> {
        let a = self.pop()?;
        let b = self.pop()?;

        self.push(a - b)?;
        Ok(a - b)
    }

    pub fn producto(&mut self) -> Result<i16, Error> {
        let a = self.pop()?;
        let b = self.pop()?;

        self.push(a * b)?;
        Ok(a * b)
    }

    pub fn division(&mut self) -> Result<i16, Error> {
        let a = self.pop()?;
        let b = self.pop()?;

        if b == 0 {
            return Err(Error::DivisionByZero);
        }

        self.push(a / b)?;
        Ok(a / b)
    }

    pub fn igual(&mut self) -> Result<i16, Error> {
        let a = self.pop()?;
        let b = self.pop()?;

        let mut value = -1;
        if a != b {
            value = 0;
        }

        self.push(value)?;

        Ok(value)
    }

    pub fn mayor(&mut self) -> Result<i16, Error> {
        let a = self.pop()?;
        let b = self.pop()?;

        let mut value = -1;
        if a < b {
            value = 0;
        }

        self.push(value)?;

        Ok(value)
    }

    pub fn menor(&mut self) -> Result<i16, Error> {
        let a = self.pop()?;
        let b = self.pop()?;

        let mut value = -1;
        if a > b {
            value = 0;
        }

        self.push(value)?;

        Ok(value)
    }

    pub fn and(&mut self) -> Result<i16, Error> {
        let a = self.pop()?;
        let b = self.pop()?;

        let value = a.bitand(b);

        self.push(value)?;

        Ok(value)
    }

    pub fn or(&mut self) -> Result<i16, Error> {
        let a = self.pop()?;
        let b = self.pop()?;

        let value = a.bitor(b);

        self.push(value)?;

        Ok(value)
    }

    pub fn not(&mut self) -> Result<i16, Error> {
        let a = self.pop()?;

        let value = !a;

        self.push(value)?;

        Ok(value)
    }

    pub fn dup(&mut self) -> Result<i16, Error> {
        let value = self.pop()?;
        let copy = value;

        self.push(value)?;
        self.push(copy)?;

        Ok(value)
    }

    pub fn drop(&mut self) -> Result<i16, Error> {
        self.pop()
    }

    pub fn swap(&mut self) -> Result<i16, Error> {
        let a = self.pop()?;
        let b = self.pop()?;

        self.push(a)?;
        self.push(b)
    }

    pub fn over(&mut self) -> Result<i16, Error> {
        let a = self.pop()?;
        let b = self.pop()?;
        let c = b;

        self.push(b)?;
        self.push(a)?;
        self.push(c)
    }

    pub fn rot(&mut self) -> Result<i16, Error> {
        let a = self.pop()?;
        let b = self.pop()?;
        let c = self.pop()?;

        self.push(b)?;
        self.push(a)?;
        self.push(c)
    }

    pub fn print(&mut self) -> Result<i16, Error> {
        let top = self.pop()?;
        print!("{}", top);
        Ok(top)
    }

    pub fn cr(&mut self) {
        println!();
    }

    pub fn define_word(&mut self, word_name: String, word_body: String) -> Result<i16, Error> {
        match word_name.parse::<i16>() {
            Ok(_number) => Err(Error::InvalidWord),
            Err(_error) => {
                self.words.insert(word_name, word_body);
                Ok(1)
            }
        }
    }
}
