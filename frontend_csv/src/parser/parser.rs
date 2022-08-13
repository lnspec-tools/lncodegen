//! Core Parser implementation for the csv tokens
use std::collections::BTreeMap;
use std::vec::Vec;

use crate::parser::ast::LNMsg;
use crate::scanner::token::{CSVToken, CSVTokenType};

struct Parser {
    simple_table: BTreeMap<String, LNMsg>,
    pos: usize,
}

impl Parser {
    /// Build a new parser
    fn new() -> Self {
        return Parser {
            pos: 0,
            simple_table: BTreeMap::new(),
        };
    }

    /// Take the element in the current position of the stream
    fn peek<'a>(&self, tokens: &'a Vec<CSVToken>) -> &'a CSVToken {
        return &tokens[self.pos];
    }

    /// Parse a message type line of the csv file, where the format looks like
    /// the following one:
    ///
    /// `msgtype,init,16`
    fn parse_msg_typ(&mut self, tokens: &Vec<CSVToken>) {}

    //// Parse a message data entry
    fn parse_msg_data(&mut self, tokens: &Vec<CSVToken>) {}

    /// PArse a TLV type declaration
    fn parse_tlv_typ(&mut self, tokens: &Vec<CSVToken>) {}

    /// parse TLV data entry
    fn parse_tlv_data(&mut self, tokens: &Vec<CSVToken>) {}

    /// Entry point of the parser!
    fn parse(&mut self, tokens: &Vec<CSVToken>) {
        while self.peek(&tokens).ty != CSVTokenType::EOF {
            match self.peek(&tokens).ty {
                CSVTokenType::MsgTy => self.parse_msg_typ(&tokens),
                CSVTokenType::MsgData => self.parse_msg_data(&tokens),
                CSVTokenType::TlvType => self.parse_tlv_typ(&tokens),
                CSVTokenType::TlvData => self.parse_tlv_data(tokens),
                _ => panic!("Unknown Token {:?}", self.peek(&tokens)),
            }
        }
    }
}
