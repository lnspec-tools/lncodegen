//! Core Parser implementation for the csv tokens
use log::trace;
use std::collections::BTreeMap;
use std::vec::Vec;

use crate::parser::ast::LNMsData;
use crate::parser::ast::LNMsg;
use crate::parser::ast::LNTlvRecord;
use crate::scanner::token::{CSVToken, CSVTokenType};

pub struct Parser {
    pub symbol_table: BTreeMap<String, LNMsg>,
    pos: usize,
}

impl<'p> Parser {
    /// Build a new parser
    pub fn new() -> Self {
        return Parser {
            pos: 0,
            symbol_table: BTreeMap::new(),
        };
    }

    /// Take the element in the current position of the stream
    fn peek(&self, tokens: &'p Vec<CSVToken>) -> &'p CSVToken {
        return &tokens[self.pos];
    }

    /// Take the element in the current position of the stream
    /// and increase the position by one
    fn advance(&mut self, tokens: &'p Vec<CSVToken>) -> &'p CSVToken {
        self.pos += 1;
        return &tokens[self.pos - 1];
    }

    /// Parse a message type line of the csv file, where the format looks like
    /// the following one:
    ///
    /// `msgtype,init,16`
    fn parse_msg_typ(&mut self, tokens: &'p Vec<CSVToken>) -> LNMsg {
        let msg_name = self.advance(&tokens);
        let msg_type = self.advance(&tokens);
        match msg_type.ty {
            CSVTokenType::Number => LNMsg::new(
                msg_type.val.parse::<u64>().unwrap(),
                msg_name.val.to_owned().as_str(),
            ),
            _ => panic!("Unknown Token {:?}", self.peek(&tokens)),
        }
    }

    /// Parse a message data entry
    ///  msgdata,init,globalfeatures,byte,gflen
    ///  msgdata,init,gflen,u16,
    fn parse_msg_data(&mut self, target_msg: &mut LNMsg, tokens: &'p Vec<CSVToken>) {
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
    fn parse_tlv_typ(&mut self, tokens: &'p Vec<CSVToken>) -> LNTlvRecord {
        // init_tlvs,networks,1
        match self.peek(&tokens).ty {
            CSVTokenType::LiteralString => {
                let _ = self.advance(&tokens).val.to_string();
                let tlv_name = self.advance(tokens).val.to_string();
                let tlv_type = self.advance(&tokens).val.parse::<u64>().unwrap();
                LNTlvRecord::new(&tlv_name.as_str(), tlv_type)
            }
            _ => panic!("Unknown Token {:?}", self.peek(&tokens)),
        }
    }

    fn parse_tlv_data(&mut self, record: &mut LNTlvRecord, tokens: &'p Vec<CSVToken>) {
        assert_eq!(self.advance(tokens).ty, CSVTokenType::TlvData);
        assert_eq!(self.advance(tokens).val, record.type_name);
        let tok_name = self.advance(tokens);
        let tok_ty = self.advance(tokens);

        if let CSVToken {
            ty: CSVTokenType::Dotdotdot,
            ..
        } = self.peek(tokens)
        {
            // FIXME: how we manage this token
            let _ = self.advance(tokens);
        }
        record.add_entry(tok_name.val.as_str(), tok_ty.val.as_str());
    }

    fn parse_msg(&mut self, tokens: &'p Vec<CSVToken>) {
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

    fn parse_tlv(&mut self, tokens: &'p Vec<CSVToken>) {
        let mut tlv_typ = self.parse_tlv_typ(tokens);
        for _ in 0..tlv_typ.type_len {
            self.parse_tlv_data(&mut tlv_typ, tokens);
        }
    }

    /// Entry point of the parser!
    pub fn parse(&mut self, tokens: &'p Vec<CSVToken>) {
        while self.peek(&tokens).ty != CSVTokenType::EOF {
            match self.peek(&tokens).ty {
                CSVTokenType::MsgTy => self.parse_msg(tokens),
                CSVTokenType::TlvType => self.parse_tlv(&tokens),
                _ => panic!("Unknown Token {:?}", self.peek(&tokens)),
            }
        }
    }
}
