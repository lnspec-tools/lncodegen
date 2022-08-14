use super::ast_msg;
use super::ast_msg::AstMsgLineType;
use super::ast_msg::AstMsgTrait;
use super::ast_msg::AstMsgType;
use super::ast_msg::InitMsg;
use crate::scanner::token::CSVToken;
use crate::scanner::token::CSVTokenType;
/// Core implementation of the parser

pub struct Parser {
    pub source: Vec<CSVToken>,
    pub buffer: Vec<CSVToken>,
    pub lines: Vec<(AstMsgLineType, Vec<String>)>,
}

impl Parser {
    pub fn new(source: Vec<CSVToken>) -> Parser {
        return Parser {
            source: source,
            buffer: Vec::new(),
            lines: Vec::new(),
        };
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

    // method for checking if lines is completed or not
    // if yes add to lines, clear the buffer and return true
    fn match_lines(&mut self) -> bool {
        let token_type_buffer = self.buffer.clone().into_iter().map(|p| p.ty).collect();
        let token_val_buffer: Vec<String> =
            self.buffer.clone().into_iter().map(|p| p.val).collect();
        let tmp = ast_msg::map_csvtoken(token_type_buffer);
        if tmp != AstMsgLineType::None {
            self.lines
                .push((tmp.clone(), token_val_buffer.to_owned().clone()));
            self.buffer.clear();
            return true;
        }
        return false;
    }

    // parsing method for Number type
    // for now only LiteralString is possible following the Number type
    fn parse_number_token(&mut self) {
        if self.match_lines() {
            return;
        }
        let token = self.pop_source();
        match token.ty {
            CSVTokenType::LiteralString => {
                self.buffer.push(token);
                self.parse_literal_token();
            }
            _ => {
                panic!("unexpected token type");
            }
        }
    }

    // We dont check for matchline here
    // because tlvs type is never at the end of a line
    fn parse_tlvs_token(&mut self) {
        let token = self.pop_source();
        self.buffer.push(token.clone());
        match token.ty {
            CSVTokenType::LiteralString => {
                self.parse_literal_token();
            }
            _ => {
                panic!("unexpected token type");
            }
        }
    }

    // For now uint token appear only at the end of the line
    fn parse_uint_token(&mut self) {
        if self.match_lines() {
            return;
        } else {
            panic!("This should be an end token");
        }
    }

    // For now ... token appear only at the end of the line
    fn parse_dotdotdot_token(&mut self) {
        if self.match_lines() {
            return;
        } else {
            panic!("This should be an end token");
        }
    }

    // only string or ... following bytes
    fn parse_byte_token(&mut self) {
        let token = self.pop_source();
        match token.ty {
            CSVTokenType::LiteralString => {
                self.buffer.push(token);
                self.parse_literal_token();
            }
            CSVTokenType::Dotdotdot => {
                self.buffer.push(token);
                self.parse_dotdotdot_token();
            }
            _ => {
                panic!("unexpected token type");
            }
        }
    }

    // only ... following chainhash
    fn parse_chainhash_token(&mut self) {
        let token = self.pop_source();
        match token.ty {
            CSVTokenType::Dotdotdot => {
                self.buffer.push(token);
                self.parse_dotdotdot_token();
            }
            _ => {
                panic!("unexpected token type");
            }
        }
    }

    // literal string has many possiblity
    fn parse_literal_token(&mut self) {
        if self.match_lines() {
            return;
        }
        let token = self.pop_source();
        self.buffer.push(token.clone());
        match token.ty {
            CSVTokenType::LiteralString => {
                self.parse_literal_token();
            }
            CSVTokenType::Number => {
                self.parse_number_token();
            }
            CSVTokenType::Tlvs => {
                self.parse_tlvs_token();
            }
            CSVTokenType::Byte => {
                self.parse_byte_token();
            }
            CSVTokenType::U16 | CSVTokenType::U32 | CSVTokenType::U64 => {
                self.parse_uint_token();
            }
            CSVTokenType::ChainHash => {
                self.parse_chainhash_token();
            }
            _ => {
                panic!("unexpected token type");
            }
        }
    }

    // all starting token is follow by a string
    fn parse_starting_token(&mut self) {
        let token = self.pop_source();
        match token.ty {
            CSVTokenType::LiteralString => {
                self.buffer.push(token);
                self.parse_literal_token();
            }
            _ => {}
        }
    }

    // helper function for poping front item
    fn pop_source(&mut self) -> CSVToken {
        let token = self.source[0].clone();
        self.source.remove(0);
        return token;
    }

    // parsing recurisve as there only could 3 starting CSVTokenType
    // main entry point for parsing
    pub fn parse_recurisve(&mut self) {
        while !self.source.is_empty() {
            let token = self.pop_source();
            match token.ty {
                CSVTokenType::MsgTy
                | CSVTokenType::MsgData
                | CSVTokenType::TlvData
                | CSVTokenType::TlvType => {
                    self.buffer.push(token);
                    print!("calling parse_literal_token\n");
                    self.parse_starting_token();
                }
                CSVTokenType::EOF => {
                    return;
                }
                _ => {
                    panic!("{:?} unexpected token type", token.ty);
                }
            }
        }
    }
}
