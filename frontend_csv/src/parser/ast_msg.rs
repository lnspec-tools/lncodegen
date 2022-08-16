use crate::scanner::token::CSVTokenType;
use std::fmt::Debug;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum AstMsgLineType {
    Msgtype,
    MsgdataLength,
    MsgDataBytes,
    MsgDataTLVInit,
    Tlvtype,
    Tlvdata,
    TlvByteDataWithDot,
    TvlChainWithDot,
    MsgDataChannelID,
    // temporary place holder for no match
    None,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum AstMsgType {
    Init,
    Warning,
    Error,
    Ping,
    Pong,
    // temporary place holder for no match
    None,
}

#[derive(Clone, PartialEq, Debug)]
pub struct AstMsg {
    pub msg_type: AstMsgType,
    pub msg_val: String,
}

#[derive(Clone, PartialEq, Debug)]
pub struct RawCsv {
    pub line_type: Vec<AstMsgLineType>,
    pub values: Vec<Vec<String>>,
}
pub struct Msg {
    pub raw: RawCsv,
    pub msg_type: Option<AstMsgType>,
    pub msg_val: Vec<Vec<String>>,
    pub msg_name: String,
    pub tlvs_name: Vec<String>,
}

pub trait AstMsgTrait {
    fn get_msg_type(&self) -> Option<AstMsgType>;
    fn get_values(&self) -> Vec<Vec<String>>;
    fn check_valid(&mut self) -> bool;
}

impl Msg {
    pub fn new(lines: &RawCsv) -> Msg {
        return Msg {
            raw: lines.clone(),
            msg_type: None,
            msg_val: lines.values.clone(),
            msg_name: String::new(),
            tlvs_name: Vec::new(),
        };
    }
}

impl AstMsgTrait for Msg {
    fn get_msg_type(&self) -> Option<AstMsgType> {
        return self.msg_type;
    }

    fn get_values(&self) -> Vec<Vec<String>> {
        return self.msg_val.clone();
    }

    // Check if init message is valid
    fn check_valid(&mut self) -> bool {
        let size = self.raw.line_type.len();
        for pos in 0..size {
            if self.raw.line_type[pos] == AstMsgLineType::Msgtype && self.msg_name.is_empty() {
                // checking if interger type matches
                // For future type checking
                // if self.raw[pos].1[2] != "16" {
                //     println!("msgtype must be 16");
                //     return false;
                // }
                // we store msg name in the msg_name for future reference
                self.msg_name = self.raw.values[pos][1].clone();
            }
            // check reference name for msg name
            if [
                AstMsgLineType::MsgdataLength,
                AstMsgLineType::MsgDataBytes,
                AstMsgLineType::MsgDataTLVInit,
            ]
            .contains(&self.raw.line_type[pos])
            {
                if self.raw.values[pos][1] != self.msg_name {
                    println!(
                        "Msg name reference is not correct {} != {}",
                        self.raw.values[pos][1], self.msg_name
                    );
                    return false;
                }
            }
            if self.raw.line_type[pos] == AstMsgLineType::MsgDataTLVInit {
                // we store tlv name in the tlvs_name vector for future reference
                self.tlvs_name.push(self.raw.values[pos][3].clone());
            }
            if [
                AstMsgLineType::Tlvdata,
                AstMsgLineType::Tlvtype,
                AstMsgLineType::TlvByteDataWithDot,
                AstMsgLineType::TvlChainWithDot,
            ]
            .contains(&self.raw.line_type[pos])
            {
                if !self.tlvs_name.contains(&self.raw.values[pos][1]) {
                    println!("TLV name reference is not correct");
                    return false;
                }
            }
        }
        return true;
    }
}

// This function is used to map vector of CSVTokenType to AstMsgType
pub fn map_csvtoken(v: Vec<CSVTokenType>) -> AstMsgLineType {
    // A preifx of CSVtokens for msgDatalength
    // We need this because at the end of the msgDatalength we can have different interger type such as
    // u16, u32, u64
    let test_msgdatalen_vec = [
        CSVTokenType::MsgData,
        CSVTokenType::LiteralString,
        CSVTokenType::LiteralString,
    ];
    match v[..] {
        [CSVTokenType::MsgTy, CSVTokenType::LiteralString, CSVTokenType::Number] => {
            AstMsgLineType::Msgtype
        }
        // Here we match the msgDatalength by combine the the base msgDatalength and the interger type
        // and test all the possible combination to see if it matches
        _ if [CSVTokenType::U16, CSVTokenType::U32, CSVTokenType::U64]
            .iter()
            .any(|item| v == [test_msgdatalen_vec.as_slice(), [*item].as_slice()].concat()) =>
        {
            AstMsgLineType::MsgdataLength
        }
        [CSVTokenType::MsgData, CSVTokenType::LiteralString, CSVTokenType::LiteralString, CSVTokenType::Byte, CSVTokenType::LiteralString] => {
            AstMsgLineType::MsgDataBytes
        }
        [CSVTokenType::MsgData, CSVTokenType::LiteralString, CSVTokenType::Data, CSVTokenType::Byte, CSVTokenType::LiteralString] => {
            AstMsgLineType::MsgDataBytes
        }
        [CSVTokenType::MsgData, CSVTokenType::LiteralString, CSVTokenType::ChannelId, CSVTokenType::ChannelId] => {
            AstMsgLineType::MsgDataChannelID
        }
        // msgdata,init,tlvs,init_tlvs,
        [CSVTokenType::MsgData, CSVTokenType::LiteralString, CSVTokenType::Tlvs, CSVTokenType::LiteralString] => {
            AstMsgLineType::MsgDataTLVInit
        }
        [CSVTokenType::TlvType, CSVTokenType::LiteralString, CSVTokenType::LiteralString, CSVTokenType::Number] => {
            AstMsgLineType::Tlvtype
        }
        [CSVTokenType::TlvData, CSVTokenType::LiteralString, CSVTokenType::LiteralString, CSVTokenType::LiteralString, CSVTokenType::ChainHash, CSVTokenType::Dotdotdot] => {
            AstMsgLineType::TvlChainWithDot
        }
        [CSVTokenType::TlvData, CSVTokenType::LiteralString, CSVTokenType::LiteralString, CSVTokenType::Data, CSVTokenType::Byte, CSVTokenType::Dotdotdot] => {
            AstMsgLineType::TlvByteDataWithDot
        }

        // None type is needed because if lines consist of only one type, it will not match any of the above cases
        // But we cannot fail here, because it could be a valid message as more token are added
        // As an example, the input cosist of {CSVTokenType::Msgty}
        // If we match it with above cases, it will return AstMsgLineType::None
        // As the reading progresses, the token will be added to the vector
        // the input now cosist of {CSVTokenType::MsgTy, CSVTokenType::LiteralString, CSVTokenType::Number}
        // Which will match the Msgtype structure
        // so we return AstMsgLineType::Msgtype
        _ => AstMsgLineType::None,
    }
}

// map sentences to msg type
// for now we don't need to concern with msg type....
pub fn map_line_to_msg(lines: Vec<AstMsgLineType>) -> AstMsgType {
    match lines[..] {
        [AstMsgLineType::Msgtype, AstMsgLineType::MsgdataLength, AstMsgLineType::MsgDataBytes, AstMsgLineType::MsgdataLength, AstMsgLineType::MsgDataBytes, AstMsgLineType::MsgDataTLVInit, AstMsgLineType::Tlvtype, AstMsgLineType::TvlChainWithDot, AstMsgLineType::Tlvtype, AstMsgLineType::TlvByteDataWithDot] => {
            AstMsgType::Init
        }

        // None type is needed because if lines consist of only one type, it will not match any of the above cases
        // But we cannot fail here, because it could be a valid message as more Lines are added
        // As an example, the input cosist of {AstMsgLineType::Msgtype}
        // If we match it with above cases, it will return AstMsgType::None
        // As the reading progresses, the lines will be added to the vector
        // the input now cosist of {AstMsgLineType::Msgtype, AstMsgLineType::MsgdataLength, AstMsgLineType::MsgDataBytes, AstMsgLineType::MsgdataLength, AstMsgLineType::MsgDataBytes, AstMsgLineType::MsgDataTLVInit, AstMsgLineType::Tlvtype, AstMsgLineType::TvlChainWithDot, AstMsgLineType::Tlvtype, AstMsgLineType::TlvByteDataWithDot}
        // Which will match the init message structure
        // so we return AstMsgType::Init
        _ => AstMsgType::None,
    }
}

impl AstMsgType {}
