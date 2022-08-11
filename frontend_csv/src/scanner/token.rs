#[derive(Clone, PartialEq, Debug)]
pub enum CSVTokenType {
    MsgTy,
    MsgData,
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
}
#[derive(Clone, PartialEq, Debug)]
pub struct CSVToken {
    pub ty: CSVTokenType,
    pub val: String,
}
