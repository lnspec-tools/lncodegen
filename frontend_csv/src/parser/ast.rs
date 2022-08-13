//! Abstract Syntax Tree implementation
use std::vec::Vec;

/// Generic lightning network message
/// with all the information that we need to implement this
pub struct LNMsg {
    msg_typ: u64,
    msg_name: String,
    msg_data: Vec<LNMsData>,
    /// encode the tlv stream.
    // FIXME: can be encoded as map?
    tlv_stream: Vec<LNTlvType>,
}

/// All the Msg Data supported by the LN
pub enum LNMsData {
    Unsigned64(String, u64),
    Unsigned32(String, u32),
    Unsigned16(String, u16),
    /// Chain Hash type
    ChainHash(String, String),
    /// The array can be bounded or we can read till the EOF
    BitfieldStream(String, Option<u64>),
}

/// Structure that encode
pub struct LNTlvType {
    tlv_typ: u64,
    /// Encode the data as free hex and the client
    /// that use the code will decode and encode it
    /// in a particular way.
    tls_type: String,
}

impl LNMsg {
    /// Build a new lightning network message with the name
    /// and type provided.
    fn new(msg_typ: u64, msg_name: &str) -> Self {
        return LNMsg {
            msg_typ: msg_typ,
            msg_name: msg_name.to_string(),
            msg_data: Vec::new(),
            tlv_stream: Vec::new(),
        };
    }
}
