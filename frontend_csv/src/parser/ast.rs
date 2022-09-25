//! Abstract Syntax Tree implementation
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
    // FIXME: can be encoded as map?
    pub tlv_stream: Vec<LNTlvType>,
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

#[derive(Clone, PartialEq, Debug)]
pub struct LNTlvData {
    pub name: String,
    pub value: String,
}

/// Structure that encode
#[derive(Clone, PartialEq, Debug)]
pub struct LNTlvType {
    pub tlv_type: u64,
    /// Encode the data as free hex and the client
    /// that use the code will decode and encode it
    /// in a particular way.
    pub tls_type: String,
    pub tlv_name: String,
    pub tlv_data: Option<LNTlvData>,
}

impl LNMsg {
    /// Build a new lightning network message with the name
    /// and type provided.
    pub fn new(msg_typ: u64, msg_name: &str) -> Self {
        return LNMsg {
            msg_typ: msg_typ,
            msg_name: msg_name.to_string(),
            msg_data: Vec::new(),
            tlv_stream: Vec::new(),
        };
    }

    pub fn add_msg_data(&mut self, data: &LNMsData) {
        self.msg_data.push(data.clone());
    }
}
