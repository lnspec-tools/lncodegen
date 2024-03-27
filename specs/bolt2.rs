// code generated with the lngen, please not edit this file.
use std::io::{Read, Write};

use fundamentals_derive::{DecodeWire, EncodeWire};

use crate::core::{FromWire, ToWire};
use crate::prelude::*;


#[derive(DecodeWire, EncodeWire, Debug, Clone)]
pub struct AcceptChannel {
    #[msg_type=33]
    pub ty: u16,
    pub temporary_channel_id: BitFlag,
    pub dust_limit_satoshis: u64,
    pub max_htlc_value_in_flight_msat: u64,
    pub channel_reserve_satoshis: u64,
    pub htlc_minimum_msat: u64,
    pub minimum_depth: u32,
    pub to_self_delay: u16,
    pub max_accepted_htlcs: u16,
    pub funding_pubkey: Point,
    pub revocation_basepoint: Point,
    pub payment_basepoint: Point,
    pub delayed_payment_basepoint: Point,
    pub htlc_basepoint: Point,
    pub first_per_commitment_point: Point,
    pub accept_channel_tlvs: Stream,
}

#[derive(DecodeWire, EncodeWire, Debug, Clone)]
pub struct AcceptChannel2 {
    #[msg_type=65]
    pub ty: u16,
    pub temporary_channel_id: ChannelId,
    pub funding_satoshis: u64,
    pub dust_limit_satoshis: u64,
    pub max_htlc_value_in_flight_msat: u64,
    pub htlc_minimum_msat: u64,
    pub minimum_depth: u32,
    pub to_self_delay: u16,
    pub max_accepted_htlcs: u16,
    pub funding_pubkey: Point,
    pub revocation_basepoint: Point,
    pub payment_basepoint: Point,
    pub delayed_payment_basepoint: Point,
    pub htlc_basepoint: Point,
    pub first_per_commitment_point: Point,
    pub second_per_commitment_point: Point,
    pub accept_tlvs: Stream,
}

#[derive(DecodeWire, EncodeWire, Debug, Clone)]
pub struct ChannelReady {
    #[msg_type=36]
    pub ty: u16,
    pub channel_id: ChannelId,
    pub second_per_commitment_point: Point,
    pub channel_ready_tlvs: Stream,
}

#[derive(DecodeWire, EncodeWire, Debug, Clone)]
pub struct ChannelReestablish {
    #[msg_type=136]
    pub ty: u16,
    pub channel_id: ChannelId,
    pub next_commitment_number: u64,
    pub next_revocation_number: u64,
    pub your_last_per_commitment_secret: BitFlag,
    pub my_current_per_commitment_point: Point,
}

#[derive(DecodeWire, EncodeWire, Debug, Clone)]
pub struct ClosingSigned {
    #[msg_type=39]
    pub ty: u16,
    pub channel_id: ChannelId,
    pub fee_satoshis: u64,
    pub signature: Signature,
    pub closing_signed_tlvs: Stream,
}

#[derive(DecodeWire, EncodeWire, Debug, Clone)]
pub struct CommitmentSigned {
    #[msg_type=132]
    pub ty: u16,
    pub channel_id: ChannelId,
    pub signature: Signature,
    pub htlc_signature: BitFlag,
}

#[derive(DecodeWire, EncodeWire, Debug, Clone)]
pub struct FundingCreated {
    #[msg_type=34]
    pub ty: u16,
    pub temporary_channel_id: BitFlag,
    pub funding_txid: Sha256,
    pub funding_output_index: u16,
    pub signature: Signature,
}

#[derive(DecodeWire, EncodeWire, Debug, Clone)]
pub struct FundingSigned {
    #[msg_type=35]
    pub ty: u16,
    pub channel_id: ChannelId,
    pub signature: Signature,
}

#[derive(DecodeWire, EncodeWire, Debug, Clone)]
pub struct OpenChannel {
    #[msg_type=32]
    pub ty: u16,
    pub chain_hash: ChainHash,
    pub temporary_channel_id: BitFlag,
    pub funding_satoshis: u64,
    pub push_msat: u64,
    pub dust_limit_satoshis: u64,
    pub max_htlc_value_in_flight_msat: u64,
    pub channel_reserve_satoshis: u64,
    pub htlc_minimum_msat: u64,
    pub feerate_per_kw: u32,
    pub to_self_delay: u16,
    pub max_accepted_htlcs: u16,
    pub funding_pubkey: Point,
    pub revocation_basepoint: Point,
    pub payment_basepoint: Point,
    pub delayed_payment_basepoint: Point,
    pub htlc_basepoint: Point,
    pub first_per_commitment_point: Point,
    pub channel_flags: BitFlag,
    pub open_channel_tlvs: Stream,
}

