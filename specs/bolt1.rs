// code generated with the lngen, please not edit this file.
use std::io::{Read, Write};

use fundamentals_derive::{DecodeWire, EncodeWire};

use crate::core::{FromWire, ToWire};
use crate::prelude::*;


#[derive(DecodeWire, EncodeWire, Debug, Clone)]
pub struct Error {
    #[msg_type=17]
    pub ty: u16,
    pub channel_id: ChannelId,
    pub data: BitFlag,
}

#[derive(DecodeWire, EncodeWire, Debug, Clone)]
pub struct Init {
    #[msg_type=16]
    pub ty: u16,
    pub globalfeatures: BitFlag,
    pub features: BitFlag,
    pub init_tlvs: Stream,
}

#[derive(DecodeWire, EncodeWire, Debug, Clone)]
pub struct Ping {
    #[msg_type=18]
    pub ty: u16,
    pub num_pong_bytes: u16,
    pub ignored: BitFlag,
}

#[derive(DecodeWire, EncodeWire, Debug, Clone)]
pub struct Pong {
    #[msg_type=19]
    pub ty: u16,
    pub ignored: BitFlag,
}

#[derive(DecodeWire, EncodeWire, Debug, Clone)]
pub struct Warning {
    #[msg_type=1]
    pub ty: u16,
    pub channel_id: ChannelId,
    pub data: BitFlag,
}

