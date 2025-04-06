use crate::{error, split};

use error::Error;
use std::{
    fs::{self, File},
    io::Write,
};

#[derive(Debug)]
/// Tipo para representar la estructura del interprete de Forth.
pub struct Forth {
    /// Stack de ejecución del interprete.
    stack: Vec<i16>,
    /// Tamaño del stack de ejecución, en Bytes.
    stack_size: usize,
    /// Representa las words definidas en el sistema.
    words: Vec<(String, String)>,
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
    /// Crea un struct Forth con un stack de size pasado por
    /// parametro.
    /// Retorna el struct Forth.
    ///
    /// # Errors
    ///
    pub fn new(size: usize) -> Self {
        Forth {
            stack: Vec::<i16>::new(),
            stack_size: size,
            words: Vec::new(),
        }
    }

    /// Añade un elemento de 16 bits al stack de ejecución.
    /// Retorna el elemento añadido.
    ///
    /// # Errors
    ///
    /// Returns [`StackOverflow`](Error::StackOverflow) si se sobrepasa
    /// el tamaño del stack.
    ///
    pub fn push(&mut self, value: i16) -> Result<i16, Error> {
        let size = (self.stack.len() + 1) * 2;

        if self.stack_size < size {
            return Err(Error::StackOverflow);
        }

        self.stack.push(value);
        Ok(value)
    }

    /// Toma y retorna el ultimo elemento del stack de ejecución.
    ///
    /// # Errors
    ///
    /// Returns [`StackUnderflow`](Error::StackUnderflow) si se intenta tomar
    /// el ultimo elemento de un stack vacio.
    ///
    pub fn pop(&mut self) -> Result<i16, Error> {
        match self.stack.pop() {
            Some(value) => Ok(value),
            None => Err(Error::StackUnderflow),
        }
    }

    /// Toma los dos ultimos elementos del stack de ejecución y los suma,
    /// añadiendo el resultado de la suma al stack de ejecución.
    /// Retorna el resultado de la suma.
    ///
    /// # Errors
    ///
    /// Returns [`StackOverflow`](Error::StackOverflow) si se intenta añadir
    /// un elemento a un stack completo.
    ///
    /// Returns [`StackUnderflow`](Error::StackUnderflow) si se intenta tomar
    /// un elemento de un stack vacio.
    ///
    pub fn add(&mut self) -> Result<i16, Error> {
        let a = self.pop()?;
        let b = self.pop()?;

        self.push(a + b)?;
        Ok(a + b)
    }

    /// Toma los dos ultimos elementos del stack de ejecución y los resta,
    /// añadiendo el resultado de la resta al stack de ejecución.
    /// Retorna el resultado de la resta.
    ///
    /// # Errors
    ///
    /// Returns [`StackOverflow`](Error::StackOverflow) si se intenta añadir
    /// un elemento a un stack completo.
    ///
    /// Returns [`StackUnderflow`](Error::StackUnderflow) si se intenta tomar
    /// un elemento de un stack vacio.
    ///
    pub fn sub(&mut self) -> Result<i16, Error> {
        let a = self.pop()?;
        let b = self.pop()?;

        self.push(b - a)?;
        Ok(b - a)
    }

    /// Toma los dos ultimos elementos del stack de ejecución y hace el
    /// producto entre ellos, añadiendo el resultado del producto al stack de ejecución.
    /// Retorna el resultado de la multiplicación.
    ///
    /// # Errors
    ///
    /// Returns [`StackOverflow`](Error::StackOverflow) si se intenta añadir
    /// un elemento a un stack completo.
    ///
    /// Returns [`StackUnderflow`](Error::StackUnderflow) si se intenta tomar
    /// un elemento de un stack vacio.
    ///
    pub fn mul(&mut self) -> Result<i16, Error> {
        let a = self.pop()?;
        let b = self.pop()?;

        self.push(a * b)?;
        Ok(a * b)
    }

