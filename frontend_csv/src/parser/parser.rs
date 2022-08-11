use super::astMsg;
use super::astMsg::AstMsgType;
use crate::scanner::token::CSVToken;
use crate::scanner::token::CSVTokenType;
/// Core implementation of the parser

pub struct Parser {}

impl Parser {
    pub fn new() -> Parser {
        return Parser {};
    }

    pub fn parse(&mut self, source: &Vec<CSVToken>) -> Vec<AstMsgType> {
        let mut current_buffer: Vec<CSVTokenType> = Vec::new();
        let mut result: Vec<AstMsgType> = Vec::new();
        let size = source.len();
        for pos in 0..size {
            let token = source[pos].ty.to_owned().clone();
            current_buffer.push(token);
            let tmp = astMsg::map_csvtoken(current_buffer.to_owned());
            println!("{:?}", source[pos].ty);
            if tmp == AstMsgType::Msgtype {
                result.push(tmp.clone());
            }
        }
        return result;
    }
}