#[derive(DecodeWire, EncodeWire, Debug, Clone)]
pub struct OpenChannel2 {
    #[msg_type=64]
    pub ty: u16,
    pub chain_hash: ChainHash,
    pub temporary_channel_id: ChannelId,
    pub funding_feerate_perkw: u32,
    pub commitment_feerate_perkw: u32,
    pub funding_satoshis: u64,
    pub dust_limit_satoshis: u64,
    pub max_htlc_value_in_flight_msat: u64,
    pub htlc_minimum_msat: u64,
    pub to_self_delay: u16,
    pub max_accepted_htlcs: u16,
    pub locktime: u32,
    pub funding_pubkey: Point,
    pub revocation_basepoint: Point,
    pub payment_basepoint: Point,
    pub delayed_payment_basepoint: Point,
    pub htlc_basepoint: Point,
    pub first_per_commitment_point: Point,
    pub second_per_commitment_point: Point,
    pub channel_flags: BitFlag,
    pub opening_tlvs: Stream,
}

#[derive(DecodeWire, EncodeWire, Debug, Clone)]
pub struct RevokeAndAck {
    #[msg_type=133]
    pub ty: u16,
    pub channel_id: ChannelId,
    pub per_commitment_secret: BitFlag,
    pub next_per_commitment_point: Point,
}

#[derive(DecodeWire, EncodeWire, Debug, Clone)]
pub struct Shutdown {
    #[msg_type=38]
    pub ty: u16,
    pub channel_id: ChannelId,
    pub scriptpubkey: BitFlag,
}

#[derive(DecodeWire, EncodeWire, Debug, Clone)]
pub struct TxAbort {
    #[msg_type=74]
    pub ty: u16,
    pub channel_id: ChannelId,
    pub data: BitFlag,
}

#[derive(DecodeWire, EncodeWire, Debug, Clone)]
pub struct TxAckRbf {
    #[msg_type=73]
    pub ty: u16,
    pub channel_id: ChannelId,
    pub tx_ack_rbf_tlvs: Stream,
}

#[derive(DecodeWire, EncodeWire, Debug, Clone)]
pub struct TxAddInput {
    #[msg_type=66]
    pub ty: u16,
    pub channel_id: ChannelId,
    pub serial_id: u64,
    pub prevtx: BitFlag,
    pub prevtx_vout: u32,
    pub sequence: u32,
}

#[derive(DecodeWire, EncodeWire, Debug, Clone)]
pub struct TxAddOutput {
    #[msg_type=67]
    pub ty: u16,
    pub channel_id: ChannelId,
    pub serial_id: u64,
    pub sats: u64,
    pub script: BitFlag,
}

#[derive(DecodeWire, EncodeWire, Debug, Clone)]
pub struct TxComplete {
    #[msg_type=70]
    pub ty: u16,
    pub channel_id: ChannelId,
}

#[derive(DecodeWire, EncodeWire, Debug, Clone)]
pub struct TxInitRbf {
    #[msg_type=72]
    pub ty: u16,
    pub channel_id: ChannelId,
    pub locktime: u32,
    pub feerate: u32,
    pub tx_init_rbf_tlvs: Stream,
}

#[derive(DecodeWire, EncodeWire, Debug, Clone)]
pub struct TxRemoveInput {
    #[msg_type=68]
    pub ty: u16,
    pub channel_id: ChannelId,
    pub serial_id: u64,
}

#[derive(DecodeWire, EncodeWire, Debug, Clone)]
pub struct TxRemoveOutput {
    #[msg_type=69]
    pub ty: u16,
    pub channel_id: ChannelId,
    pub serial_id: u64,
}

#[derive(DecodeWire, EncodeWire, Debug, Clone)]
pub struct TxSignatures {
    #[msg_type=71]
    pub ty: u16,
    pub channel_id: ChannelId,
    pub txid: Sha256,
    pub witnesses: BitFlag,
}

#[derive(DecodeWire, EncodeWire, Debug, Clone)]
pub struct UpdateAddHtlc {
    #[msg_type=128]
    pub ty: u16,
    pub channel_id: ChannelId,
    pub id: u64,
    pub amount_msat: u64,
    pub payment_hash: Sha256,
    pub cltv_expiry: u32,
    pub onion_routing_packet: BitFlag,
}

#[derive(DecodeWire, EncodeWire, Debug, Clone)]
pub struct UpdateFailHtlc {
    #[msg_type=131]
    pub ty: u16,
    pub channel_id: ChannelId,
    pub id: u64,
    pub reason: BitFlag,
}

#[derive(DecodeWire, EncodeWire, Debug, Clone)]
pub struct UpdateFailMalformedHtlc {
    #[msg_type=135]
    pub ty: u16,
    pub channel_id: ChannelId,
    pub id: u64,
    pub sha256_of_onion: Sha256,
    pub failure_code: u16,
}

#[derive(DecodeWire, EncodeWire, Debug, Clone)]
pub struct UpdateFee {
    #[msg_type=134]
    pub ty: u16,
    pub channel_id: ChannelId,
    pub feerate_per_kw: u32,
}

#[derive(DecodeWire, EncodeWire, Debug, Clone)]
pub struct UpdateFulfillHtlc {
    #[msg_type=130]
    pub ty: u16,
    pub channel_id: ChannelId,
    pub id: u64,
    pub payment_preimage: BitFlag,
}