    /// Toma los dos ultimos elementos del stack de ejecución y los divide,
    /// añadiendo el resultado de la división al stack de ejecución.
    /// Retorna el resultado de la división.
    ///
    /// # Errors
    ///
    /// Returns [`StackOverflow`](Error::StackOverflow) si se intenta añadir
    /// un elemento a un stack completo.
    ///
    /// Returns [`StackUnderflow`](Error::StackUnderflow) si se intenta tomar
    /// un elemento de un stack vacio.
    ///
    /// Returns [`DivisionByZero`](Error::DivisionByZero) si se intenta realizar
    /// una división por cero.
    ///
    pub fn div(&mut self) -> Result<i16, Error> {
        let a = self.pop()?;
        let b = self.pop()?;

        if a == 0 {
            return Err(Error::DivisionByZero);
        }

        self.push(b / a)?;
        Ok(b / a)
    }

    /// Toma los dos ultimos elementos del stack de ejecución y los compara
    /// para determinar si son iguales, añade al stack de ejecución -1 si es
    /// verdadero y 0 en caso contrario.
    /// Retorna el resultado de la comparación.
    ///
    /// # Errors
    ///
    /// Returns [`StackOverflow`](Error::StackOverflow) si se intenta añadir
    /// un elemento a un stack completo.
    ///
    /// Returns [`StackUnderflow`](Error::StackUnderflow) si se intenta tomar
    /// un elemento de un stack vacio.
    ///
    pub fn equal(&mut self) -> Result<i16, Error> {
        let a = self.pop()?;
        let b = self.pop()?;

        let mut value = -1;
        if a != b {
            value = 0;
        }

        self.push(value)?;

        Ok(value)
    }

    /// Toma los dos ultimos elementos del stack de ejecución y los compara
    /// para determinar si el segundo elemento tomado es mayor al primero,
    /// añade al stack de ejecución -1 si es verdadero y 0 en caso contrario.
    /// Retorna el resultado de la comparación.
    ///
    /// # Errors
    ///
    /// Returns [`StackOverflow`](Error::StackOverflow) si se intenta añadir
    /// un elemento a un stack completo.
    ///
    /// Returns [`StackUnderflow`](Error::StackUnderflow) si se intenta tomar
    /// un elemento de un stack vacio.
    ///
    pub fn greater(&mut self) -> Result<i16, Error> {
        let a = self.pop()?;
        let b = self.pop()?;

        let mut value = 0;
        if b > a {
            value = -1;
        }

        self.push(value)?;

        Ok(value)
    }

    /// Toma los dos ultimos elementos del stack de ejecución y los compara
    /// para determinar si el segundo elemento tomado es menor al primero,
    /// añade al stack de ejecución -1 si es verdadero y 0 en caso contrario.
    /// Retorna el resultado de la comparación.
    ///
    /// # Errors
    ///
    /// Returns [`StackOverflow`](Error::StackOverflow) si se intenta añadir
    /// un elemento a un stack completo.
    ///
    /// Returns [`StackUnderflow`](Error::StackUnderflow) si se intenta tomar
    /// un elemento de un stack vacio.
    ///
    pub fn lower(&mut self) -> Result<i16, Error> {
        let a = self.pop()?;
        let b = self.pop()?;

        let mut value = 0;
        if b < a {
            value = -1;
        }

        self.push(value)?;

        Ok(value)
    }

    /// Toma los dos ultimos elementos del stack de ejecución y aplica la
    /// operación "and", considerando que todo elemento distinto de cero es
    /// verdadero o -1 y todo elemento igual a cero es falso o 0.
    /// Retorna el resultado de la operación.
    ///
    /// # Errors
    ///
    /// Returns [`StackOverflow`](Error::StackOverflow) si se intenta añadir
    /// un elemento a un stack completo.
    ///
    /// Returns [`StackUnderflow`](Error::StackUnderflow) si se intenta tomar
    /// un elemento de un stack vacio.
    ///
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

    /// Toma los dos ultimos elementos del stack de ejecución y aplica la
    /// operación "or", considerando que todo elemento distinto de cero es
    /// verdadero o -1 y todo elemento igual a cero es falso o 0.
    /// Retorna el resultado de la operación.
    ///
    /// # Errors
    ///
    /// Returns [`StackOverflow`](Error::StackOverflow) si se intenta añadir
    /// un elemento a un stack completo.
    ///
    /// Returns [`StackUnderflow`](Error::StackUnderflow) si se intenta tomar
    /// un elemento de un stack vacio.
    ///
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

