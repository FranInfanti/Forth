use crate::r#const::consts::*;
use crate::utils::split::split;

pub mod r#const;
pub mod error;
pub mod forth;
pub mod utils;

use error::Error;
use forth::Forth;
use std::{
    env::args,
    fs::File,
    io::{BufRead, BufReader},
};

/// Retorna el stack_size
///
/// # Errors
///
/// Returns [`InvalidArguments`](Error::InvalidArguments) si los argumentos
/// recibidos son invalidos.
///
/// Returns [`ParsingError`](Error::ParsingError) si el size del stack no es
/// númerico.
///
fn get_stack_size(arg: &str) -> Result<usize, Error> {
    let chars: Vec<&str> = arg.splitn(2, '=').collect();

    if chars.len() != 2 {
        return Err(Error::InvalidArguments);
    }

    match chars[1].parse::<usize>() {
        Ok(stack_size) => Ok(stack_size),
        Err(_) => Err(Error::ParsingError),
    }
}

/// Retorna el path del archivo a interpretar y el stack size.
/// En caso de no especificar un stack size, se considera 128 Kb.
///
/// # Errors
///
/// Returns [`InvalidAmountOfArguments`](Error::InvalidAmountOfArguments) si se envia
/// una cantidad invalida de parametros.
///
/// # Examples
///
/// ```
///     let mut env = vec!["data/suma.fth", "stack-size=10"];
///     let (size, path) = parse_cmd_arguments(&mut env);
///     // size = 10;
///     // path = "data/suma.fth";
/// ```
///
fn parse_cmd_arguments(argv: &mut Vec<String>) -> Result<(usize, &String), Error> {
    argv.remove(0);

    if argv.is_empty() || argv.len() > ARGS {
        return Err(Error::InvalidAmountOfArguments);
    }

    let mut stack_size = DEFAULT_SIZE;
    if argv.len() == ARGS {
        stack_size = get_stack_size(&argv[1])?;
    }

    Ok((stack_size, &argv[0]))
}

/// Retorna si la definición del word comienza y finaliza en buf.
///
/// # Errors
///
fn word_not_complete(buf: &str) -> bool {
    buf.contains(START_WORD) && !buf.contains(END_WORD)
}

/// Realiza un parsing de la definición de un word.
/// Retorna un String que contiene el parseo deseado.
///
/// # Errors
///
/// # Examples
///
/// ```
///     let cmds = vec![":", "MAX", "OVER", "OVER", ";"]
///     let mut i = 0;
///     let word = parse_word(&cmds, &mut i);
///     // word = ": MAX OVER OVER ;";
///     // i = 4;
/// ```
///
fn parse_word(cmds: &[String], i: &mut usize) -> String {
    let mut word = String::new();

    loop {
        word = format!("{} {}", word, cmds[*i]);
        if cmds[*i].contains(END_WORD) {
            *i += 1;
            break;
        }

        *i += 1;
    }

    word.trim().to_string()
}

/// Parsea un String a un formato conveniente.
/// Retorna un `Vec<String>` con el formato deseado.
///
/// # Errors
///
/// # Examples
///
/// ```
///     let buf = "1 2 + IF : MAX OVER ; ELSE .\" Nein   \" THEN";
///     let args = parse_line(buf);
///     // args = ["1", "2", "+", "IF", ": MAX OVER ;", "ELSE", ." Nein   ", "THEN"];
/// ```
///
fn parse_line(buf: &str) -> Vec<String> {
    let cmds = split(buf.trim());
    let mut argv = Vec::<String>::new();

    let mut i = 0;
    while i < cmds.len() {
        if cmds[i].contains(START_WORD) {
            argv.push(parse_word(&cmds, &mut i));
            continue;
        }

        argv.push(cmds[i].to_string());
        i += 1;
    }

    argv
}

/// Retorna si los siguientes args forman parte de un word.
///
/// # Errors
///
fn is_word(arg: &str) -> bool {
    arg.contains(START_WORD)
}

/// Define un word en el interprete.
/// Retorna 0.
///
/// # Errors
///
/// Returns [`InvalidWord`](Error::InvalidWord) si se intenta definir
/// un word cuyo word-name es invalido.
///
fn define_word(forth: &mut Forth, word: &str) -> Result<i16, Error> {
    let string = word.trim_matches([START_WORD, END_WORD]).trim_ascii();
    let args: Vec<&str> = string.splitn(2, ' ').collect();

    let word_name = args[0].to_string();
    let word_body = args[1].to_string();

    forth.define_word(word_name, word_body)
}

