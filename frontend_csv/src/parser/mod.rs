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
    fn parse_simple_recursive_line() {
        let contents = "msgtype,init,16\nmsgdata,init,gflen,u16,\n";
        let char_vec: Vec<char> = contents.chars().collect();
        let mut scanner = scanner::Scanner::new();
        let result: Vec<CSVToken> = scanner.scan(&char_vec);
        let mut parser = parser::Parser::new(result.clone());
        parser.parse_recurisve_tokens();
        debug_assert!(parser.lines.len() > 0);
        debug_assert_eq!(parser.lines[0].0, ast_msg::AstMsgLineType::Msgtype);
        debug_assert_eq!(parser.lines[1].0, ast_msg::AstMsgLineType::MsgdataLength);
    }

    #[test]
    #[should_panic]
    fn parse_simple_failure_line() {
        let contents = "msgtype,init,16\nmsgdata,init,gflen,u16,\n
        msgdata,init,globalfeatures,gflen\n";
        let char_vec: Vec<char> = contents.chars().collect();
        let mut scanner = scanner::Scanner::new();
        let result: Vec<CSVToken> = scanner.scan(&char_vec);
        let mut parser = parser::Parser::new(result.clone());
        parser.parse_recurisve_tokens();
        debug_assert!(parser.lines.len() > 0);
        debug_assert_eq!(parser.lines[0].0, ast_msg::AstMsgLineType::Msgtype);
        debug_assert_eq!(parser.lines[1].0, ast_msg::AstMsgLineType::MsgdataLength);
    }

    #[test]
    fn parse_simple_recursive_msg() {
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
        let mut parser2 = parser::Parser::new(result.clone());
        parser.parse_recursive();
        parser2.parse_recurisve_tokens();
        debug_assert!(parser.messages.len() > 0);
        let expected: Vec<Vec<String>> = parser2.lines.into_iter().map(|p| p.1).collect();
        debug_assert_eq!(parser.messages[0].get_values(), expected);
    }

    #[test]
    fn parse_two_recursive_msg() {
        let contents = "msgtype,error,17\n
        msgdata,error,channel_id,channel_id,\n
        msgdata,error,len,u16,\n
        msgdata,error,data,byte,len\n
        msgtype,warning,1\n
        msgdata,warning,channel_id,channel_id,\n
        msgdata,warning,len,u16,\n
        msgdata,warning,data,byte,len\n";

        let expected = [
            "msgtype,error,17\n
        msgdata,error,channel_id,channel_id,\n
        msgdata,error,len,u16,\n
        msgdata,error,data,byte,len\n",
            "msgtype,warning,1\n
        msgdata,warning,channel_id,channel_id,\n
        msgdata,warning,len,u16,\n
        msgdata,warning,data,byte,len\n",
        ];

        let char_vec: Vec<char> = contents.chars().collect();
        let mut scanner = scanner::Scanner::new();
        let result: Vec<CSVToken> = scanner.scan(&char_vec);
        debug_assert_eq!(result[0].val, "msgtype");
        let mut parser = parser::Parser::new(result.clone());
        parser.parse_recursive();
        debug_assert!(parser.messages.len() == 2);
        for pos in 1..expected.len() {
            let expected_token = expected[pos].chars().collect();
            let expectedline = scanner.scan(&expected_token);
            let mut expected_parser = parser::Parser::new(expectedline.clone());
            expected_parser.parse_recurisve_tokens();
            let expectedmsg: Vec<Vec<String>> =
                expected_parser.lines.into_iter().map(|p| p.1).collect();
            debug_assert_eq!(parser.messages[pos].get_values(), expectedmsg);
        }
    }
}
