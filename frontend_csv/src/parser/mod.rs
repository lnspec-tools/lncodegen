/// Parse implementation!
pub mod ast;
pub mod parser;

#[cfg(test)]
mod test {

    use crate::parser::ast;
    use crate::parser::ast::LNMsgType;
    use crate::parser::parser;
    use crate::scanner::scanner;
    use crate::scanner::token::CSVToken;
    use std::fs;
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
        if let LNMsgType::Msg(msg) = parser.symbol_table.get("init").unwrap() {
            assert_eq!(msg.msg_data[0], ast::LNMsData::Uint("u16".to_string()));
        } else {
            panic!();
        }
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
        match parser.symbol_table.get("init").unwrap() {
            LNMsgType::Msg(msg) => assert_eq!(
                msg.to_owned().msg_data[1],
                ast::LNMsData::BitfieldStream("globalfeatures".to_string(), "gflen".to_string())
            ),
            _ => panic!("wrong value in the symbol table"),
        }
        // check TLV line
        assert!(parser.symbol_table.contains_key("init_tlvs"));
    }

    #[test]
    fn parse_bolt7_file() {
        let path_file = std::env::var_os("CSV_PATH").unwrap();
        let contents = fs::read_to_string(format!("{}/bolt7.csv", path_file.to_str().unwrap()))
            .expect("Something went wrong reading the file");
        let char_vec: Vec<char> = contents.chars().collect();
        let mut scanner = scanner::Scanner::new();
        let result = scanner.scan(&char_vec);
        let mut parser = parser::Parser::new();
        parser.parse(&result);

        // TODO: make check
    }
}
