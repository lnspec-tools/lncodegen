/// Parse implementation!
pub mod astMsg;
pub mod parser;

#[cfg(test)]
mod test {
    use crate::parser::astMsg;
    use crate::parser::parser;
    use crate::scanner::scanner;
    use crate::scanner::token::CSVToken;
    #[test]
    fn scan_simple_one() {
        let contents = "msgtype,init,16\n";
        let char_vec: Vec<char> = contents.chars().collect();
        let mut scanner = scanner::Scanner::new(1);
        let result: Vec<CSVToken> = scanner.scan(&char_vec);
        let mut parser = parser::Parser::new();
        let result = parser.parse(&result);
        assert!(result.len() > 0);
        debug_assert_eq!(result[0], astMsg::AstMsgType::Msgtype);
    }
}
