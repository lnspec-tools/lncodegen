//! Core Parser implementation for the csv tokens
use log::trace;
use std::collections::BTreeMap;

use crate::parser::ast::LNMsData;
use crate::parser::ast::LNMsg;
use crate::parser::ast::LNTlvEntry;
use crate::parser::ast::LNTlvRecord;
use crate::scanner::token::{CSVToken, CSVTokenType};

use super::ast::LNMsgType;
use super::ast::LNSubType;

pub struct Parser {
    pub symbol_table: BTreeMap<String, LNMsgType>,
    pos: usize,
}

impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
}

impl Parser {
    /// Build a new parser
    pub fn new() -> Self {
        Self {
            pos: 0,
            symbol_table: BTreeMap::new(),
        }
    }

    fn symbol_table_add_lnmsg(&mut self, msg: &LNMsg) {
        self.symbol_table
            .insert(msg.msg_name.to_string(), LNMsgType::Msg(msg.to_owned()));
    }

    fn symbol_table_add_tlv(&mut self, tlv: &LNTlvRecord) {
        self.symbol_table
            .insert(tlv.stream_name.to_string(), LNMsgType::Tlv(tlv.to_owned()));
    }

    /// Take the element in the current position of the stream
    fn peek<'p>(&self, tokens: &'p [CSVToken]) -> &'p CSVToken {
        &tokens[self.pos]
    }

    /// Take the element in the current position of the stream
    /// and increase the position by one
    fn advance<'p>(&mut self, tokens: &'p [CSVToken]) -> &'p CSVToken {
        self.pos += 1;
        &tokens[self.pos - 1]
    }

    /// Return the last element insert inside the token view.
    fn lookup_last<'p>(&self, tokens: &'p [CSVToken]) -> Option<&'p CSVToken> {
        tokens.get(self.pos - 2)
    }

    /// Parse a message type line of the csv file, where the format looks like
    /// the following one:
    ///
    /// `msgtype,init,16`
    fn parse_msg_typ(&mut self, tokens: &[CSVToken]) -> LNMsg {
        let msg_name = self.advance(tokens);
        let msg_type = self.advance(tokens);
        match msg_type.ty {
            CSVTokenType::Number => {
                let mut msg = LNMsg::new(
                    msg_type.val.parse::<u64>().unwrap(),
                    msg_name.val.to_owned().as_str(),
                );
                if self.peek(tokens).val == "gossip_queries" {
                    msg.is_gossip_query = true;
                    let _ = self.advance(tokens);
                }
                msg
            }
            _ => panic!("Unknown Token {:?}", self.peek(tokens)),
        }
    }

    /// peek the next value and check if the next one is not a declaration type,
    /// in the cvs file is the first token in the line.
    fn peek_and_check_if_type_declaration(&self, tokens: &[CSVToken]) -> bool {
        let next = self.peek(tokens);
        matches!(
            next.ty,
            CSVTokenType::MsgData
                | CSVTokenType::MsgTy
                | CSVTokenType::TlvType
                | CSVTokenType::TlvData,
        )
    }

    /// Parse a message data entry
    ///  msgdata,init,globalfeatures,byte,gflen
    ///  msgdata,init,gflen,u16,
    fn parse_msg_data(&mut self, target_msg: &mut LNMsg, tokens: &[CSVToken]) {
        assert!(
            self.peek(tokens).ty == CSVTokenType::MsgData
                || self.peek(tokens).ty == CSVTokenType::SubMsgData
        );
        let _ = self.advance(tokens);
        assert!(
            self.advance(tokens).val == target_msg.msg_name,
            "{}",
            target_msg.msg_name
        );

        let token = self.advance(tokens);
        let msg_data_name = token.val.to_string();
        trace!("Data token after prefix: {:?}", token);
        let token = self.advance(tokens);
        trace!("Data type after prefix {:?}", token);

        let msg_data = match token.ty {
            CSVTokenType::U16 => {
                let tok = self.lookup_last(tokens).unwrap();
                LNMsData::Uint16(tok.val.to_owned())
            }
            CSVTokenType::U32 => {
                let tok = self.lookup_last(tokens).unwrap();
                LNMsData::Uint32(tok.val.to_owned())
            }
            CSVTokenType::U64 => {
                let tok = self.lookup_last(tokens).unwrap();
                LNMsData::Uint64(tok.val.to_owned())
            }
            CSVTokenType::ChainHash => {
                let msg_val = self.lookup_last(tokens).unwrap();
                LNMsData::ChainHash(msg_data_name, msg_val.val.to_owned())
            }
            CSVTokenType::ChannelId => {
                let msg_val = self.lookup_last(tokens).unwrap();
                LNMsData::ChannelId(msg_val.val.to_owned())
            }
            CSVTokenType::Signature => {
                let msg_val = self.lookup_last(tokens).unwrap();
                LNMsData::Signature(msg_val.val.to_owned())
            }
            CSVTokenType::ShortChannelId => {
                let msg_val = self.lookup_last(tokens).unwrap();
                LNMsData::ShortChannelId(msg_val.val.to_owned())
            }
            CSVTokenType::Point => {
                let msg_val = self.lookup_last(tokens).unwrap();
                LNMsData::Point(msg_val.val.to_owned())
            }
            CSVTokenType::Byte => {
                let tok = self.lookup_last(tokens).unwrap();
                let size = if !self.peek_and_check_if_type_declaration(tokens) {
                    self.advance(tokens).val.to_owned()
                } else {
                    "1".to_string()
                };
                trace!("bytes name {:?}\n", tok);
                LNMsData::BitfieldStream(tok.val.to_owned(), size)
            }
            // FIXME: this is a start point for a tlv stream
            CSVTokenType::LiteralString => {
                // be compatible with cln csv
                //
                //  The difference is that cln use some alias type
                //
                // search for u64 alias
                if ["amount_sat"].contains(&token.val.to_string().as_str()) {
                    let tok = self.lookup_last(tokens).unwrap();
                    LNMsData::Uint16(tok.val.to_owned())
                } else if ["u8"].contains(&token.val.as_str()) {
                    let tok = self.lookup_last(tokens).unwrap();
                    let size = if !self.peek_and_check_if_type_declaration(tokens) {
                        self.advance(tokens).val.to_owned()
                    } else {
                        "1".to_string()
                    };
                    trace!("bytes name {:?}\n", tok);
                    LNMsData::BitfieldStream(tok.val.to_owned(), size)
                } else {
                    LNMsData::TLVinit(token.val.to_string(), msg_data_name)
                }
            }
            _ => panic!("Unknown Token {:?}", token),
        };
        /* HACK: the bitfiled struct usually is able to length, so at this point
         * we should trip the last element and put jut the bit field */
        if let Some(LNMsData::Uint16(_)) = target_msg.peek() {
            if let LNMsData::BitfieldStream(_, _) = msg_data {
                trace!("replace the last msg data (u16) with the bitfiled");
                target_msg.replace_last_with(msg_data);
                return;
            }
        }
        trace!("Append msg data {:?} to msg {:?}", msg_data, target_msg);
        target_msg.add_msg_data(&msg_data);
    }

    /// PArse a TLV type declaration
    fn parse_tlv_typ(&mut self, tokens: &[CSVToken]) -> LNTlvRecord {
        assert_eq!(self.advance(tokens).ty, CSVTokenType::TlvType);
        // init_tlvs,networks,1
        match self.peek(tokens).ty {
            CSVTokenType::LiteralString => {
                let tlv_record_name = self.advance(tokens).val.to_string();
                trace!("Record name {:?}", tlv_record_name);
                let tlv_name = self.advance(tokens).val.to_string();
                let tlv_type = self.advance(tokens).val.parse::<u64>().unwrap();
                LNTlvRecord::new(&tlv_record_name, &tlv_name, tlv_type)
            }
            _ => panic!("Unknown Token {:?}", self.peek(tokens)),
        }
    }

    fn peek_and_check_if_dotdot(&self, tokens: &[CSVToken]) -> bool {
        let CSVToken {
            ty: CSVTokenType::Dotdotdot,
            ..
        } = self.peek(tokens) else {
            return false;
        };
        true
    }

    fn parse_tlv_data(&mut self, record: &mut LNTlvRecord, tokens: &[CSVToken]) {
        assert_eq!(self.advance(tokens).ty, CSVTokenType::TlvData);
        assert_eq!(self.advance(tokens).val, record.stream_name);
        assert_eq!(self.advance(tokens).val, record.type_name);
        let tok_name = self.advance(tokens);
        let tok_ty = self.advance(tokens);

        // TODO: we should support the encoding as different field?
        let _is_gossip_query = tok_name.val.starts_with("encoded_");
        let _is_encoding_flag = tok_name.val.eq("encoding_type");

        if self.peek_and_check_if_dotdot(tokens) {
            // FIXME: how we manage this token
            let _ = self.advance(tokens);
        }

        trace!(
            "add tlv record inside the stream {:?} - {:?}",
            tok_name,
            tok_ty
        );

        let entry = LNTlvEntry::new(tok_name.val.as_str(), tok_ty.val.as_str());
        trace!("TLV entry: {:?}", entry);
        //trace!("TLV encoding: {:?}", encoding_typ);
        //entry.encoding = encoding_typ;
        record.add_entry(&entry);
    }

    fn parse_msg(&mut self, tokens: &[CSVToken]) {
        assert!(self.advance(tokens).ty == CSVTokenType::MsgTy);
        let mut msg_typ = self.parse_msg_typ(tokens);
        while let CSVTokenType::MsgData = self.peek(tokens).ty {
            self.parse_msg_data(&mut msg_typ, tokens);
        }
        trace!("Insert message in the symbol table: {:#?}", msg_typ);
        self.symbol_table_add_lnmsg(&msg_typ);
    }

    fn parse_tlv(&mut self, tokens: &[CSVToken]) {
        let mut tlv_typ = self.parse_tlv_typ(tokens);
        trace!("parsing tlv type {:?}", tlv_typ);
        while let CSVTokenType::TlvData = self.peek(tokens).ty {
            self.parse_tlv_data(&mut tlv_typ, tokens);
        }
        self.symbol_table_add_tlv(&tlv_typ);
    }

    fn parse_subtype_ty(&mut self, tokens: &[CSVToken]) -> LNSubType {
        let subtype_name = self.advance(tokens);
        trace!("parsing subtype name {:?}", subtype_name);
        LNSubType::new(subtype_name.val.as_str())
    }

    fn parse_subtype(&mut self, tokens: &[CSVToken]) {
        assert_eq!(self.advance(tokens).ty, CSVTokenType::SubTy);
        let mut typ = self.parse_subtype_ty(tokens);
        trace!("parsing subtype");
        // FIXME: remove this trick and decode a real subtype!
        let mut fake_lnmessage = LNMsg::new(0, typ.ty.as_str());
        while let CSVTokenType::SubMsgData = self.peek(tokens).ty {
            self.parse_msg_data(&mut fake_lnmessage, tokens);
        }
        typ.ty_data = fake_lnmessage.msg_data;
    }

    /// Entry point of the parser!
    pub fn parse(&mut self, tokens: &[CSVToken]) {
        while self.peek(tokens).ty != CSVTokenType::EOF {
            match self.peek(tokens).ty {
                CSVTokenType::MsgTy => self.parse_msg(tokens),
                CSVTokenType::SubTy => self.parse_subtype(tokens),
                CSVTokenType::TlvType => self.parse_tlv(tokens),
                _ => {
                    trace!("parser status {:#?}", self.symbol_table);
                    trace!("look up token {:?}", self.lookup_last(tokens));
                    trace!("previous token {:?}", tokens.get(self.pos - 1).unwrap());
                    panic!("Unknown Token {:?}", self.peek(tokens))
                }
            }
        }
        trace!("Terminating with Parser: {:#?}", self.symbol_table);
    }
}
