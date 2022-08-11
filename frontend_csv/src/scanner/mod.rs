pub mod scanner;
/// Scanner implementation!
pub mod token;

#[cfg(test)]
mod test {
    use crate::scanner::scanner;
    use crate::scanner::token;
    use std::fs;
    #[test]
    fn scan_simple_one() {
        let path_file = std::env::var_os("CSV_PATH").unwrap();
        let contents =
            fs::read_to_string(path_file).expect("Something went wrong reading the file");
        let char_vec: Vec<char> = contents.chars().collect();
        let mut scanner = scanner::Scanner::new(1);
        let result = scanner.scan(&char_vec);
        assert!(result.len() > 0);
        for c in result {
            match c.ty {
                token::CSVTokenType::MsgTy => assert_eq!(c.val, "msgtype"),
                token::CSVTokenType::MsgData => assert_eq!(c.val, "msgdata"),
                token::CSVTokenType::TlvType => assert_eq!(c.val, "tlvtype"),
                token::CSVTokenType::TlvData => assert_eq!(c.val, "tlvdata"),
                token::CSVTokenType::ShortChannelId => assert_eq!(c.val, "short_channel_id"),
                token::CSVTokenType::Sha256 => assert_eq!(c.val, "sha256"),
                token::CSVTokenType::U16 => assert_eq!(c.val, "u16"),
                token::CSVTokenType::U32 => assert_eq!(c.val, "u32"),
                token::CSVTokenType::U64 => assert_eq!(c.val, "u64"),
                token::CSVTokenType::ChannelId => assert_eq!(c.val, "channel_id"),
                token::CSVTokenType::Signature => assert_eq!(c.val, "signature"),
                token::CSVTokenType::Point => assert_eq!(c.val, "point"),
                token::CSVTokenType::ChainHash => assert_eq!(c.val, "chain_hash"),
                token::CSVTokenType::Byte => assert_eq!(c.val, "byte"),
                token::CSVTokenType::BigSize => assert_eq!(c.val, "bigsize"),
                token::CSVTokenType::LiteralString => continue,
                token::CSVTokenType::Number => continue,
                token::CSVTokenType::Tu32 => assert_eq!(c.val, "tu32"),
                token::CSVTokenType::Tu64 => assert_eq!(c.val, "tu64"),
            }
        }
    }
    #[test]
    fn test_one_line() {
        let contents = "msgtype,init,16";
        let char_vec: Vec<char> = contents.chars().collect();
        let mut scanner = scanner::Scanner::new(1);
        let result = scanner.scan(&char_vec);
        assert!(result.len() > 0);
        let expected = vec![
            token::CSVToken {
                ty: token::CSVTokenType::MsgTy,
                val: "msgtype".to_string(),
            },
            token::CSVToken {
                ty: token::CSVTokenType::LiteralString,
                val: "init".to_string(),
            },
            token::CSVToken {
                ty: token::CSVTokenType::Number,
                val: "16".to_string(),
            },
        ];
        for c in 0..expected.len() - 1 {
            assert!(result[c].val == expected[c].val);
            debug_assert_eq!(result[c].ty, expected[c].ty);
        }
    }
    #[test]
    fn test_middle_line() {
        let path_file = std::env::var_os("CSV_PATH").unwrap();
        let contents =
            fs::read_to_string(path_file).expect("Something went wrong reading the file");
        let char_vec: Vec<char> = contents.chars().collect();
        let mut scanner = scanner::Scanner::new(1);
        let result = scanner.scan(&char_vec);
        assert!(result.len() > 0);
        let expected = vec![
            token::CSVToken {
                ty: token::CSVTokenType::MsgTy,
                val: "msgtype".to_string(),
            },
            token::CSVToken {
                ty: token::CSVTokenType::LiteralString,
                val: "error".to_string(),
            },
            token::CSVToken {
                ty: token::CSVTokenType::Number,
                val: "17".to_string(),
            },
        ];
        for c in 0..expected.len() - 1 {
            debug_assert_eq!(result[c + 45].val, expected[c].val);
            debug_assert_eq!(result[c + 45].ty, expected[c].ty);
        }
    }
    #[test]
    fn test_last_line() {
        let path_file = std::env::var_os("CSV_PATH").unwrap();
        let contents =
            fs::read_to_string(path_file).expect("Something went wrong reading the file");
        let char_vec: Vec<char> = contents.chars().collect();
        let mut scanner = scanner::Scanner::new(1);
        let mut result = scanner.scan(&char_vec);
        result.reverse();
        assert!(result.len() > 0);
        let expected = vec![
            token::CSVToken {
                ty: token::CSVTokenType::Tu32,
                val: "tu32".to_string(),
            },
            token::CSVToken {
                ty: token::CSVTokenType::LiteralString,
                val: "cltv_expiry".to_string(),
            },
            token::CSVToken {
                ty: token::CSVTokenType::LiteralString,
                val: "tlv2".to_string(),
            },
            token::CSVToken {
                ty: token::CSVTokenType::LiteralString,
                val: "n2".to_string(),
            },
            token::CSVToken {
                ty: token::CSVTokenType::TlvData,
                val: "tlvdata".to_string(),
            },
        ];
        for c in 0..expected.len() - 1 {
            debug_assert_eq!(result[c].val, expected[c].val);
            debug_assert_eq!(result[c].ty, expected[c].ty);
        }
    }
}
