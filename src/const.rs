/// Constantes que se utilizaran a lo largo del programa.
pub mod consts {
    pub const DEFAULT_SIZE: usize = 128 * 1024;
    pub const ARGV: usize = 2;
    pub const FILE: &str = "stack.fth";
    pub const IF: &str = "if";
    pub const CR: &str = "cr";
    pub const OR: &str = "or";
    pub const DUP: &str = "dup";
    pub const ROT: &str = "rot";
    pub const NOT: &str = "not";
    pub const AND: &str = "and";
    pub const THEN: &str = "then";
    pub const DROP: &str = "drop";
    pub const OVER: &str = "over";
    pub const ELSE: &str = "else";
    pub const EMIT: &str = "emit";
    pub const SWAP: &str = "swap";
    pub const START_STRING: &str = ".\"";
    pub const PRINT_STACK: &str = ".";
    pub const ADD: &str = "+";
    pub const SUB: &str = "-";
    pub const MUL: &str = "*";
    pub const DIV: &str = "/";
    pub const EQUAL: &str = "=";
    pub const GREATER: &str = ">";
    pub const LOWER: &str = "<";
    pub const START_WORD: char = ':';
    pub const END_WORD: char = ';';
}
