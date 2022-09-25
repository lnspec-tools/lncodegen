//! Core Parser implementation for the csv tokens
use log::trace;
use std::collections::BTreeMap;
use std::vec::Vec;

use crate::parser::ast::LNMsData;
use crate::parser::ast::LNMsg;
use crate::parser::ast::LNTlvData;
use crate::parser::ast::LNTlvType;
use crate::scanner::token::{CSVToken, CSVTokenType};

pub struct Parser {
    pub symbol_table: BTreeMap<String, LNMsg>,
    pos: usize,
    lntlv_buffer: Option<LNTlvType>,
}

impl Parser {
    /// Build a new parser
    pub fn new() -> Self {
        return Parser {
            pos: 0,
            symbol_table: BTreeMap::new(),
            lntlv_buffer: None,
        };
    }

    /// Take the element in the current position of the stream
    fn peek<'a>(&self, tokens: &'a Vec<CSVToken>) -> &'a CSVToken {
        return &tokens[self.pos];
    }

    /// Take the element in the current position of the stream
    /// and increase the position by one
    fn advance<'a>(&mut self, tokens: &'a Vec<CSVToken>) -> &'a CSVToken {
        self.pos += 1;
        return &tokens[self.pos - 1];
    }

    /// Parse a message type line of the csv file, where the format looks like
    /// the following one:
    ///
    /// `msgtype,init,16`
    fn parse_msg_typ(&mut self, tokens: &Vec<CSVToken>) -> LNMsg {
        let msg_name = self.advance(&tokens);
        let msg_type = self.advance(&tokens);
        match msg_type.ty {
            CSVTokenType::Number => LNMsg {
                msg_typ: msg_type.val.parse::<u64>().unwrap(),
                msg_name: msg_name.val.to_string(),
                msg_data: Vec::new(),
                tlv_stream: Vec::new(),
            },
            _ => panic!("Unknown Token {:?}", self.peek(&tokens)),
        }
    }

    /// Parse a message data entry
    ///  msgdata,init,globalfeatures,byte,gflen
    ///  msgdata,init,gflen,u16,
    fn parse_msg_data(&mut self, target_msg: &mut LNMsg, tokens: &Vec<CSVToken>) {
        assert!(self.advance(&tokens).ty == CSVTokenType::MsgData);
        assert!(self.advance(&tokens).val == target_msg.msg_name);

        let token = self.advance(&tokens);
        let msg_data_name = token.val.to_string();
        trace!("Data token after prefix: {:?}", token);
        let token = self.advance(&tokens);
        trace!("Data type after prefix {:?}", token);

        let msg_data = match token.ty {
            CSVTokenType::U16 | CSVTokenType::U32 | CSVTokenType::U64 => {
                LNMsData::Uint(token.val.parse().unwrap())
            }
            CSVTokenType::ChainHash => {
                let msg_val = self.advance(&tokens);
                LNMsData::ChainHash(msg_data_name, msg_val.val.to_owned())
            }
            CSVTokenType::Byte => {
                let byte_name = self.advance(&tokens).val.to_string();
                trace!("bytes name {:?}\n", byte_name);
                LNMsData::BitfieldStream(msg_data_name.clone(), byte_name)
            }
            // FIXME: this is a start point for a tlv stream
            CSVTokenType::LiteralString => LNMsData::TLVinit(token.val.to_string(), msg_data_name),
            _ => panic!("Unknown Token {:?}", token),
        };
        trace!("Append msg data {:?} to msg {:?}", msg_data, target_msg);
        target_msg.add_msg_data(&msg_data);
    }

    /// PArse a TLV type declaration
    fn parse_tlv_typ(&mut self, tokens: &Vec<CSVToken>) {
        // init_tlvs,networks,1
        match self.peek(&tokens).ty {
            CSVTokenType::LiteralString => {
                let tls_name = self.advance(&tokens).val.to_string();
                let tlv_name = self.advance(tokens).val.to_string();
                let tlv_type = self.advance(&tokens).val.parse::<u64>().unwrap();
                self.lntlv_buffer = Some(LNTlvType {
                    tls_type: tls_name,
                    tlv_name: tlv_name,
                    tlv_type: tlv_type,
                    tlv_data: None,
                });
            }
            _ => panic!("Unknown Token {:?}", self.peek(&tokens)),
        }
    }

    // tlvdata,init_tlvs,networks,chains,chain_hash,...
    /// parse TLV data entry
    fn parse_tlv_data(&mut self, tokens: &Vec<CSVToken>) {
        self.advance(&tokens); // advance tls name
        self.advance(&tokens); // advance tlv name

        self.lntlv_buffer.as_mut().unwrap().tlv_data = Some(LNTlvData {
            name: self.advance(&tokens).val.to_string(),
            value: self.advance(&tokens).val.to_string(),
        });
        if self.peek(&tokens).ty == CSVTokenType::Dotdotdot {
            self.advance(tokens);
        }
    }

    fn insert_and_reset_tlv(&mut self) {
        // TO be refactored!
    }

    fn parse_msg<'a>(&mut self, tokens: &Vec<CSVToken>) {
        assert!(self.advance(tokens).ty == CSVTokenType::MsgTy);
        let mut msg_typ = self.parse_msg_typ(tokens);
        loop {
            match self.peek(tokens).ty {
                CSVTokenType::MsgData => self.parse_msg_data(&mut msg_typ, tokens),
                _ => break,
            }
        }
        trace!("Insert message in the symbol table: {:?}", msg_typ);
        self.symbol_table
            .insert(msg_typ.msg_name.clone(), msg_typ.to_owned());
    }

    /// Entry point of the parser!
    pub fn parse(&mut self, tokens: &Vec<CSVToken>) {
        while self.peek(&tokens).ty != CSVTokenType::EOF {
            match self.peek(&tokens).ty {
                CSVTokenType::MsgTy => self.parse_msg(tokens),
                CSVTokenType::TlvType => {
                    self.insert_and_reset_tlv();
                    self.parse_tlv_typ(&tokens);
                }
                CSVTokenType::TlvData => self.parse_tlv_data(tokens),
                _ => panic!("Unknown Token {:?}", self.peek(&tokens)),
            }
        }
        self.insert_and_reset_tlv();
    }
}
