pub mod scanner;
/// Scanner implementation!
pub mod token;

#[cfg(test)]
mod test {
    use crate::scanner::scanner::Scanner;
    use crate::scanner::token;
    use std::fs;
    #[test]
    fn scan_simple_one() {
        let path_file = std::env::var_os("CSV_PATH").unwrap();
        let contents =
            fs::read_to_string(path_file).expect("Something went wrong reading the file");
        let char_vec: Vec<char> = contents.chars().collect();
        let mut scanner = Scanner { pos: 10 };
        let result = scanner.scan(&char_vec);
        for c in result {
            match c._type {
                token::CSVTokenType::Comma => assert_eq!(c._value, ","),
                token::CSVTokenType::MsgTy => assert_eq!(c._value, "msgtype"),
                token::CSVTokenType::MsgData => assert_eq!(c._value, "msgdata"),
                token::CSVTokenType::NewLine => assert_eq!(c._value, "\n"),
                token::CSVTokenType::TlvType => assert_eq!(c._value, "tlvtype"),
                token::CSVTokenType::TlvData => assert_eq!(c._value, "tlvdata"),
                token::CSVTokenType::ShortChannelId => assert_eq!(c._value, "short_channel_id"),
                token::CSVTokenType::Sha256 => assert_eq!(c._value, "sha256"),
                token::CSVTokenType::U16 => assert_eq!(c._value, "u16"),
                token::CSVTokenType::U32 => assert_eq!(c._value, "u32"),
                token::CSVTokenType::U64 => assert_eq!(c._value, "u64"),
                token::CSVTokenType::ChannelId => assert_eq!(c._value, "channel_id"),
                token::CSVTokenType::Signature => assert_eq!(c._value, "signature"),
                token::CSVTokenType::Point => assert_eq!(c._value, "point"),
                token::CSVTokenType::ChainHash => assert_eq!(c._value, "chain_hash"),
                token::CSVTokenType::Byte => assert_eq!(c._value, "byte"),
                token::CSVTokenType::BigSize => assert_eq!(c._value, "bigsize"),
                token::CSVTokenType::LiteralString => continue,
                token::CSVTokenType::None => continue,
            }
        }
    }
}
