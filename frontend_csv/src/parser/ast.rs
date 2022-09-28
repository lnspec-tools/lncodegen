//! Abstract Syntax Tree implementation
use std::fmt::Debug;
use std::vec::Vec;

#[derive(Clone, PartialEq, Debug)]
pub enum LNMsgType {
    Msg(LNMsg),
    Tlv(LNTlvRecord),
    SubType(LNSubType),
}

/// Generic lightning network message
/// with all the information that we need to implement this
#[derive(Clone, PartialEq, Debug)]
pub struct LNMsg {
    pub msg_typ: u64,
    pub msg_name: String,
    pub msg_data: Vec<LNMsData>,
    pub is_gossip_query: bool,
}

/// All the Msg Data supported by the LN
#[derive(Clone, PartialEq, Debug)]
pub enum LNMsData {
    Uint16(String),
    Uint32(String),
    Uint64(String),
    /// Chain Hash type
    ChainHash(String, String),
    /// Channel id
    ChannelId(String),
    Signature(String),
    ShortChannelId(String),
    Point(String),
    /// The array can be bounded or we can read till the EOF
    BitfieldStream(String, String),
    TLVinit(String, String),
}

///
/// A tlv_record represents a single field, encoded in the form:
/// `[bigsize: type]`
/// `[bigsize: length]`
/// `[length: value]`
#[derive(Clone, PartialEq, Debug)]
pub struct LNTlvRecord {
    pub stream_name: String,
    pub type_name: String,
    pub type_len: u64,
    pub record_entry: Vec<LNTlvEntry>,
}

impl LNTlvRecord {
    pub fn new(stream: &str, name: &str, len: u64) -> Self {
        LNTlvRecord {
            stream_name: stream.to_string(),
            type_name: name.to_string(),
            type_len: len,
            record_entry: Vec::new(),
        }
    }

    pub fn add_entry(&mut self, entry: &LNTlvEntry) {
        self.record_entry.push(entry.to_owned());
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct LNTlvEntry {
    pub entry_name: String,
    pub entry_ty: String,
    pub encoding: Option<EncodingType>,
}

impl LNTlvEntry {
    pub fn new(name: &str, ty: &str) -> Self {
        LNTlvEntry {
            entry_name: name.to_string(),
            entry_ty: ty.to_string(),
            encoding: None,
        }
    }

    pub fn add_encoding() {}
}

#[derive(Clone, PartialEq, Debug)]
pub struct EncodingType {
    pub ty: String,
    pub size: String,
}

impl LNMsg {
    /// Build a new lightning network message with the name
    /// and type provided.
    pub fn new(msg_typ: u64, msg_name: &str) -> Self {
        return LNMsg {
            msg_typ,
            msg_name: msg_name.to_string(),
            msg_data: Vec::new(),
            is_gossip_query: false,
        };
    }

    pub fn add_msg_data(&mut self, data: &LNMsData) {
        self.msg_data.push(data.clone());
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct LNSubType {
    pub ty: String,
    pub ty_data: Vec<LNMsData>,
}

impl LNSubType {
    /// create a new subtype
    pub fn new(ty: &str) -> Self {
        LNSubType {
            ty: ty.to_string(),
            ty_data: Vec::new(),
        }
    }

    /// add a new msg data
    pub fn add_msg_data(&mut self, data: &LNMsData) {
        self.ty_data.push(data.to_owned())
    }
}
