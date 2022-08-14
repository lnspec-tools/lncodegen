use super::ast_msg;
use super::ast_msg::AstMsgLineType;
use super::ast_msg::AstMsgTrait;
use super::ast_msg::AstMsgType;
use super::ast_msg::InitMsg;
use crate::scanner::token::CSVToken;
use crate::scanner::token::CSVTokenType;
/// Core implementation of the parser

pub struct Parser {}

impl Parser {
    pub fn new() -> Parser {
        return Parser {};
    }

    // parse tokens into lines
    pub fn parse_token(&mut self, source: &Vec<CSVToken>) -> Vec<(AstMsgLineType, Vec<String>)> {
        let mut current_buffer: Vec<CSVTokenType> = Vec::new();
        let mut current_buffer_val: Vec<String> = Vec::new();
        let mut result: Vec<(AstMsgLineType, Vec<String>)> = Vec::new();
        let size = source.len();
        for pos in 0..size {
            let token = source[pos].ty.to_owned().clone();
            let token_val = source[pos].val.to_owned().clone();
            current_buffer.push(token);
            current_buffer_val.push(token_val);
            let tmp = ast_msg::map_csvtoken(current_buffer.to_owned().clone());
            if tmp != AstMsgLineType::None {
                result.push((tmp.clone(), current_buffer_val.to_owned().clone()));
                current_buffer_val.clear();
                current_buffer.clear();
            }
        }
        return result;
    }

    // parse lines into messages
    pub fn parse_lines(
        &mut self,
        source: &Vec<(AstMsgLineType, Vec<String>)>,
    ) -> Vec<Box<dyn AstMsgTrait>> {
        let mut current_buffer: Vec<AstMsgLineType> = Vec::new();
        let mut current_buffer_val: Vec<Vec<String>> = Vec::new();
        let mut result: Vec<Box<dyn AstMsgTrait>> = Vec::new();
        let size = source.len();
        for pos in 0..size {
            current_buffer_val.push(source[pos].1.to_owned().clone());
            current_buffer.push(source[pos].0.to_owned().clone());
            let tmp = ast_msg::map_line_to_msg(current_buffer.clone());
            if tmp == AstMsgType::Init {
                let mut combin_vec: Vec<(AstMsgLineType, Vec<String>)> = Vec::new();
                for i in 0..current_buffer_val.len() {
                    combin_vec.push((current_buffer[i].clone(), current_buffer_val[i].clone()));
                }
                let mut init_msg = InitMsg::new(combin_vec);
                if init_msg.check_valid() {
                    result.push(Box::new(init_msg));
                }
                current_buffer_val.clear();
                current_buffer.clear();
            }
        }
        return result;
    }

    pub fn parse(&mut self, source: &Vec<CSVToken>) -> Vec<Box<dyn AstMsgTrait>> {
        let lines = self.parse_token(source);
        let msgs = self.parse_lines(&lines);
        return msgs;
    }
}
