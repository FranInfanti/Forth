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

/// Retorna el size del stack que deberia estar especificado en
/// el argumento pasado por parametro.
///
/// # Errors
///
/// Returns [`InvalidArguments`](Error::InvalidArguments) si los argumentos
/// recibidos son invalidos.
///
/// Returns [`ParsingError`](Error::ParsingError) si el size del stack no es
/// númerico.
///
/// # Examples
///
/// ```
///     let env = "stack-size=120";
///     let stack_size = get_stack_size(env);
///     // stack_size = 120;
/// ```
///
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

/// Retorna el path del archivo a interpretar y el size del stack.
/// En caso de no especificar este ultimo, se considera un size de 128 Kb.
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

/// Retorna si la definición del word esta completa en buf.
///
/// # Errors
///
/// # Examples
///
/// ```
/// let bool = word_not_complete(": MAX OVER ");
/// // bool = false;
/// ```
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
/// Retorna un vector de String que contiene el formato deseado.
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
    let mut args = Vec::<String>::new();
    let mut i = 0;

    while i < cmds.len() {
        if cmds[i].contains(START_WORD) {
            args.push(parse_word(&cmds, &mut i));
            continue;
        }

        args.push(cmds[i].to_string());
        i += 1;
    }

    args
}

fn define_word(forth: &mut Forth, buf: &str) -> Result<i16, Error> {
    let string = buf.trim_matches([START_WORD, END_WORD]).trim_ascii();
    let args: Vec<&str> = string.splitn(2, ' ').collect();

    let word_name = args[0].to_string();
    let word_body = args[1].to_string();

    forth.define_word(word_name, word_body)
}

fn get_body(forth: &mut Forth, args: &mut Vec<String>, mut i: usize) -> Result<i16, Error> {
    let body = forth.get_word_body(&args[i])?;

    let word_body = body;
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

/// Se encarga de procesar la condición del if, haciendo la evaluación y
/// determinando que rama tomar.
/// Retorna 0.
///
/// # Errors
///
/// Returns [`StackUnderflow`](Error::StackUnderflow) si se intenta tomar
/// un elemento de un stack vacio.
///
/// # Example
///
/// ```
///     let mut args = vec!["0", "-1", "+", "IF", "1", "+", "ELSE", "0", "/", "THEN"];
///     if_statement(forth, &mut args, 3);
///     // args = ["0", "-1", "+", "1", "+"];
/// ```
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

/// Extrae del formato de strings, el String a mostrar por stdout.
///
/// # Errors
///
/// # Example
///
/// ```
///     let str = ".\" Hola Mundo\"";
///     print_string(forth, str);
///     // stdout = Hola Mundo
/// ```
fn print_string(forth: &mut Forth, string: &str) {
    let chars: Vec<char> = string.chars().collect();
    let mut string = String::new();

    let mut i = 3; // ignora ' ." (space) '
    while i < chars.len() {
        if chars[i] == '"' {
            break;
        }

        string = format!("{}{}", string, chars[i]);
        i += 1;
    }

    forth.print_string(string.to_string());
}

/// Intenta parsear un &str a un i16.
/// Retorna, en caso de exito, el valor parseado y un booleano
/// con valor true que indica el exito de la operación.
/// Retorna, en caso de error, 0 y un booleano con valor false
/// indicando que no se pudo
///
/// # Errors
///
fn is_numeric(buf: &str) -> (i16, bool) {
    match buf.parse::<i16>() {
        Ok(n) => (n, true),
        Err(_) => (0, false),
    }
}

/// Realiza el matching según las operaciones nativas del interprete.
/// Una vez que realiza el match ejecuta la operación.
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
        SUMA => forth.suma(),
        RESTA => forth.resta(),
        PRODUCTO => forth.producto(),
        DIVISION => forth.division(),
        DUP => forth.dup(),
        DROP => forth.drop(),
        SWAP => forth.swap(),
        OVER => forth.over(),
        ROT => forth.rot(),
        PRINT_STACK => forth.print_stack(),
        EMIT => forth.emit(),
        CR => forth.cr(),
        IGUAL => forth.igual(),
        MENOR => forth.menor(),
        MAYOR => forth.mayor(),
        AND => forth.and(),
        OR => forth.or(),
        NOT => forth.not(),
        &_ => Err(Error::MissingWord),
    }
}

/// Se encarga de, dada una linea, delegar a distintas funciones
/// según que operación se debe ejecutar.
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
fn read_line(forth: &mut Forth, mut args: Vec<String>) -> Result<i16, Error> {
    let mut i = 0;
    while i < args.len() {
        if args[i].contains(START_WORD) {
            define_word(forth, &args[i])?;
        } else if forth.word_exists(&args[i]) {        
            get_body(forth, &mut args, i)?;
        } else if args[i].eq(IF) {
            if_statement(forth, &mut args, i)?;
            continue;
        } else if args[i].contains(START_STRING) {
            print_string(forth, &args[i]);
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

/// Se encarga de abrir el archivo a interpretar y leerlo linea por linea.
/// Delegando la ejecución de cada linea a otra función.
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
