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

pub struct InitMsg {
    pub raw: Vec<(AstMsgLineType, Vec<String>)>,
    pub msg_type: AstMsgType,
    pub msg_val: String,
    pub msg_name: String,
    pub tlvs_name: Vec<String>,
}

pub trait AstMsgTrait {
    fn get_msg_type(&self) -> AstMsgType;
    fn get_values(&self) -> String;
    fn check_valid(&mut self) -> bool;
}

impl InitMsg {
    pub fn new(lines: Vec<(AstMsgLineType, Vec<String>)>) -> InitMsg {
        return InitMsg {
            raw: lines,
            msg_type: AstMsgType::Init,
            msg_val: String::new(),
            msg_name: String::new(),
            tlvs_name: Vec::new(),
        };
    }
}

impl AstMsgTrait for InitMsg {
    fn get_msg_type(&self) -> AstMsgType {
        return self.msg_type.clone();
    }

    fn get_values(&self) -> String {
        return self.msg_val.clone();
    }

    // Check if init message is valid
    fn check_valid(&mut self) -> bool {
        let size = self.raw.len();
        for pos in 0..size {
            if self.raw[pos].0 == AstMsgLineType::Msgtype && self.msg_name.is_empty() {
                // checking if interger type matches
                if self.raw[pos].1[2] != "16" {
                    println!("msgtype must be 16");
                    return false;
                }
                // we store msg name in the msg_name for future reference
                self.msg_name = self.raw[pos].1[1].clone();
            }
            if [
                AstMsgLineType::MsgdataLength,
                AstMsgLineType::MsgDataBytes,
                AstMsgLineType::MsgDataTLVInit,
            ]
            .contains(&self.raw[pos].0)
            {
                if self.raw[pos].1[1] != self.msg_name {
                    println!(
                        "Msg name reference is not correct {} != {}",
                        self.raw[pos].1[1], self.msg_name
                    );
                    return false;
                }
            }
            if self.raw[pos].0 == AstMsgLineType::MsgDataTLVInit {
                // we store tlv name in the tlvs_name vector for future reference
                self.tlvs_name.push(self.raw[pos].1[3].clone());
            }
            if [
                AstMsgLineType::Tlvdata,
                AstMsgLineType::Tlvtype,
                AstMsgLineType::TlvByteDataWithDot,
                AstMsgLineType::TvlChainWithDot,
            ]
            .contains(&self.raw[pos].0)
            {
                if !self.tlvs_name.contains(&self.raw[pos].1[1]) {
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
