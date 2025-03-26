pub mod error;
pub mod forth;

use error::Error;
use forth::Forth;
use std::{
    env::args,
    fs::File,
    io::{BufRead, BufReader},
};

const FILE: &str = "stack.fth";
const DEFAULT_SIZE: usize = 128 * 1024; // En Bytes
const ARGV: usize = 2;

const IF: &str = "if";
const ELSE: &str = "else";
const THEN: &str = "then";
const START_STRING: &str = ".\"";
const START_WORD: char = ':';
const END_WORD: char = ';';

fn get_stack_size(env: &str) -> Result<usize, Error> {
    let chars: Vec<&str> = env.splitn(2, '=').collect();

    if chars.len() != 2 {
        return Err(Error::InvalidArguments);
    }

    match chars[1].parse::<usize>() {
        Ok(stack_size) => Ok(stack_size),
        Err(_) => Err(Error::ParsingError),
    }
}

fn parse_cmd_arguments(env: &mut Vec<String>) -> Result<(usize, &String), Error> {
    env.remove(0);

    if env.is_empty() || env.len() > ARGV {
        return Err(Error::InvalidAmountOfArguments);
    }

    let mut stack_size = DEFAULT_SIZE;
    if env.len() == ARGV {
        stack_size = get_stack_size(&env[1])?;
    }

    Ok((stack_size, &env[0]))
}

fn word_not_complete(buf: &str) -> bool {
    buf.contains(START_WORD) && !buf.contains(END_WORD)
}

fn parse_word(cmds: &[&str], i: &mut usize) -> String {
    let mut word = String::new();

    loop {
        word = format!("{} {}", word, cmds[*i]);
        if cmds[*i].contains(END_WORD) {
            *i += 1;
            break;
        }

        *i += 1;
    }

    word.to_lowercase().trim().to_string()
}

fn parse_line(buf: &str) -> Vec<String> {
    let cmds: Vec<&str> = buf.split_whitespace().collect();

    let mut args = Vec::<String>::new();
    let mut i = 0;

    while i < cmds.len() {
        if cmds[i].contains(START_WORD) {
            args.push(parse_word(&cmds, &mut i));
            continue;
        }

        args.push(cmds[i].to_ascii_lowercase().to_string());
        i += 1;
    }

    args
}

fn expand_word_body(forth: &mut Forth, word_body: &str) -> Result<String, Error> {
    let words: Vec<&str> = word_body.split_whitespace().collect();
    let mut final_word_body = String::new();

    for word in words {
        let mut w = word.to_string();
        if forth.word_exists(&w) {
            let index = forth.get_word_body_index(&w)?;
            w = format!("{}={}", w, index);
        }
        final_word_body = format!("{} {}", final_word_body, w);
    }

    Ok(final_word_body.trim().to_string())
}

fn define_word(forth: &mut Forth, buf: &str) -> Result<i16, Error> {
    let string = buf.trim_matches([START_WORD, END_WORD]).trim_ascii();
    let args: Vec<&str> = string.splitn(2, ' ').collect();

    let word_name = args[0].to_string();
    let word_body = expand_word_body(forth, args[1])?;

    forth.define_word(word_name, word_body)
}

fn get_word_body(forth: &mut Forth, args: &mut Vec<String>, mut i: usize) -> Result<i16, Error> {
    let words_body: &Vec<String>;
    let index: usize;

    if args[i].contains('=') {
        let aux: Vec<String> = args[i].splitn(2, '=').map(|s| s.to_string()).collect();
        words_body = forth.get_word_body(&aux[0])?;

        let (n, _) = is_numeric(aux[1].trim_matches('='));
        index = n as usize;
    } else {
        words_body = forth.get_word_body(&args[i])?;
        index = words_body.len() - 1;
    }

    let word_body: Vec<&str> = words_body[index].split_whitespace().collect();
    for word in word_body {
        args.insert(i + 1, word.to_string());
        i += 1;
    }
    Ok(0)
}

