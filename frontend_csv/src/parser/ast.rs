//! Abstract Syntax Tree implementation
use std::collections::HashMap;
use std::fmt::Debug;
use std::vec::Vec;

/// Generic lightning network message
/// with all the information that we need to implement this
#[derive(Clone, PartialEq, Debug)]
pub struct LNMsg {
    pub msg_typ: u64,
    pub msg_name: String,
    pub msg_data: Vec<LNMsData>,
    /// encode the tlv stream.
    pub tlv_stream: HashMap<String, LNTlvRecord>,
}

/// All the Msg Data supported by the LN
#[derive(Clone, PartialEq, Debug)]
pub enum LNMsData {
    Uint(String),
    /// Chain Hash type
    ChainHash(String, String),
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
    pub type_name: String,
    pub type_len: u64,
    pub record_entry: Vec<LNTlvEntry>,
}

impl LNTlvRecord {
    pub fn new(name: &str, len: u64) -> Self {
        LNTlvRecord {
            type_name: name.to_string(),
            type_len: len,
            record_entry: Vec::new(),
        }
    }

    pub fn add_entry(&mut self, name: &str, ty: &str) {
        self.record_entry.push(LNTlvEntry::new(name, ty));
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct LNTlvEntry {
    pub entry_name: String,
    pub entry_ty: String,
}

impl LNTlvEntry {
    pub fn new(name: &str, ty: &str) -> Self {
        LNTlvEntry {
            entry_name: name.to_string(),
            entry_ty: ty.to_string(),
        }
    }
}

impl LNMsg {
    /// Build a new lightning network message with the name
    /// and type provided.
    pub fn new(msg_typ: u64, msg_name: &str) -> Self {
        return LNMsg {
            msg_typ,
            msg_name: msg_name.to_string(),
            msg_data: Vec::new(),
            tlv_stream: HashMap::new(),
        };
    }

    pub fn add_msg_data(&mut self, data: &LNMsData) {
        self.msg_data.push(data.clone());
    }

    pub fn add_tlv_record(&mut self, key: &str, record: &LNTlvRecord) {
        assert!(!self.tlv_stream.contains_key(key));
        self.tlv_stream.insert(key.to_string(), record.to_owned());
    }
}