    /// Toma el ultimo elemento del stack de ejecución y aplica la
    /// operación "not", considerando que todo elemento distinto de cero es
    /// verdadero o -1 y todo elemento igual a cero es falso o 0.
    /// Retorna el resultado de la operación.
    ///
    /// # Errors
    ///
    /// Returns [`StackOverflow`](Error::StackOverflow) si se intenta añadir
    /// un elemento a un stack completo.
    ///
    /// Returns [`StackUnderflow`](Error::StackUnderflow) si se intenta tomar
    /// un elemento de un stack vacio.
    ///
    pub fn not(&mut self) -> Result<i16, Error> {
        let a = self.pop()?;

        let mut value = -1;
        if a != 0 {
            value = 0;
        }

        self.push(value)?;
        Ok(value)
    }

    /// Toma el ultimo elemento del stack y lo duplica, insertando tambien la
    /// copia en el stack de ejecución.
    /// Retorna el valor del elemento duplicado.
    ///
    /// # Errors
    ///
    /// Returns [`StackOverflow`](Error::StackOverflow) si se intenta añadir
    /// un elemento a un stack completo.
    ///
    /// Returns [`StackUnderflow`](Error::StackUnderflow) si se intenta tomar
    /// un elemento de un stack vacio.
    ///
    pub fn dup(&mut self) -> Result<i16, Error> {
        let value = self.pop()?;
        let copy = value;

        self.push(value)?;
        self.push(copy)?;

        Ok(value)
    }

    /// Toma y descarta el ultimo elemento del stack de ejecución.
    /// Retorna el valor del elemento descartado.
    ///
    /// # Errors
    ///
    /// Returns [`StackOverflow`](Error::StackOverflow) si se intenta añadir
    /// un elemento a un stack completo.
    ///
    /// Returns [`StackUnderflow`](Error::StackUnderflow) si se intenta tomar
    /// un elemento de un stack vacio.
    ///
    pub fn drop(&mut self) -> Result<i16, Error> {
        self.pop()
    }

    /// Intercambia la posición de los dos ultimos elementos del stack de
    /// ejecución.
    /// Retorna el valor del elemento que quedo en el tope del stack.
    ///
    /// # Errors
    ///
    /// Returns [`StackOverflow`](Error::StackOverflow) si se intenta añadir
    /// un elemento a un stack completo.
    ///
    /// Returns [`StackUnderflow`](Error::StackUnderflow) si se intenta tomar
    /// un elemento de un stack vacio.
    ///
    pub fn swap(&mut self) -> Result<i16, Error> {
        let a = self.pop()?;
        let b = self.pop()?;

        self.push(a)?;
        self.push(b)
    }

    /// Toma el segundo elemento del stack y lo dupica, insertando dicha copia
    /// en el ultimo lugar del stack. Todos los demas elementos conservan su
    /// posición actual.
    /// Retorna el valor del elemento que quedo en el tope del stack.
    ///
    /// # Errors
    ///
    /// Returns [`StackOverflow`](Error::StackOverflow) si se intenta añadir
    /// un elemento a un stack completo.
    ///
    /// Returns [`StackUnderflow`](Error::StackUnderflow) si se intenta tomar
    /// un elemento de un stack vacio.
    ///
    pub fn over(&mut self) -> Result<i16, Error> {
        let a = self.pop()?;
        let b = self.pop()?;
        let c = b;

        self.push(b)?;
        self.push(a)?;
        self.push(c)
    }

    /// Realiza un intercambio de posiciones entre los ultimos tres elementos
    /// del stack de ejecución.
    /// Retorna el valor del elemento que quedo en el tope del stack.
    ///
    /// # Errors
    ///
    /// Returns [`StackOverflow`](Error::StackOverflow) si se intenta añadir
    /// un elemento a un stack completo.
    ///
    /// Returns [`StackUnderflow`](Error::StackUnderflow) si se intenta tomar
    /// un elemento de un stack vacio.
    ///
    pub fn rot(&mut self) -> Result<i16, Error> {
        let a = self.pop()?;
        let b = self.pop()?;
        let c = self.pop()?;

        self.push(b)?;
        self.push(a)?;
        self.push(c)
    }

