use std::default;

#[derive(Clone, PartialEq, Debug)]
pub enum CSVTokenType {
    Comma,
    MsgTy,
    MsgData,
    NewLine,
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
    None,
}
#[derive(Clone, PartialEq, Debug)]
pub struct CSVToken {
    pub ty: CSVTokenType,
    pub val: String,
}
