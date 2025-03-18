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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn push_test() {
        let mut forth = Forth::new(128);
        let value = 10;

        match forth.push(value) {
            Ok(result) => assert_eq!(result, value),
            Err(_error) => {}
        }
    }

    #[test]
    pub fn pop_test() {
        let mut forth = Forth::new(128);
        let value = 10;

        let _ = forth.push(value);

        match forth.pop() {
            Ok(result) => assert_eq!(result, value),
            Err(_error) => {}
        }
    }

    #[test]
    pub fn suma_test() {
        let mut forth = Forth::new(128);
        let a = 10;
        let b = 5;

        let _ = forth.push(a);
        let _ = forth.push(b);

        let _ = forth.suma();

        match forth.pop() {
            Ok(result) => assert_eq!(result, a + b),
            Err(_error) => {}
        }
    }

    #[test]
    pub fn resta_test() {
        let mut forth = Forth::new(128);
        let a = 10;
        let b = 5;

        let _ = forth.push(a);
        let _ = forth.push(b);

        let _ = forth.resta();

        match forth.pop() {
            Ok(result) => assert_eq!(result, b - a),
            Err(_error) => {}
        }
    }

    #[test]
    pub fn producto_test() {
        let mut forth = Forth::new(128);
        let a = 10;
        let b = 5;

        let _ = forth.push(a);
        let _ = forth.push(b);

        let _ = forth.producto();

        match forth.pop() {
            Ok(result) => assert_eq!(result, a * b),
            Err(_error) => {}
        }
    }

    #[test]
    pub fn division_test() {
        let mut forth = Forth::new(128);
        let a = 10;
        let b = 5;

        let _ = forth.push(a);
        let _ = forth.push(b);

        let _ = forth.division();

        match forth.pop() {
            Ok(result) => assert_eq!(result, b / a),
            Err(_error) => {}
        }
    }

    #[test]
    pub fn igual_test() {
        let mut forth = Forth::new(128);
        let a = 1;
        let b = 1;

        let _ = forth.push(a);
        let _ = forth.push(b);

        // a == b ?
        match forth.igual() {
            Ok(result) => assert_eq!(result, -1),
            Err(_) => (),
        }
    }

    #[test]
    pub fn mayor_test() {
        let mut forth = Forth::new(128);
        let a = 4;
        let b = 2;

        let _ = forth.push(a);
        let _ = forth.push(b);

        // b > a ?
        match forth.mayor() {
            Ok(result) => assert_eq!(result, 0),
            Err(_) => (),
        }
    }

    #[test]
    pub fn menor_test() {
        let mut forth = Forth::new(128);
        let a = 10;
        let b = 1;

        let _ = forth.push(a);
        let _ = forth.push(b);

        // b < a ?
        match forth.menor() {
            Ok(result) => assert_eq!(result, -1),
            Err(_) => (),
        }
    }

    #[test]
    pub fn and_test() {
        let mut forth = Forth::new(128);
        let a = 2;
        let b = 3;

        let _ = forth.push(a);
        let _ = forth.push(b);

        // 0010 && 0011 = 0010  
        match forth.and() {
            Ok(result) => assert_eq!(result, 2),
            Err(_) => (),
        }
    }

    #[test]
    pub fn or_test() {
        let mut forth = Forth::new(128);
        let a = 5;
        let b = 1;

        let _ = forth.push(a);
        let _ = forth.push(b);

        // 0101 || 0001 = 0101
        match forth.or() {
            Ok(result) => assert_eq!(result, 5),
            Err(_) => (),
        }
    }

    #[test]
    pub fn not_test() {
        let mut forth = Forth::new(128);
        let a = 5;

        let _ = forth.push(a);

        // not 0101 = 1010
        match forth.menor() {
            Ok(result) => assert_eq!(result, 10),
            Err(_) => (),
        }
    }

    #[test]
    pub fn dup_test() {
        let mut forth = Forth::new(128);
        let a = 10;

        let _ = forth.push(a);
        let _ = forth.dup();

        match forth.pop() {
            Ok(result) => assert_eq!(result, a),
            Err(_) => (),
        }
    }

    #[test]
    pub fn swap_test() {
        let mut forth = Forth::new(128);
        let a = 5;
        let b = 1;

        let _ = forth.push(a);
        let _ = forth.push(b);
        let _ = forth.rot();

        match forth.pop() {
            Ok(first) => assert_eq!(first, b),
            Err(_) => {},
        };

        match forth.pop() {
            Ok(second) => assert_eq!(second, a),
            Err(_) => {},
        }
    }

    #[test]
    pub fn over_test() {
        let mut forth = Forth::new(128);
        let a = 5;
        let b = 2;

        let _ = forth.push(a);
        let _ = forth.push(b);
        let _ = forth.over();

        match forth.pop() {
            Ok(expected) => assert_eq!(expected, a),
            Err(_) => {},
        };
    }

    #[test]
    pub fn rot_test() {
        let mut forth = Forth::new(128);
        let a = 5;
        let b = 2;
        let c = 3;

        let _ = forth.push(a);
        let _ = forth.push(b);
        let _ = forth.push(c);
        let _ = forth.rot();

        match forth.pop() {
            Ok(expected) => assert_eq!(expected, a),
            Err(_) => {},
        };

        match forth.pop() {
            Ok(expected) => assert_eq!(expected, c),
            Err(_) => {},            
        }

        match forth.pop() {
            Ok(expected) => assert_eq!(expected, b),
            Err(_) => {},            
        }
    }

    #[test]
    pub fn define_word_test() {
        let mut forth = Forth::new(128);
        let word_name = String::from("MAX");
        let word_body = String::from("OVER OVER < IF SWAP THEN DROP");

        match forth.define_word(word_name, word_body) {
            Ok(result) => assert_eq!(result, 1),
            Err(_error) => assert_eq!(true, false),
        }
    }
}