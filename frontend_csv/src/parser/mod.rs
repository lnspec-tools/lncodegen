/// Parse implementation!
pub mod ast;
pub mod parser;

#[cfg(test)]
mod test {

    use crate::parser::ast;
    use crate::parser::parser;
    use crate::scanner::scanner;
    use crate::scanner::token::CSVToken;
    use std::sync::Once;

    static INIT: Once = Once::new();

    fn init() {
        // ignore error
        INIT.call_once(|| {
            env_logger::init();
        });
    }

    #[test]
    fn parse_simple_recursive_line() {
        init();
        let contents = "msgtype,init,16\nmsgdata,init,gflen,u16,\n";
        let char_vec: Vec<char> = contents.chars().collect();
        let mut scanner = scanner::Scanner::new();
        let result: Vec<CSVToken> = scanner.scan(&char_vec);
        let mut parser = parser::Parser::new();
        parser.parse(&result);
        debug_assert_eq!(
            parser.symbol_table.get("init").unwrap().msg_data[0],
            ast::LNMsData::Uint("u16".to_string())
        );
    }

    #[test]
    #[should_panic]
    fn parse_simple_failure_line() {
        init();
        let contents = "msgtype,init,16 \
                        msgdata,init,gflen,u16";
        let char_vec: Vec<char> = contents.chars().collect();
        let mut scanner = scanner::Scanner::new();
        let result: Vec<CSVToken> = scanner.scan(&char_vec);
        let mut parser = parser::Parser::new();
        parser.parse(&result);
    }

    #[test]
    fn parse_simple_recursive_msg() {
        init();

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
        let mut parser = parser::Parser::new();
        parser.parse(&result);
        // check bytes line
        assert_eq!(
            parser.symbol_table.get("init").unwrap().msg_data[1],
            ast::LNMsData::BitfieldStream("globalfeatures".to_string(), "2".to_string())
        );
        // check TLV line
        assert!(parser
            .symbol_table
            .get("init")
            .unwrap()
            .tlv_stream
            .contains_key("init_tlvs"));
    }
}
