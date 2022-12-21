// code generated with the lncodegen, please not edit this file.
use lnspec_derive::{DecodeWire, EncodeWire};
use crate::core::{FromWire, ToWire, IOError};
use std::io::{Read, Write};


#[derive(DecodeWire, EncodeWire)]
pub struct Error {
    len: u16,
}

#[derive(DecodeWire, EncodeWire)]
pub struct Init {
    gflen: u16,
    flen: u16,
}

#[derive(DecodeWire, EncodeWire)]
pub struct Ping {
    num_pong_bytes: u16,
    byteslen: u16,
}

#[derive(DecodeWire, EncodeWire)]
pub struct Pong {
    byteslen: u16,
}

#[derive(DecodeWire, EncodeWire)]
pub struct Warning {
    len: u16,
}

