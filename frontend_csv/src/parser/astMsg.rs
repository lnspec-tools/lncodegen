use crate::scanner::token::CSVTokenType;
#[derive(Clone, PartialEq, Debug)]
pub enum AstMsgType {
    Msgtype,
    Msgdata,
    Tlvtype,
    Tlvdata,
    None,
}
pub trait AstMsg {
    fn get_msg_type(&self) -> AstMsgType;
    fn get_values(&self) -> String;
}

pub fn map_csvtoken(v: Vec<CSVTokenType>) -> AstMsgType {
    match v[..] {
        [CSVTokenType::MsgTy, CSVTokenType::LiteralString, CSVTokenType::Number] => {
            AstMsgType::Msgtype
        }
        _ => AstMsgType::None,
    }
}

impl AstMsgType {}
