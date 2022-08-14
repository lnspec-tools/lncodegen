/// Parse implementation!
pub mod ast_msg;
pub mod parser;

#[cfg(test)]
mod test {
    use crate::parser::ast_msg;
    use crate::parser::parser;
    use crate::scanner::scanner;
    use crate::scanner::token::CSVToken;

    #[test]
    fn parse_simple_one_line() {
        let contents = "msgtype,init,16\n";
        let char_vec: Vec<char> = contents.chars().collect();
        let mut scanner = scanner::Scanner::new();
        let result: Vec<CSVToken> = scanner.scan(&char_vec);
        let mut parser = parser::Parser::new(result.clone());
        let result = parser.parse_token(&result);
        assert!(result.len() > 0);
        debug_assert_eq!(result[0].0, ast_msg::AstMsgLineType::Msgtype);
    }

    #[test]
    fn parse_simple_lines() {
        let contents = "msgtype,init,16\nmsgdata,init,gflen,u16,\n";
        let char_vec: Vec<char> = contents.chars().collect();
        let mut scanner = scanner::Scanner::new();
        let result: Vec<CSVToken> = scanner.scan(&char_vec);
        let mut parser = parser::Parser::new(result.clone());
        let result = parser.parse_token(&result);
        assert!(result.len() > 1);
        debug_assert_eq!(result[0].0, ast_msg::AstMsgLineType::Msgtype);
        debug_assert_eq!(result[1].0, ast_msg::AstMsgLineType::MsgdataLength);
    }

    #[test]
    fn parse_simple_init_msg() {
        let contents = "msgtype,init,16\n
        msgdata,init,gflen,u16,
        msgdata,init,globalfeatures,byte,gflen\n
        msgdata,init,flen,u16,
        msgdata,init,features,byte,flen\n
        msgdata,init,tlvs,init_tlvs,
        tlvtype,init_tlvs,networks,1\n
        tlvdata,init_tlvs,networks,chains,chain_hash,...\n
        tlvtype,init_tlvs,remote_addr,3\n
        tlvdata,init_tlvs,remote_addr,data,byte,...\n";
        let char_vec: Vec<char> = contents.chars().collect();
        let mut scanner = scanner::Scanner::new();
        let result: Vec<CSVToken> = scanner.scan(&char_vec);
        let mut parser = parser::Parser::new(result.clone());
        let messages = parser.parse(&result);
        assert!(result.len() > 0);
        debug_assert_eq!(messages[0].get_msg_type(), ast_msg::AstMsgType::Init);
    }

    #[test]
    fn parse_simple_recursive_line() {
        let contents = "msgtype,init,16\nmsgdata,init,gflen,u16,\n";
        let char_vec: Vec<char> = contents.chars().collect();
        let mut scanner = scanner::Scanner::new();
        let result: Vec<CSVToken> = scanner.scan(&char_vec);
        let mut parser = parser::Parser::new(result.clone());
        parser.parse_recurisve();
        debug_assert!(parser.lines.len() > 0);
        debug_assert_eq!(parser.lines[0].0, ast_msg::AstMsgLineType::Msgtype);
        debug_assert_eq!(parser.lines[1].0, ast_msg::AstMsgLineType::MsgdataLength);
    }
}