    /// Imprime por stdout el ultimo elemento del stack, consumiendo su valor.
    /// Retorna el elemento mostrado por el stdout.
    ///
    /// # Errors
    ///
    /// Returns [`StackOverflow`](Error::StackOverflow) si se intenta añadir
    /// un elemento a un stack completo.
    ///
    /// Returns [`StackUnderflow`](Error::StackUnderflow) si se intenta tomar
    /// un elemento de un stack vacio.
    ///
    pub fn print_stack(&mut self) -> Result<i16, Error> {
        let top = self.pop()?;
        print!("{top} ");
        Ok(top)
    }

    /// Imprime por stdout el ultimo elemento del stack, consumiendo su
    /// valor, pero en formato char.
    /// Retorna el valor del elemento mostrado por stdout en i16.
    ///
    /// # Errors
    ///
    /// Returns [`StackOverflow`](Error::StackOverflow) si se intenta añadir
    /// un elemento a un stack completo.
    ///
    /// Returns [`StackUnderflow`](Error::StackUnderflow) si se intenta tomar
    /// un elemento de un stack vacio.
    ///
    /// Returns [`ParsingError`](Error::ParsingError) si se intenta tomar
    /// un elemento de un stack vacio.
    ///
    pub fn emit(&mut self) -> Result<i16, Error> {
        let number = self.pop()?;
        match char::from_u32(number as u32) {
            Some(char) => print!("{char} "),
            None => return Err(Error::ParsingError),
        };

        Ok(number)
    }

    /// Imprime por stdout un salto de linea.
    /// Retorna 0.
    ///
    /// # Errors
    ///
    pub fn cr(&self) -> Result<i16, Error> {
        println!();
        Ok(0)
    }

    /// Imprime por stdout un string proporcionado como argumento.
    ///
    /// # Errors
    ///
    pub fn print_string(&self, string: String) {
        print!("{string} ");
    }

    /// Toma el ultimo elemento del stack y verifica si es verdadero o falso.
    /// Retorna el resultado del if.
    ///
    /// # Errors
    ///
    /// Returns [`StackUnderflow`](Error::StackUnderflow) si se intenta tomar
    /// un elemento de un stack vacio.
    ///
    pub fn if_statement(&mut self) -> Result<i16, Error> {
        let a = self.pop()?;

        if a != 0 { Ok(-1) } else { Ok(0) }
    }

    pub fn define_word(&mut self, word_name: String, word_body: String) -> Result<i16, Error> {
        if word_name.parse::<i16>().is_ok() {
            return Err(Error::InvalidWord);
        }

        self.words.insert(0, (word_name, word_body));

        Ok(0)
    }

    fn expand_body(&self, mut word_body: Vec<String>, index: usize) -> Result<Vec<String>, Error> {
        let mut j = 0;
        while j < word_body.len() {
            if !self.word_exists(&word_body[j]) {
                j += 1;
                continue;
            }

            let mut i = index;
            while i < self.words.len() {
                let (name, new_body) = &self.words[i];
                if name.eq(&word_body[j]) {
                    word_body.remove(j);
                    for body in split(new_body) {
                        word_body.insert(j, body);
                        j += 1;
                    }
                    break;
                }
                i += 1;
            }

            j += 1;
        }
        Ok(word_body)
    }

    pub fn get_word_body(&mut self, word_name: &String) -> Result<Vec<String>, Error> {
        let mut i = 0;
        while i < self.words.len() {
            let (name, body) = &self.words[i];
            if name.eq(word_name) {
                return self.expand_body(split(body), i + 1);
            }

            i += 1;
        }
        Err(Error::MissingWord)
    }

    /// Retorna si existe definido un word con el nombre word-name.
    ///
    /// # Errors
    ///
    pub fn word_exists(&self, word_name: &str) -> bool {
        let mut i = 0;
        while i < self.words.len() {
            let (name, _) = &self.words[i];
            if name.eq(word_name) {
                return true;
            }

            i += 1;
        }
        false
    }

