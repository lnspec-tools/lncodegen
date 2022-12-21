// code generated with the lncodegen, please not edit this file.
use lnspec_derive::{DecodeWire, EncodeWire};
use crate::core::{FromWire, ToWire, IOError};
use std::io::{Read, Write};
use crate::types::{ChainHash, ChannelId, Point, Signature};


#[derive(DecodeWire, EncodeWire)]
pub struct Error {
    #[type=17]
    ty: u16,
    channel_id: ChannelId,
    len: u16,
}

#[derive(DecodeWire, EncodeWire)]
pub struct Init {
    #[type=16]
    ty: u16,
    gflen: u16,
    flen: u16,
}

#[derive(DecodeWire, EncodeWire)]
pub struct Ping {
    #[type=18]
    ty: u16,
    num_pong_bytes: u16,
    byteslen: u16,
}

#[derive(DecodeWire, EncodeWire)]
pub struct Pong {
    #[type=19]
    ty: u16,
    byteslen: u16,
}

#[derive(DecodeWire, EncodeWire)]
pub struct Warning {
    #[type=1]
    ty: u16,
    channel_id: ChannelId,
    len: u16,
}

