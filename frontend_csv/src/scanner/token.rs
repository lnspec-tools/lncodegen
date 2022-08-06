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
    None,
}

pub struct CSVToken {
    pub _type: CSVTokenType,
    pub _value: String,
}