/// Retorna si ya existe un word definido con nombre word_name.
///
/// # Errors
///
fn is_word_defined(forth: &Forth, word_name: &str) -> bool {
    forth.word_exists(word_name)
}

/// Añade a los argumentos a ejecutar el word-body de la word que
/// se encontro.
/// Retorna 0.
///
/// # Errors
///
/// Returns [`MissingWord`](Error::MissingWord) si no existe un word cuyo
/// nombre sea igual al encontrado.
///
fn get_body(forth: &mut Forth, argv: &mut Vec<String>, mut i: usize) -> Result<i16, Error> {
    let word_body = forth.get_word_body(&argv[i])?;

    for operation in word_body {
        argv.insert(i + 1, operation.to_string());
        i += 1;
    }
    Ok(0)
}

/// Retorna si se esta por ejecutar una operación condicional.
///
/// # Errors
///
fn is_if_statment(arg: &String) -> bool {
    arg.eq(IF)
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

/// Ejecuta, según el resultado del if, el bloque de codigo del if o
/// el else, si existe un else.
/// Retorna 0.
///
/// # Errors
///
/// Returns [`StackUnderflow`](Error::StackUnderflow) si se intenta tomar
/// un elemento de un stack vacio.
///
fn if_statement(forth: &mut Forth, argv: &mut Vec<String>, mut i: usize) -> Result<i16, Error> {
    let result = forth.if_statement()?;
    argv.remove(i);

    let mut anidados = 0;
    while anidados > 0 || argv[i].ne(ELSE) && argv[i].ne(THEN) {
        anidados += count_anidados(&argv[i]);
        if result != 0 {
            i += 1;
        } else {
            argv.remove(i);
        }
    }

    anidados = 0;
    if argv[i].eq(ELSE) {
        argv.remove(i);
    }

    while anidados > 0 || argv[i].ne(THEN) {
        anidados += count_anidados(&argv[i]);
        if result != 0 {
            argv.remove(i);
        } else {
            i += 1;
        }
    }

    argv.remove(i);
    Ok(0)
}

/// Retorna si se esta por definir un string para mostrar
/// por stdout.
///
/// # Errors
///
fn is_string(arg: &str) -> bool {
    arg.contains(START_STRING)
}

/// Extrae y muestra el string por stdout.
///
/// # Errors
///
fn print_string(forth: &mut Forth, string: &str) {
    let chars: Vec<char> = string.chars().collect();
    let mut string = String::new();

    let mut i = 3; // ignora ' ." (space)'
    while i < chars.len() {
        if chars[i] == '"' {
            break;
        }

        string = format!("{}{}", string, chars[i]);
        i += 1;
    }

    forth.print_string(string.to_string());
}

/// Retorna si el buf puede ser parseado a i16.
///
/// # Errors
///
fn is_numeric(buf: &str) -> bool {
    buf.parse::<i16>().is_ok()
}

/// Pushea al stack del interprete un arg númerico.
/// Retorna el value pusheado.
///
/// # Errors
///
/// Returns [`ParsingError`](Error::ParsingError) si ocurre algun error
/// de parseo.
///
/// Returns [`StackOverflow`](Error::StackOverflow) si se intenta pushear
/// un elemento a un stack completo.
///  
fn push_value(forth: &mut Forth, arg: &str) -> Result<i16, Error> {
    let value = match arg.parse::<i16>() {
        Ok(value) => value,
        Err(_) => return Err(Error::ParsingError),
    };
    forth.push(value)
}

/// Ejecuta las operaciones nativas del interprete.
/// Retorna lo que retorne el resultado de ejecutar la operación.
///
/// # Errors
///
/// Returns [`StackOverflow`](Error::StackOverflow) si se sobrepasa
/// el tamaño del stack.
///
/// Returns [`StackUnderflow`](Error::StackUnderflow) si se intenta tomar
/// un elemento de un stack vacio.
///
/// Returns [`DivisionByZero`](Error::DivisionByZero) si se intenta realizar
/// una división por cero.
///
/// Returns [`ParsingError`](Error::ParsingError) si se intenta tomar
/// un elemento de un stack vacio.
///
/// Returns [`MissingWord`](Error::MissingWord) si se intenta acceder a un
/// word que no se encuentra definido.
///
fn do_operation(forth: &mut Forth, buf: &str) -> Result<i16, Error> {
    match buf {
        ADD => forth.add(),
        SUB => forth.sub(),
        MUL => forth.mul(),
        DIV => forth.div(),
        DUP => forth.dup(),
        DROP => forth.drop(),
        SWAP => forth.swap(),
        OVER => forth.over(),
        ROT => forth.rot(),
        PRINT => forth.print_stack(),
        EMIT => forth.emit(),
        CR => forth.cr(),
        EQUAL => forth.equal(),
        LOWER => forth.lower(),
        GREATER => forth.greater(),
        AND => forth.and(),
        OR => forth.or(),
        NOT => forth.not(),
        &_ => Err(Error::MissingWord),
    }
}

/// Ejecuta las operaciones de la linea que se leyo del archivo.
/// Retorna 0.
///
/// # Errors
///
/// Returns [`StackOverflow`](Error::StackOverflow) si se sobrepasa
/// el tamaño del stack.
///
/// Returns [`StackUnderflow`](Error::StackUnderflow) si se intenta tomar
/// un elemento de un stack vacio.
///
/// Returns [`DivisionByZero`](Error::DivisionByZero) si se intenta realizar
/// una división por cero.
///
/// Returns [`ParsingError`](Error::ParsingError) si se intenta tomar
/// un elemento de un stack vacio.
///
/// Returns [`InvalidWord`](Error::InvalidWord) si se intenta definir un  
/// word con un nombre invalido.
///
/// Returns [`MissingWord`](Error::MissingWord) si se intenta acceder a un
/// word que no se encuentra definido.
///
fn read_line(forth: &mut Forth, mut argv: Vec<String>) -> Result<i16, Error> {
    let mut i = 0;
    while i < argv.len() {
        if is_word(&argv[i]) {
            define_word(forth, &argv[i])?;
        } else if is_word_defined(forth, &argv[i]) {
            get_body(forth, &mut argv, i)?;
        } else if is_if_statment(&argv[i]) {
            if_statement(forth, &mut argv, i)?;
            continue;
        } else if is_string(&argv[i]) {
            print_string(forth, &argv[i]);
        } else if is_numeric(&argv[i]) {
            push_value(forth, &argv[i])?;
        } else {
            do_operation(forth, &argv[i])?;
        }

        i += 1;
    }

    Ok(0)
}

/// Abre el archivo a interpretar y ejecuta linea por linea.
/// Retorna 0.
///
/// # Errors
///
/// Returns [`StackOverflow`](Error::StackOverflow) si se sobrepasa
/// el tamaño del stack.
///
/// Returns [`StackUnderflow`](Error::StackUnderflow) si se intenta tomar
/// un elemento de un stack vacio.
///
/// Returns [`DivisionByZero`](Error::DivisionByZero) si se intenta realizar
/// una división por cero.
///
/// Returns [`ParsingError`](Error::ParsingError) si se intenta tomar
/// un elemento de un stack vacio.
///
/// Returns [`InvalidWord`](Error::InvalidWord) si se intenta definir un  
/// word con un nombre invalido.
///
/// Returns [`MissingWord`](Error::MissingWord) si se intenta acceder a un
/// word que no se encuentra definido.
///
/// Returns [`FileOpenError`](Error::FileOpenError) si se intenta abrir un
/// y ocurre algun error.
///
/// Returns [`FileReadError`](Error::FileReadError) si se lee un archivo
/// y ocurre algun error.
///
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
    let mut argv: Vec<String> = args().collect();
    let (stack_size, path) = match parse_cmd_arguments(&mut argv) {
        Ok((stack_size, path)) => (stack_size, path),
        Err(error) => return println!("{}", error),
    };

    let mut forth = Forth::new(stack_size);

    match run(path, &mut forth) {
        Ok(_) => {}
        Err(error) => println!("{error}"),
    }

    match forth.write_stack(FILE) {
        Ok(_) => {}
        Err(error) => println!("{error}"),
    }
}