    /// Escribe en un archivo, que se encuentra en la ruta pasada por parametro,
    /// el stack de ejecución restante.
    /// Retorna 0.
    ///
    /// # Errors
    ///
    /// Returns [`FileOpenError`](Error::FileOpenError) si se intenta abrir un
    /// y ocurre algun error.
    ///
    /// Returns [`StackUnderflow`](Error::StackUnderflow) si se intenta tomar
    /// un elemento de un stack vacio.
    ///
    /// Returns [`StackError`](Error::StackError) si ocurre algún error cuando se
    /// intenta acceder a un stack.
    ///
    /// Returns [`FileWriteError`](Error::FileWriteError) si ocurre algún error de
    /// escritura en un archivo.
    ///
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
    pub fn add_test() {
        let mut forth = Forth::new(128);
        let a = 10;
        let b = 5;

        let _ = forth.push(a);
        let _ = forth.push(b);

        let _ = forth.add();

        match forth.pop() {
            Ok(result) => assert_eq!(result, a + b),
            Err(_error) => {}
        }
    }

    #[test]
    pub fn sub_test() {
        let mut forth = Forth::new(128);
        let a = 5;
        let b = 10;

        // 5 10 - => 5 - 10
        let _ = forth.push(a);
        let _ = forth.push(b);

        let _ = forth.sub();

        match forth.pop() {
            Ok(result) => assert_eq!(result, a - b),
            Err(_error) => {}
        }
    }

    #[test]
    pub fn mul_test() {
        let mut forth = Forth::new(128);
        let a = 10;
        let b = 5;

        let _ = forth.push(a);
        let _ = forth.push(b);

        let _ = forth.mul();

        match forth.pop() {
            Ok(result) => assert_eq!(result, a * b),
            Err(_error) => {}
        }
    }

    #[test]
    pub fn div_test() {
        let mut forth = Forth::new(128);
        let a = 10;
        let b = 5;

        let _ = forth.push(a);
        let _ = forth.push(b);

        let _ = forth.div();

        match forth.pop() {
            Ok(result) => assert_eq!(result, a / b),
            Err(_error) => {}
        }
    }

    #[test]
    pub fn equal_test() {
        let mut forth = Forth::new(128);
        let a = 1;
        let b = 1;

        let _ = forth.push(a);
        let _ = forth.push(b);

        // a == b ?
        match forth.equal() {
            Ok(result) => assert_eq!(result, -1),
            Err(_) => (),
        }
    }

    #[test]
    pub fn greater_test() {
        let mut forth = Forth::new(128);
        let a = 3;
        let b = 4;

        let _ = forth.push(a);
        let _ = forth.push(b);

        // 3 4 > => 3 > 4
        match forth.greater() {
            Ok(result) => assert_eq!(result, 0),
            Err(_) => (),
        }
    }

    #[test]
    pub fn lower_test() {
        let mut forth = Forth::new(128);
        let a = 3;
        let b = 4;

        let _ = forth.push(a);
        let _ = forth.push(b);

        // 3 4 < => 3 < 4
        match forth.lower() {
            Ok(result) => assert_eq!(result, -1),
            Err(_) => (),
        }
    }

    #[test]
    pub fn and_test() {
        let mut forth = Forth::new(128);
        let a = 2;
        let b = 0;

        let _ = forth.push(a);
        let _ = forth.push(b);

        match forth.and() {
            Ok(result) => assert_eq!(result, 0),
            Err(_) => (),
        }
    }

    #[test]
    pub fn or_test() {
        let mut forth = Forth::new(128);
        let a = 5;
        let b = 0;

        let _ = forth.push(a);
        let _ = forth.push(b);

        match forth.or() {
            Ok(result) => assert_eq!(result, -1),
            Err(_) => (),
        }
    }

    #[test]
    pub fn not_test() {
        let mut forth = Forth::new(128);
        let a = 5;

        let _ = forth.push(a);

        match forth.not() {
            Ok(result) => assert_eq!(result, 0),
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
