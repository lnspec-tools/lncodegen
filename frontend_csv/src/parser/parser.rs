//! Core Parser implementation for the csv tokens
use std::collections::BTreeMap;
use std::vec::Vec;

use crate::parser::ast::LNMsData;
use crate::parser::ast::LNMsg;
use crate::parser::ast::LNTlvData;
use crate::parser::ast::LNTlvType;
use crate::scanner::token::{CSVToken, CSVTokenType};

#[derive(Debug)]
pub struct Parser {
    pub symbol_table: BTreeMap<String, LNMsg>,
    pos: usize,
    lnmsg_buffer: Option<LNMsg>,
    lntlv_buffer: Option<LNTlvType>,
}

impl Parser {
    /// Build a new parser
    pub fn new() -> Self {
        return Parser {
            pos: 0,
            symbol_table: BTreeMap::new(),
            lnmsg_buffer: None,
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
        assert_eq!(self.advance(tokens).ty, CSVTokenType::MsgTy);
        let msg_name = self.advance(&tokens);
        let msg_type = self.advance(&tokens);
        match msg_type.ty {
            CSVTokenType::Number => LNMsg {
                msg_typ: msg_type.val.parse::<u64>().unwrap(),
                msg_name: msg_name.val.to_owned(),
                msg_data: Vec::new(),
                tlv_stream: Vec::new(),
            },
            _ => panic!("Unknown Token {:?}", self.peek(&tokens)),
        }
    }

    // This method is to get the lenght of the integer type for bytes field
    // for example:
    // msgdata,init,flen,u16,
    // msgdata,init,features,byte,flen
    // flen is the length of the bytes field which is the type u16 which is 2 bytes
    // so for features field the lenght is 2 bytes and this method map it together.
    fn get_byte_length(&self, keyword: String) -> u64 {
        let tmp_vec = &self.lnmsg_buffer.as_ref().unwrap().msg_data;

        for i in 0..tmp_vec.len() {
            match tmp_vec[i] {
                LNMsData::Unsigned64(ref name, _) => {
                    if name == &keyword {
                        return 8;
                    }
                }
                LNMsData::Unsigned32(ref name, _) => {
                    if name == &keyword {
                        return 4;
                    }
                }
                LNMsData::Unsigned16(ref name, _) => {
                    if name == &keyword {
                        return 2;
                    }
                }
                _ => {
                    continue;
                }
            }
        }
        return 0;
    }

    //// Parse a message data entry
    ///  msgdata,init,globalfeatures,byte,gflen
    ///  msgdata,init,gflen,u16,
    fn parse_msg_data(&mut self, tokens: &Vec<CSVToken>) -> Vec<LNMsData> {
        let mut msg_data = vec![];
        while self.peek(tokens).ty == CSVTokenType::MsgData {
            assert_eq!(self.advance(&tokens).ty, CSVTokenType::MsgData); // consume msgdata tok.
            let msg_data_name = self.advance(&tokens).val.to_owned();
            println!(" Msg Name: {:?}", msg_data_name);

            let msg_data_type = self.advance(&tokens);
            println!("Msg: type {:?}", msg_data_type);
            print!("\n msg type {:?} \n", msg_data_type.ty);

            let msg = match msg_data_type.ty {
                CSVTokenType::U16 => LNMsData::Unsigned16(msg_data_name, 2),
                CSVTokenType::U32 => LNMsData::Unsigned32(msg_data_name, 4),
                CSVTokenType::U64 => LNMsData::Unsigned64(msg_data_name, 8),
                CSVTokenType::ChainHash => {
                    let msg_val = self.advance(&tokens);
                    LNMsData::ChainHash(msg_data_name, msg_val.val.to_owned())
                }
                CSVTokenType::Byte => {
                    let byte_name = self.advance(&tokens).val.to_string();
                    print!("\n bytes name {:?}\n", byte_name);
                    let msg_val = self.get_byte_length(byte_name);
                    LNMsData::BitfieldStream(msg_data_name, Some(msg_val))
                }
                // TODO: this is correct? we must generalize here otherwise the parser
                // start to have no sense.
                CSVTokenType::LiteralString => {
                    LNMsData::TLVinit(msg_data_type.val.to_owned(), msg_data_name)
                }
                _ => panic!("Unknown Token {:?}", msg_data_type),
            };
            msg_data.push(msg);
        }
        msg_data
    }

    /// Parse a TLV type declaration
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
        if self.lntlv_buffer != None {
            self.lnmsg_buffer
                .as_mut()
                .unwrap()
                .tlv_stream
                .push(self.lntlv_buffer.clone().unwrap());
            self.lntlv_buffer = None
        }
    }

    fn is_eof(&self, tokens: &Vec<CSVToken>) -> bool {
        return self.peek(tokens).ty == CSVTokenType::EOF;
    }

    /// Entry point of the parser!
    pub fn parse(&mut self, tokens: &Vec<CSVToken>) {
        while !self.is_eof(tokens) {
            match self.peek(&tokens).ty {
                CSVTokenType::MsgTy => {
                    let mut msg = self.parse_msg_typ(&tokens);
                    assert_eq!(self.peek(tokens).ty, CSVTokenType::MsgData);
                    let mut msg_data = self.parse_msg_data(tokens);
                    msg.msg_data.append(&mut msg_data);
                    self.symbol_table.insert(msg.msg_name.to_owned(), msg);
                }
                CSVTokenType::TlvType => {
                    self.insert_and_reset_tlv();
                    self.parse_tlv_typ(&tokens)
                }
                CSVTokenType::TlvData => self.parse_tlv_data(tokens),
                _ => panic!(
                    "Unknown Token {:?}, parser status {:?}",
                    self.peek(&tokens),
                    self
                ),
            }
        }
        self.insert_and_reset_tlv();
    }
}
