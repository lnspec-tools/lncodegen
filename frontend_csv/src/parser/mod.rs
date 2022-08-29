/// Parse implementation!
pub mod ast;
pub mod parser;

#[cfg(test)]
mod test {

    use crate::parser::ast;
    use crate::parser::parser;
    use crate::scanner::scanner;
    use crate::scanner::token::CSVToken;

    #[test]
    fn parse_simple_recursive_line() {
        let contents = "msgtype,init,16\nmsgdata,init,gflen,u16,\n";
        let char_vec: Vec<char> = contents.chars().collect();
        let mut scanner = scanner::Scanner::new();
        let result: Vec<CSVToken> = scanner.scan(&char_vec);
        let mut parser = parser::Parser::new();
        parser.parse(&result);
        debug_assert_eq!(
            parser.symbol_table.get("init").unwrap().msg_data[0],
            ast::LNMsData::Unsigned16("gflen".to_string(), 2,)
        );
    }

    #[test]
    #[should_panic]
    fn parse_simple_failure_line() {
        let contents = "msgtype,init,16\nmsgdata,init,gflen,u16,\n
        msgdata,init,,gflen\n";
        let char_vec: Vec<char> = contents.chars().collect();
        let mut scanner = scanner::Scanner::new();
        let result: Vec<CSVToken> = scanner.scan(&char_vec);
        let mut parser = parser::Parser::new();
        parser.parse(&result);
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
        let mut parser = parser::Parser::new();
        parser.parse(&result);
        // check bytes line
        debug_assert_eq!(
            parser.symbol_table.get("init").unwrap().msg_data[1],
            ast::LNMsData::BitfieldStream("globalfeatures".to_string(), Some(2),)
        );
        // check TLV line
        debug_assert_eq!(
            parser.symbol_table.get("init").unwrap().tlv_stream[0].tlv_name,
            "networks".to_string()
        );
        debug_assert_eq!(
            parser.symbol_table.get("init").unwrap().tlv_stream[0].tlv_type,
            1
        );
        debug_assert_eq!(
            parser.symbol_table.get("init").unwrap().tlv_stream[0]
                .tlv_data
                .as_ref()
                .unwrap()
                .name,
            "chains".to_string()
        );
        debug_assert_eq!(
            parser.symbol_table.get("init").unwrap().tlv_stream[0]
                .tlv_data
                .as_ref()
                .unwrap()
                .value,
            "chain_hash".to_string()
        );
        debug_assert_eq!(
            parser.symbol_table.get("init").unwrap().tlv_stream[1]
                .tlv_data
                .as_ref()
                .unwrap()
                .name,
            "data".to_string()
        );
        debug_assert_eq!(
            parser.symbol_table.get("init").unwrap().tlv_stream[1]
                .tlv_data
                .as_ref()
                .unwrap()
                .value,
            "byte".to_string()
        );
    }
}
