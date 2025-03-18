pub mod forth;

use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

const DEFAULT_SIZE: usize = 128;

fn procces_line(forth: &forth::Forth, buf: &String) {}

fn run(path: String, forth: &forth::Forth) -> Result<&forth::Forth, io::Error> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    loop {
        let mut buf = String::new();
        let result = reader.read_line(&mut buf)?;

        if result == 0 {
            break;
        }

        procces_line(forth, &buf);

        buf.clear();
    }

    Ok(forth)
}

fn main() {
    let path = String::from("data/word.fth");
    let stack_size = DEFAULT_SIZE;
    let forth = forth::Forth::new(stack_size);

    let _ = run(path, &forth);
}
