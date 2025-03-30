use std::{
    fs::File,
    io::{BufRead, BufReader},
    process::Command,
};

const PROGRAM: &str = "cargo";

fn read_file() -> Vec<i16> {
    let mut output = Vec::<i16>::new();
    let file = File::open("stack.fth").unwrap();
    let mut reader = BufReader::new(file);

    let mut buf = String::new();
    while reader.read_line(&mut buf).unwrap() > 0 {
        let vec: Vec<&str> = buf.trim().split_ascii_whitespace().collect();

        for v in vec {
            let number = v.parse::<i16>().unwrap();
            output.push(number);
        }

        buf.clear();
    }

    output
}

#[test]
fn test_arithmetic_operations() {
    let _ = Command::new(PROGRAM)
        .args(["run", "--", "data/arithmetic.fth"])
        .output()
        .unwrap();

    let expected: Vec<i16> = vec![3, 1, 5, -1, 1, 9, 8, 1, 6, 4, 2, 1, 4, -1, 2, 13, 7];
    let output = read_file();

    let mut output_correct = true;
    let mut i = 0;
    while i < expected.len() {
        if expected[i] != output[i] {
            output_correct = false;
            break;
        }
        i += 1;
    }

    assert!(output_correct);
}

#[test]
fn test_boolean_operations() {
    let _ = Command::new(PROGRAM)
        .args(["run", "--", "data/boolean.fth"])
        .output()
        .unwrap();

    let expected: Vec<i16> = vec![-1, 0, -1, 0, 0, -1, 0, 0, 0, 0, -1, 0, -1, -1, 0, -1, -1];
    let output = read_file();

    let mut output_correct = true;
    let mut i = 0;
    while i < expected.len() {
        if expected[i] != output[i] {
            output_correct = false;
            break;
        }
        i += 1;
    }

    assert!(output_correct);
}

#[test]
fn test_conditionals_operations() {
    let _ = Command::new(PROGRAM)
        .args(["run", "--", "data/conditionals.fth"])
        .output()
        .unwrap();

    let expected: Vec<i16> = vec![2, 2, 3, 1, 2, 3, 2, 3, 4, 10];
    let output = read_file();

    let mut output_correct = true;
    let mut i = 0;
    while i < expected.len() {
        if expected[i] != output[i] {
            output_correct = false;
            break;
        }
        i += 1;
    }

    assert!(output_correct);
}

#[test]
fn test_words_operations() {
    let _ = Command::new(PROGRAM)
        .args(["run", "--", "data/words.fth"])
        .output()
        .unwrap();

    let expected: Vec<i16> = vec![152, 7985, 16, 136, 511, 1024, 5, 6, 1, 1, 1, 1, 12, 1, 1, 1];
    let output = read_file();

    let mut output_correct = true;
    let mut i = 0;
    while i < expected.len() {
        if expected[i] != output[i] {
            output_correct = false;
            break;
        }
        i += 1;
    }

    assert!(output_correct);
}

#[test]
fn test_native_operations() {
    let _ = Command::new(PROGRAM)
        .args(["run", "--", "data/native.fth"])
        .output()
        .unwrap();

    let expected: Vec<i16> = vec![
        1, 1, 1, 2, 2, 1, 2, 1, 1, 3, 2, 1, 2, 1, 1, 2, 3, 2, 2, 3, 1, 1, 2, 3, 1, 1, 1, 1, 1, 2,
        3, 4, 1, 1, 2, 1, 2, 1,
    ];
    let output = read_file();

    let mut output_correct = true;
    let mut i = 0;
    while i < expected.len() {
        if expected[i] != output[i] {
            output_correct = false;
            break;
        }
        i += 1;
    }

    assert!(output_correct);
}

#[test]
fn test_output_operations() {
    let output = Command::new(PROGRAM)
        .args(["run", "--", "data/output.fth"])
        .output()
        .unwrap();

    let expected = b"A B C D Hola Mundo Hello      World! Hallo \nWelt 1 \n\n2";
    let stdout = output.stdout.to_vec();

    let mut i = 0;
    let mut output_correct = true;
    while i < expected.len() {
        if stdout[i] != expected[i] {
            output_correct = false;
            break;
        }

        i += 1;
    }

    assert!(output_correct);
}

#[test]
fn test_error_stack_underflow() {
    let output = Command::new(PROGRAM)
        .args(["run", "--", "data/stack_underflow.fth"])
        .output()
        .unwrap();

    let expected = b"stack-underflow";
    let stdout = output.stdout.to_vec();

    let mut i = 0;
    let mut output_correct = true;
    while i < expected.len() {
        if stdout[i] != expected[i] {
            output_correct = false;
            break;
        }

        i += 1;
    }

    assert!(output_correct);
}

#[test]
fn test_error_stack_overflow() {
    let output = Command::new(PROGRAM)
        .args(["run", "--", "data/stack_overflow.fth", "stack-size=10"])
        .output()
        .unwrap();

    let expected = b"5 \nstack-overflow";
    let stdout = output.stdout.to_vec();

    println!("{:?}", expected);
    println!("{:?}", stdout);

    let mut i = 0;
    let mut output_correct = true;
    while i < expected.len() {
        if stdout[i] != expected[i] {
            output_correct = false;
            break;
        }

        i += 1;
    }

    assert!(output_correct);
}
