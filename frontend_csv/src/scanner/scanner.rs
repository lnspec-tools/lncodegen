use super::token::CSVToken;

/// Core implementation of the scanner
pub struct Scanner {
    pos: usize,
    line: u64,
    tokens: Vec<CSVToken>,
    identifier_symbols: Vec<char>,
}

impl Scanner {
    fn scan(&mut self, _symbols: &Vec<char>) {}
}
