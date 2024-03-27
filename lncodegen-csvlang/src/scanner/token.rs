use core::fmt;

#[derive(Clone, PartialEq, Debug)]
pub enum CSVTokenType {
    MsgTy,
    SubTy,
    MsgData,
    SubMsgData,
    TlvType,
    TlvData,
    ShortChannelId,
    Sha256,
    U16,
    U32,
    U64,
    ChannelId,
    Signature,
    Point,
    ChainHash,
    Byte,
    BigSize,
    LiteralString,
    Number,
    Tu32,
    Tu64,
    Tlvs,
    Dotdotdot,
    Data,
    EOF,
}

impl fmt::Display for CSVTokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self.to_owned() {
            Self::MsgTy => "msgtype",
            Self::SubTy => "subtype",
            Self::SubMsgData => "subtypedata",
            Self::MsgData => "msgdata",
            Self::TlvType => "tlvtype",
            Self::TlvData => "tlvdata",
            Self::ShortChannelId => "short_channel_id",
            Self::Sha256 => "sha256",
            Self::U16 => "u16",
            Self::U32 => "u32",
            Self::U64 => "u64",
            Self::ChannelId => "channel_id",
            Self::Signature => "signature",
            Self::Point => "point",
            Self::ChainHash => "chain_hash",
            Self::Byte => "byte",
            Self::BigSize => "bigsize",
            Self::Tu32 => "tu32",
            Self::Tu64 => "tu64",
            Self::Tlvs => "tlvs",
            Self::Dotdotdot => "...",
            Self::Data => "data",
            Self::Number => "number",
            Self::LiteralString => "str",
            Self::EOF => "eof",
        };
        write!(f, "{name}")
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct CSVToken {
    pub ty: CSVTokenType,
    pub val: String,
    pub code_line: Option<u64>,
}
