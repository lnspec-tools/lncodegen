// code generated with the lncodegen, please not edit this file.
use lnspec_derive::{DecodeWire, EncodeWire};
use crate::core::{FromWire, ToWire, IOError};
use std::io::{Read, Write};
use crate::types::{ChainHash, ChannelId, Point, Signature};


#[derive(DecodeWire, EncodeWire)]
pub struct AnnouncementSignatures {
    #[msg_type=259]
    ty: u16,
    channel_id: ChannelId,
    short_channel_id: ShortChannelId,
    node_signature: Signature,
    bitcoin_signature: Signature,
}

#[derive(DecodeWire, EncodeWire)]
pub struct ChannelAnnouncement {
    #[msg_type=256]
    ty: u16,
    node_signature_1: Signature,
    node_signature_2: Signature,
    bitcoin_signature_1: Signature,
    bitcoin_signature_2: Signature,
    len: u16,
    chain_hash: ChainHash,
    short_channel_id: ShortChannelId,
    node_id_1: Point,
    node_id_2: Point,
    bitcoin_key_1: Point,
    bitcoin_key_2: Point,
}

#[derive(DecodeWire, EncodeWire)]
pub struct ChannelUpdate {
    #[msg_type=258]
    ty: u16,
    signature: Signature,
    chain_hash: ChainHash,
    short_channel_id: ShortChannelId,
    timestamp: u32,
    cltv_expiry_delta: u16,
    htlc_minimum_msat: u64,
    fee_base_msat: u32,
    fee_proportional_millionths: u32,
    htlc_maximum_msat: u64,
}

#[derive(DecodeWire, EncodeWire)]
pub struct GossipTimestampFilter {
    #[msg_type=265]
    ty: u16,
    chain_hash: ChainHash,
    first_timestamp: u32,
    timestamp_range: u32,
}

#[derive(DecodeWire, EncodeWire)]
pub struct NodeAnnouncement {
    #[msg_type=257]
    ty: u16,
    signature: Signature,
    flen: u16,
    timestamp: u32,
    node_id: Point,
    addrlen: u16,
}

#[derive(DecodeWire, EncodeWire)]
pub struct QueryChannelRange {
    #[msg_type=263]
    ty: u16,
    chain_hash: ChainHash,
    first_blocknum: u32,
    number_of_blocks: u32,
}

#[derive(DecodeWire, EncodeWire)]
pub struct QueryShortChannelIds {
    #[msg_type=261]
    ty: u16,
    chain_hash: ChainHash,
    len: u16,
}

#[derive(DecodeWire, EncodeWire)]
pub struct ReplyChannelRange {
    #[msg_type=264]
    ty: u16,
    chain_hash: ChainHash,
    first_blocknum: u32,
    number_of_blocks: u32,
    len: u16,
}

#[derive(DecodeWire, EncodeWire)]
pub struct ReplyShortChannelIdsEnd {
    #[msg_type=262]
    ty: u16,
    chain_hash: ChainHash,
}