fn count_anidados(buf: &str) -> i16 {
    if buf.eq(IF) {
        1
    } else if buf.eq(THEN) {
        -1
    } else {
        0
    }
}

fn if_statement(forth: &mut Forth, args: &mut Vec<String>, mut i: usize) -> Result<i16, Error> {
    let result = forth.if_statement()?;
    args.remove(i);

    let mut anidados = 0;
    while anidados > 0 || args[i].ne(ELSE) && args[i].ne(THEN) {
        anidados += count_anidados(&args[i]);

        if result != 0 {
            i += 1;
        } else {
            args.remove(i);
        }
    }

    anidados = 0;

    if args[i].eq(ELSE) {
        args.remove(i);
    }

    while anidados > 0 || args[i].ne(THEN) {
        anidados += count_anidados(&args[i]);

        if result != 0 {
            args.remove(i);
        } else {
            i += 1;
        }
    }

    args.remove(i);
    Ok(0)
}

fn get_string(forth: &mut Forth, args: &mut [String], i: &mut usize) {
    *i += 1;

    let mut string = String::new();

    while !args[*i].contains('\"') {
        string = format!("{} {}", string, args[*i]);
        *i += 1;
    }
    string = format!("{} {}", string, args[*i]);

    forth.print_string(string.trim_matches('\"').to_string())
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
        "dup" => forth.dup(),
        "drop" => forth.drop(),
        "swap" => forth.swap(),
        "over" => forth.over(),
        "rot" => forth.rot(),
        "." => forth.print_stack(),
        "emit" => forth.emit(),
        "cr" => forth.cr(),
        "=" => forth.igual(),
        "<" => forth.menor(),
        ">" => forth.mayor(),
        "and" => forth.and(),
        "or" => forth.or(),
        "not" => forth.not(),
        &_ => Err(Error::MissingWord),
    }
}

fn read_line(forth: &mut Forth, mut args: Vec<String>) -> Result<i16, Error> {
    let mut i = 0;
    while i < args.len() {
        //println!("CMD = {:?}", args);

        if args[i].contains(START_WORD) {
            define_word(forth, &args[i])?;
        } else if forth.word_exists(&args[i]) {
            get_word_body(forth, &mut args, i)?;
        } else if args[i].eq(IF) {
            if_statement(forth, &mut args, i)?;
            continue;
        } else if args[i].eq(START_STRING) {
            get_string(forth, &mut args, &mut i);
        } else {
            let (value, is_numeric) = is_numeric(&args[i]);
            if is_numeric {
                forth.push(value)?;
            } else {
                do_operation(forth, &args[i])?;
            }
        }

        i += 1;
    }

    Ok(0)
}

fn run(path: &String, forth: &mut Forth) -> Result<i16, Error> {
    let file = match File::open(path) {
        Ok(file) => file,
        Err(_) => return Err(Error::FileOpenError),
    };

    let mut reader = BufReader::new(file);
    let mut buf = String::new();

    loop {
        match reader.read_line(&mut buf) {
            Ok(result) => {
                if result == 0 {
                    break;
                }
            }
            Err(_) => return Err(Error::FileReadError),
        };

        if word_not_complete(&buf) {
            continue;
        }

        read_line(forth, parse_line(&buf))?;
        buf.clear();
    }

    Ok(0)
}

pub fn main() {
    let mut env: Vec<String> = args().collect();
    let (stack_size, path) = match parse_cmd_arguments(&mut env) {
        Ok((stack_size, path)) => (stack_size, path),
        Err(error) => return println!("{}", error),
    };

    let mut forth = Forth::new(stack_size);

    match run(path, &mut forth) {
        Ok(_) => {}
        Err(error) => println!("{}", error),
    }

    match forth.write_stack(FILE) {
        Ok(_) => {}
        Err(error) => println!("{}", error),
    }
}
