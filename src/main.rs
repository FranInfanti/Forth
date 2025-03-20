pub mod error;
pub mod forth;

use error::Error;
use forth::Forth;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const DEFAULT_SIZE: usize = 128;

fn is_word(buf: &str) -> bool {
    buf.contains(':') && buf.contains(';')
}

fn define_word(forth: &mut Forth, buf: &str) -> Result<i16, Error> {
    let aux = buf
        .trim()
        .trim_matches([':', ';'])
        .trim_ascii_end()
        .trim_ascii_start();
    let args: Vec<&str> = aux.splitn(2, ' ').collect();

    forth.define_word(args[0].to_string(), args[1].to_string())
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
        "EMIT" => forth.emit(),
        "CR" => forth.cr(),
        "=" => forth.igual(),
        "<" => forth.menor(),
        ">" => forth.mayor(),
        "and" => forth.and(),
        "or" => forth.or(),
        "not" => forth.not(),
        &_ => Err(Error::MissingWord),
    }
}

fn process_conditional(
    forth: &mut Forth,
    args: &mut Vec<String>,
    mut i: usize,
) -> Result<i16, Error> {
    let result = forth.if_statement()?;
    args.remove(i);

    if result != 0 {
        while args[i].ne("ELSE") && args[i].ne("THEN") {
            i += 1;
        }

        while args[i].ne("THEN") {
            args.remove(i);
        }
        args.remove(i);
    } else {
        while args[i].ne("ELSE") && args[i].ne("THEN") {
            args.remove(i);
        }

        while args[i].ne("THEN") {
            i += 1;
        }
        args.remove(i);
    }

    Ok(0)
}

fn process_word(forth: &mut Forth, args: &mut Vec<String>, mut i: usize) -> Result<i16, Error> {
    let word_body: Vec<String> = match forth.get_word_body(&args[i]) {
        Ok(word_body) => word_body
            .split_whitespace()
            .map(|s| s.to_string())
            .collect(),
        Err(error) => return Err(error),
    };

    for word in word_body {
        args.insert(i + 1, word);
        i += 1;
    }

    Ok(0)
}

fn read_line(forth: &mut Forth, buf: &str) -> Result<i16, Error> {
    if is_word(buf) {
        return define_word(forth, buf);
    }

    let mut args: Vec<String> = buf.split_whitespace().map(|s| s.to_string()).collect();
    let mut i = 0;
    while i < args.len() {
        if forth.word_exists(&args[i]) {
            process_word(forth, &mut args, i)?;
            i += 1;
            continue;
        }

        if args[i].eq("IF") {
            process_conditional(forth, &mut args, i)?;
            continue;
        }

        if args[i].eq(".\"") {
            i += 1;

            let mut string = String::new();

            while !args[i].contains('\"') {
                string = format!("{} {}", string, args[i]);
                i += 1;
            }
            string = format!("{} {}", string, args[i]);
            i += 1;

            forth.print_string(string.trim_matches('\"').to_string());
            continue;
        }

        let (value, is_numeric) = is_numeric(&args[i]);
        if is_numeric {
            forth.push(value)?;
            i += 1;
            continue;
        }

        do_operation(forth, &args[i])?;

        i += 1;
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

        read_line(forth, &buf)?;
    }

    Ok(0)
}

fn main() {
    // falta el parsing de comandos

    let path = String::from("data/string.fth");
    let stack_size = DEFAULT_SIZE;
    let mut forth = Forth::new(stack_size);

    match run(path, &mut forth) {
        Ok(_) => {}
        Err(error) => println!("{}", error),
    }
}
