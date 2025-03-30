/// Definición de constantes que se utilizaran a
/// lo largo del programa.
pub mod consts {
    pub const FILE: &str = "stack.fth";
    pub const DEFAULT_SIZE: usize = 128 * 1024;
    pub const ARGV: usize = 2;

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
    pub const SUMA: &str = "+";
    pub const RESTA: &str = "-";
    pub const PRODUCTO: &str = "*";
    pub const DIVISION: &str = "/";
    pub const IGUAL: &str = "=";
    pub const MAYOR: &str = ">";
    pub const MENOR: &str = "<";

    pub const START_WORD: char = ':';
    pub const END_WORD: char = ';';
}
