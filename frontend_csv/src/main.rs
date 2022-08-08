use frontend_csv::scanner::scanner::Scanner;
use frontend_csv::scanner::token;
use std::fs;
fn main() {
    let path_file = "/home/micky/Desktop/lncodegen.rs/specs/bolt1.csv";
    let contents = fs::read_to_string(path_file).expect("Something went wrong reading the file");
    let char_vec: Vec<char> = contents.chars().collect();
    let mut scanner = Scanner::new(1);
    let result = scanner.scan(&char_vec);
    println!("{}", result.len());
    for c in result {
        match c.ty {
            token::CSVTokenType::Comma => assert_eq!(c.val, ","),
            token::CSVTokenType::MsgTy => assert_eq!(c.val, "msgtype"),
            token::CSVTokenType::MsgData => assert_eq!(c.val, "msgdata"),
            token::CSVTokenType::NewLine => assert_eq!(c.val, "\n"),
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
            token::CSVTokenType::None => continue,
            token::CSVTokenType::Number => continue,
        }
    }
    println!("hello world!");
}
