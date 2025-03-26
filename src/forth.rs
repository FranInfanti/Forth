use crate::error;

use error::Error;
use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
};

#[derive(Debug)]
pub struct Forth {
    stack: Vec<i16>,
    stack_size: usize, // En Bytes
    words: HashMap<String, Vec<String>>,
}

fn open_file(path: &str) -> Result<File, Error> {
    match fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(path)
    {
        Ok(file) => Ok(file),
        Err(_) => Err(Error::FileOpenError),
    }
}

impl Forth {
    pub fn new(size: usize) -> Self {
        Forth {
            stack: Vec::<i16>::new(),
            stack_size: size,
            words: HashMap::<String, Vec<String>>::new(),
        }
    }

    pub fn push(&mut self, value: i16) -> Result<i16, Error> {
        let size = (self.stack.len() + 1) * 2;

        if self.stack_size < size {
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

        self.push(b - a)?;
        Ok(b - a)
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

        if a == 0 {
            return Err(Error::DivisionByZero);
        }

        self.push(b / a)?;
        Ok(b / a)
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

        let mut value = 0;
        if b > a {
            value = -1;
        }

        self.push(value)?;

        Ok(value)
    }

    pub fn menor(&mut self) -> Result<i16, Error> {
        let a = self.pop()?;
        let b = self.pop()?;

        let mut value = 0;
        if b < a {
            value = -1;
        }

        self.push(value)?;

        Ok(value)
    }

    pub fn and(&mut self) -> Result<i16, Error> {
        let a = self.pop()?;
        let b = self.pop()?;

        let mut value = -1;
        if a == 0 || b == 0 {
            value = 0;
        }

        self.push(value)?;
        Ok(value)
    }

    pub fn or(&mut self) -> Result<i16, Error> {
        let a = self.pop()?;
        let b = self.pop()?;

        let mut value = 0;
        if a != 0 || b != 0 {
            value = -1;
        }

        self.push(value)?;
        Ok(value)
    }

    pub fn not(&mut self) -> Result<i16, Error> {
        let a = self.pop()?;

        let mut value = -1;
        if a != 0 {
            value = 0;
        }

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

    pub fn print_stack(&mut self) -> Result<i16, Error> {
        let top = self.pop()?;
        print!("{} ", top);
        Ok(top)
    }

    pub fn emit(&mut self) -> Result<i16, Error> {
        let number = self.pop()?;
        match char::from_u32(number as u32) {
            Some(char) => print!("{} ", char),
            None => return Err(Error::ParsingError),
        };

        Ok(number)
    }

    pub fn cr(&self) -> Result<i16, Error> {
        println!();
        Ok(0)
    }

    pub fn print_string(&self, string: String) {
        print!("{} ", string);
    }

    pub fn if_statement(&mut self) -> Result<i16, Error> {
        let a = self.pop()?;

        if a != 0 { Ok(-1) } else { Ok(0) }
    }

    pub fn define_word(&mut self, word_name: String, word_body: String) -> Result<i16, Error> {
        if word_name.parse::<i16>().is_ok() {
            return Err(Error::InvalidWord);
        }

        match self.get_word_body(&word_name) {
            Ok(words_body) => {
                let mut vec = Vec::<String>::new();
                for words in words_body {
                    vec.push(words.to_string());
                }

                vec.push(word_body);
                self.words.insert(word_name, vec);
            }
            Err(_) => {
                let vec = vec![word_body];
                self.words.insert(word_name, vec);
            }
        };
        Ok(0)
    }

    pub fn get_word_body(&mut self, word_name: &String) -> Result<&Vec<String>, Error> {
        match self.words.get(word_name) {
            Some(word_body) => Ok(word_body),
            None => Err(Error::MissingWord),
        }
    }

    pub fn get_word_body_index(&mut self, word_name: &String) -> Result<usize, Error> {
        let words_body = self.get_word_body(word_name)?;

        Ok(words_body.len() - 1)
    }

    pub fn word_exists(&mut self, word_name: &str) -> bool {
        let aux: Vec<&str> = word_name.splitn(2, '=').collect();
        self.get_word_body(&aux[0].to_string()).is_ok()
    }

    pub fn write_stack(&mut self, path: &str) -> Result<i16, Error> {
        let mut file = open_file(path)?;

        let mut aux_stack = Vec::<i16>::new();
        while !self.stack.is_empty() {
            let value = self.pop()?;
            aux_stack.push(value);
        }

        while !aux_stack.is_empty() {
            let value = match aux_stack.pop() {
                Some(value) => value,
                None => return Err(Error::StackError),
            };

            let format = format!("{} ", value);
            match file.write_all(format.as_bytes()) {
                Ok(_) => continue,
                Err(_) => return Err(Error::FileWriteError),
            }
        }
        Ok(0)
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
        let a = 5;
        let b = 10;

        // 5 10 - => 5 - 10
        let _ = forth.push(a);
        let _ = forth.push(b);

        let _ = forth.resta();

        match forth.pop() {
            Ok(result) => assert_eq!(result, a - b),
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
            Ok(result) => assert_eq!(result, a / b),
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
        let a = 3;
        let b = 4;

        let _ = forth.push(a);
        let _ = forth.push(b);

        // 3 4 > => 3 > 4
        match forth.mayor() {
            Ok(result) => assert_eq!(result, 0),
            Err(_) => (),
        }
    }

    #[test]
    pub fn menor_test() {
        let mut forth = Forth::new(128);
        let a = 3;
        let b = 4;

        let _ = forth.push(a);
        let _ = forth.push(b);

        // 3 4 < => 3 < 4
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
            Err(_) => {}
        };

        match forth.pop() {
            Ok(second) => assert_eq!(second, a),
            Err(_) => {}
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
            Err(_) => {}
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
            Err(_) => {}
        };

        match forth.pop() {
            Ok(expected) => assert_eq!(expected, c),
            Err(_) => {}
        }

        match forth.pop() {
            Ok(expected) => assert_eq!(expected, b),
            Err(_) => {}
        }
    }
}
