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
const DEFAULT_SIZE: usize = 128;
const ARGV: usize = 2;

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

fn parse_word(cmds: &[&str], i: &mut usize) -> String {
    let mut word = String::new();
    loop {
        word = format!("{} {}", word, cmds[*i]);
        if cmds[*i].eq(";") {
            *i += 1;
            break;
        }

        *i += 1;
    }

    word.trim().to_string()
}

fn parse_line(buf: &str) -> Vec<String> {
    let cmds: Vec<&str> = buf.split_whitespace().collect();

    let mut args = Vec::<String>::new();
    let mut i = 0;

    while i < cmds.len() {
        if cmds[i].eq(":") {
            args.push(parse_word(&cmds, &mut i));
            continue;
        }

        args.push(cmds[i].to_string());
        i += 1;
    }

    args
}

fn define_word(forth: &mut Forth, buf: &str) -> Result<i16, Error> {
    let string = buf.trim_matches([':', ';']).trim_ascii();
    let args: Vec<&str> = string.splitn(2, ' ').collect();

    forth.define_word(args[0].to_string(), args[1].to_string())
}

fn is_numeric(buf: &str) -> (i16, bool) {
    match buf.parse::<i16>() {
        Ok(n) => (n, true),
        Err(_) => (0, false),
    }
}

fn get_word_body(forth: &mut Forth, args: &mut Vec<String>, mut i: usize) -> Result<i16, Error> {
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

fn process_conditional(
    forth: &mut Forth,
    args: &mut Vec<String>,
    mut i: usize,
) -> Result<i16, Error> {
    let result = forth.if_statement()?;

    while args[i].ne("ELSE") && args[i].ne("THEN") {
        if result != 0 {
            i += 1;
        } else {
            args.remove(i);
        }
    }

    while args[i].ne("THEN") {
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
        "AND" => forth.and(),
        "OR" => forth.or(),
        "NOT" => forth.not(),
        &_ => Err(Error::MissingWord),
    }
}

fn process_args(forth: &mut Forth, args: &mut Vec<String>, i: &mut usize) -> Result<i16, Error> {
    if args[*i].contains(":") {
        define_word(forth, &args[*i])?;
    } else if forth.word_exists(&args[*i]) {
        get_word_body(forth, args, *i)?;
    } else if args[*i].eq("IF") {
        process_conditional(forth, args, *i)?;
        *i -= 1;
    } else if args[*i].eq(".\"") {
        get_string(forth, args, i);
    } else {
        let (value, is_numeric) = is_numeric(&args[*i]);
        if is_numeric {
            forth.push(value)?;
        } else {
            do_operation(forth, &args[*i])?;
        }
    }

    Ok(0)
}

fn read_line(forth: &mut Forth, mut args: Vec<String>) -> Result<i16, Error> {
    let mut i = 0;
    while i < args.len() {
        process_args(forth, &mut args, &mut i)?;
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
    loop {
        let mut buf = String::new();
        let result = match reader.read_line(&mut buf) {
            Ok(result) => result,
            Err(_) => return Err(Error::FileReadError),
        };

        if result == 0 {
            break;
        }

        read_line(forth, parse_line(&buf))?;
    }

    Ok(0)
}

fn main() {
    let mut env: Vec<String> = args().collect();
    let (stack_size, path) = match parse_cmd_arguments(&mut env) {
        Ok((stack_size, path)) => (stack_size, path),
        Err(error) => return println!("{}", error),
    };

    let mut forth = Forth::new(stack_size);

    match run(path, &mut forth) {
        Ok(_) => match forth.write_stack(FILE) {
            Ok(_) => {}
            Err(error) => println!("{}", error),
        },
        Err(error) => println!("{}", error),
    }
}
