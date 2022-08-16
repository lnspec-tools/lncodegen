use super::ast_msg;
use super::ast_msg::AstMsgLineType;
use super::ast_msg::AstMsgTrait;
use super::ast_msg::RawCsv;
use crate::scanner::token::CSVToken;
use crate::scanner::token::CSVTokenType;
/// Core implementation of the parser

pub struct Parser {
    pub source: Vec<CSVToken>,
    pub buffer: Vec<CSVToken>,
    pub lines: Vec<(AstMsgLineType, Vec<String>)>,
    pub lines_buffer: Vec<(AstMsgLineType, Vec<String>)>,
    pub messages: Vec<Box<dyn AstMsgTrait>>,
}

impl Parser {
    pub fn new(source: Vec<CSVToken>) -> Parser {
        return Parser {
            source: source,
            buffer: Vec::new(),
            lines: Vec::new(),
            messages: Vec::new(),
            lines_buffer: Vec::new(),
        };
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

    fn parse_data_token(&mut self) {
        let token = self.pop_source();
        match token.ty {
            CSVTokenType::Byte => {
                self.buffer.push(token);
                self.parse_byte_token();
            }
            _ => {
                panic!("unexpected token type");
            }
        }
    }

    fn parse_channel_id_token(&mut self) {
        if self.match_lines() {
            return;
        }
        let token = self.pop_source();
        match token.ty {
            CSVTokenType::ChannelId => {
                self.buffer.push(token);
                self.parse_channel_id_token();
            }
            CSVTokenType::MsgData => {
                self.buffer.push(token);
                self.parse_msg_data_line();
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
            CSVTokenType::Data => {
                self.parse_data_token();
            }
            CSVTokenType::ChannelId => {
                self.parse_channel_id_token();
            }
            _ => {
                panic!("{:?} unexpected token type", token.ty);
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
            _ => panic!("{:?} unexpected token type {:?}", token.ty, token.val),
        }
    }

    // helper function for poping front item
    fn pop_source(&mut self) -> CSVToken {
        let token = self.source[0].clone();
        self.source.remove(0);
        return token;
    }

    fn pop_line(&mut self) -> (AstMsgLineType, Vec<String>) {
        let line = self.lines[0].clone();
        self.lines.remove(0);
        return line;
    }

    fn peek_line(&mut self) -> (AstMsgLineType, Vec<String>) {
        if !self.lines.is_empty() {
            let line = self.lines[0].clone();
            return line;
        }
        return (AstMsgLineType::None, Vec::new());
    }

    // parsing recurisve as there only could 3 starting CSVTokenType
    // main entry point for parsing
    pub fn parse_recurisve_tokens(&mut self) {
        while !self.source.is_empty() {
            let token = self.pop_source();
            match token.ty {
                CSVTokenType::MsgTy
                | CSVTokenType::MsgData
                | CSVTokenType::TlvData
                | CSVTokenType::TlvType => {
                    self.buffer.push(token);
                    self.parse_starting_token();
                }
                CSVTokenType::EOF => {
                    return;
                }
                _ => {
                    panic!("{:?} unexpected token type {:?}", token.ty, token.val);
                }
            }
        }
    }

    fn parse_tlv_data_line(&mut self) {
        let mut line = self.peek_line();
        match line.0 {
            AstMsgLineType::Msgtype | AstMsgLineType::None => {
                return;
            }
            AstMsgLineType::Tlvtype => {
                line = self.pop_line();
                self.lines_buffer.push(line);
                self.parse_tlv_type_line();
            }
            _ => {
                panic!("{:?} unexpected line type", line.0);
            }
        }
    }

    fn parse_tlv_type_line(&mut self) {
        let line = self.pop_line();
        match line.0 {
            AstMsgLineType::Tlvdata
            | AstMsgLineType::TlvByteDataWithDot
            | AstMsgLineType::TvlChainWithDot => {
                self.lines_buffer.push(line);
                self.parse_tlv_data_line();
            }
            _ => {
                panic!("{:?} unexpected line type", line.0);
            }
        }
    }

    fn parse_tlv_init_line(&mut self) {
        let line = self.pop_line();
        match line.0 {
            AstMsgLineType::Tlvtype => {
                self.lines_buffer.push(line);
                self.parse_tlv_type_line();
            }
            _ => {
                panic!("{:?} unexpected line type", line.0);
            }
        }
    }

    fn parse_byte_line(&mut self) {
        let mut line = self.peek_line();
        match line.0 {
            AstMsgLineType::Msgtype | AstMsgLineType::None => {
                return;
            }
            AstMsgLineType::MsgdataLength => {
                line = self.pop_line();
                self.lines_buffer.push(line);
                self.parse_msg_data_line();
            }
            AstMsgLineType::MsgDataTLVInit => {
                line = self.pop_line();
                self.lines_buffer.push(line);
                self.parse_tlv_init_line();
            }
            _ => {
                panic!("unexpected line type");
            }
        }
    }

    fn parse_msg_data_line(&mut self) {
        let mut line = self.peek_line();
        match line.0 {
            AstMsgLineType::Msgtype | AstMsgLineType::None => {
                return;
            }
            AstMsgLineType::MsgdataLength => {
                line = self.pop_line();
                self.lines_buffer.push(line);
                self.parse_msg_data_line();
            }
            AstMsgLineType::MsgDataBytes => {
                line = self.pop_line();
                self.lines_buffer.push(line);
                self.parse_byte_line();
            }
            _ => {
                panic!("{:?} unexpected line type", line.0);
            }
        }
    }

    fn parse_starting_line(&mut self) {
        let line = self.pop_line();
        match line.0 {
            AstMsgLineType::MsgdataLength
            | AstMsgLineType::MsgDataBytes
            | AstMsgLineType::MsgDataChannelID => {
                self.lines_buffer.push(line);
                self.parse_msg_data_line();
            }
            _ => {
                panic!("{:?} unexpected line type", line.0);
            }
        }
    }

    pub fn parse_recurisve_lines(&mut self) {
        while !self.lines.is_empty() {
            let line = self.pop_line();
            match line.0 {
                AstMsgLineType::Msgtype => {
                    self.lines_buffer.push(line);
                    self.parse_starting_line();
                }
                _ => {
                    panic!("{:?} unexpected Line type", line.0);
                }
            }
            let raw_csv = RawCsv {
                values: self.lines_buffer.clone().into_iter().map(|p| p.1).collect(),
                line_type: self.lines_buffer.clone().into_iter().map(|p| p.0).collect(),
            };
            let msg = ast_msg::Msg::new(&raw_csv);
            self.messages.push(Box::new(msg));
            self.lines_buffer.clear();
        }
    }
    pub fn parse_recursive(&mut self) {
        self.parse_recurisve_tokens();
        self.parse_recurisve_lines();
    }
}
