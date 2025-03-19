pub mod error;
pub mod forth;

use std::{
    fs::File,
    io::{BufRead, BufReader}
};

use forth::Forth;

use error::Error;

const DEFAULT_SIZE: usize = 128;

fn define_word(buf: &str) -> bool {
    buf.contains(':') && buf.contains(';')
}

fn parse_word(_forth: &mut Forth, buf: &mut str) -> Result<i16, Error> {
    let _array: Vec<&str> = buf.split_whitespace().collect();
    Ok(0)
}

fn is_numeric(buf: &str) -> (i16, bool) {
    match buf.parse::<i16>() {
        Ok(n) => (n, true), 
        Err(_) => (0, false),
    }
}

fn do_operation(forth: &mut Forth, buf: &str) -> Result<i16, Error> {
    match buf {
        "+" => forth.suma(),
        "-" => forth.resta(),
        "*" => forth.producto(),
        "/" => forth.division(),
        "DUP" => forth.dup(),
        "DROP" => forth.drop(),
        "SWAP" => forth.swap(),
        "OVER" => forth.over(),
        "ROT" => forth.rot(),
        "." => forth.print_stack(),
        "CR" => forth.cr(),
        "=" => forth.igual(),
        "<" => forth.menor(),
        ">" => forth.mayor(),
        "and" => forth.and(),
        "or" => forth.or(),
        "not" => forth.not(),
        &_ => Ok(0),
    }
}

fn procces_arguments(forth: &mut Forth, arg: &str) -> Result<i16, Error> {
    let (num, is_numeric) = is_numeric(arg);
    if is_numeric {
        forth.push(num)?;
    }

    do_operation(forth, arg)?;
    Ok(0)
}

    // 10 25 +
    // : MAX DROP IF > ;
fn procces_line(forth: &mut Forth, buf: &mut str) -> Result<i16, Error> {
    if define_word(buf) {
        // Definio la word => no hay mas que procesar en esta linea
        return parse_word(forth, buf);
    }

    let args: Vec<&str> = buf.split_whitespace().collect();
    for arg in args {
        procces_arguments(forth, arg)?;
    }
    
    Ok(0)
}

fn run(path: String, forth: &mut Forth) -> Result<i16, Error> {
    let file = match File::open(path) {
        Ok(file) => file,
        Err(_) => return Err(Error::FileOpenError),
    };
    let mut reader = BufReader::new(file);

    loop {
        let mut buf = String::new();
        let result = match reader.read_line(&mut buf) {
            Ok(result) => result,
            Err(_) => return Err(Error::FileReadError),
        };
        if result == 0 {
            break;
        }

        procces_line(forth, &mut buf)?;
    }

    Ok(0)
}

fn main() {
    // falta el parsing de comandos

    let path = String::from("data/par.fth");
    let stack_size = DEFAULT_SIZE;
    let mut forth = Forth::new(stack_size);

    match run(path, &mut forth) {
        Ok(_) => {},
        Err(error) => println!("{}", error),
    }
}
