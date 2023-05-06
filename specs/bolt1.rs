// code generated with the lngen, please not edit this file.
use std::io::{Read, Write};

use fundamentals_derive::{DecodeWire, EncodeWire};

use crate::core::{FromWire, ToWire};
use crate::prelude::*;


#[derive(DecodeWire, EncodeWire, Debug)]
pub struct Error {
    #[warn(dead_code)]
    #[msg_type=17]
    ty: u16,
    channel_id: ChannelId,
    data: BitFlag,
}

#[derive(DecodeWire, EncodeWire, Debug)]
pub struct Init {
    #[warn(dead_code)]
    #[msg_type=16]
    ty: u16,
    globalfeatures: BitFlag,
    features: BitFlag,
    init_tlvs: Stream,
}

#[derive(DecodeWire, EncodeWire, Debug)]
pub struct Ping {
    #[warn(dead_code)]
    #[msg_type=18]
    ty: u16,
    ignored: BitFlag,
}

#[derive(DecodeWire, EncodeWire, Debug)]
pub struct Pong {
    #[warn(dead_code)]
    #[msg_type=19]
    ty: u16,
    ignored: BitFlag,
}

#[derive(DecodeWire, EncodeWire, Debug)]
pub struct Warning {
    #[warn(dead_code)]
    #[msg_type=1]
    ty: u16,
    channel_id: ChannelId,
    data: BitFlag,
}

