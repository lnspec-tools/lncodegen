//! Code Generation module implementation
use std::collections::BTreeMap;

use log::debug;

use csvlang::parser::ast::{LNMsData, LNMsg, LNMsgType, LNSubType, LNTlvRecord};

/// Code Gen trait that specify all the basic method to create a new
/// code generation target to encode and decode a lightning network message.
///
/// This idea is heavily inspired to LLVM language binding to generate LLVM IR
/// in a compiler backend.
pub trait CodeGen<'g> {
    fn new(symbol_table: &'g BTreeMap<String, LNMsgType>) -> Self;

    /// Build a LN message the correct format for the actual
    /// code generation for the language.
    fn build_msg(&mut self, _msg: &LNMsg);

    /// Close a LN message building process.
    fn end_msg(&mut self, _msg: &LNMsg);

    /// build the decode function
    fn build_decode_fun(&mut self);

    /// end decode function
    fn end_decode_fn(&mut self);

    /// build the encode function
    fn build_encode_fn(&mut self);

    /// end encode function
    fn end_encode_fn(&mut self);

    fn build_u16(&mut self, field: &LNMsData);

    fn write_u16(&mut self, field: &LNMsData);

    fn build_u32(&mut self, field: &LNMsData);

    fn write_u32(&mut self, field: &LNMsData);

    fn build_u64(&mut self, field: &LNMsData);

    fn write_u64(&mut self, field: &LNMsData);

    fn build_chain_hash(&mut self, field: &LNMsData);

    fn write_chain_hash(&mut self, filed: &LNMsData);

    fn build_channel_id(&mut self, filed: &LNMsData);

    fn write_channel_id(&mut self, field: &LNMsData);

    fn build_short_channel_id(&mut self, filed: &LNMsData);

    fn write_short_channel_id(&mut self, field: &LNMsData);

    fn build_signature(&mut self, filed: &LNMsData);

    fn write_signature(&mut self, field: &LNMsData);

    fn build_point(&mut self, field: &LNMsData);

    fn write_point(&mut self, field: &LNMsData);

    fn build_bitfield(&mut self, filed: &LNMsData);

    fn write_bitfiled(&mut self, field: &LNMsData);

    fn build_tlv_stream(&mut self, field: &LNTlvRecord);

    fn write_tlv_stream(&mut self, field: &LNTlvRecord);

    fn generate_decode_fn(&mut self, msg: &LNMsg, symbol_table: &'g BTreeMap<String, LNMsgType>) {
        self.build_decode_fun();
        for field in &msg.msg_data {
            match field {
                LNMsData::Uint16(_) => self.build_u16(field),
                LNMsData::Uint32(_) => self.build_u32(field),
                LNMsData::Uint64(_) => self.build_u64(field),
                LNMsData::ChainHash(_, _) => self.build_chain_hash(field),
                LNMsData::ChannelId(_) => self.build_channel_id(field),
                LNMsData::ShortChannelId(_) => self.build_short_channel_id(field),
                LNMsData::Signature(_) => self.build_signature(field),
                LNMsData::Point(_) => self.build_point(field),
                LNMsData::BitfieldStream(_, _) => self.build_bitfield(field),
                LNMsData::TLVinit(tlv_name, _) => {
                    let tlv = symbol_table.get(tlv_name).unwrap();
                    if let LNMsgType::Tlv(tlv) = tlv {
                        self.build_tlv_stream(tlv);
                    } else {
                        panic!("Wrong type, we should look for a tlv record {:?}", tlv);
                    }
                }
            }
        }
        self.end_decode_fn();
    }

    fn generate_encode_fn(&mut self, msg: &LNMsg, symbol_table: &'g BTreeMap<String, LNMsgType>) {
        self.build_encode_fn();
        for field in &msg.msg_data {
            match field {
                LNMsData::Uint16(_) => self.write_u16(field),
                LNMsData::Uint32(_) => self.write_u32(field),
                LNMsData::Uint64(_) => self.write_u64(field),
                LNMsData::ChainHash(_, _) => self.write_chain_hash(field),
                LNMsData::ChannelId(_) => self.write_channel_id(field),
                LNMsData::ShortChannelId(_) => self.write_short_channel_id(field),
                LNMsData::Signature(_) => self.write_signature(field),
                LNMsData::Point(_) => self.write_point(field),
                LNMsData::BitfieldStream(_, _) => self.write_bitfiled(field),
                LNMsData::TLVinit(tlv_name, _) => {
                    let tlv = symbol_table.get(tlv_name).unwrap();
                    if let LNMsgType::Tlv(tlv) = tlv {
                        self.write_tlv_stream(tlv);
                    } else {
                        panic!("Wrong type inside the TLV Init {:?}", tlv);
                    }
                }
            }
        }
        self.end_encode_fn();
    }

    fn generate_msg(&mut self, msg: &LNMsg, symbol_table: &'g BTreeMap<String, LNMsgType>) {
        self.build_msg(msg);
        self.generate_encode_fn(msg, symbol_table);
        self.generate_decode_fn(msg, symbol_table);
        self.end_msg(msg);
    }

    fn generate_tlv(&mut self, _tlv: &LNTlvRecord) {}

    fn generate_subtype(&mut self, _subtyp: &LNSubType) {}

    fn pre_generation(&mut self);

    fn post_generation(&mut self) {}

    fn generate(&mut self, symbol_table: &'g BTreeMap<String, LNMsgType>) {
        self.pre_generation();
        for ast_item in symbol_table.values() {
            match ast_item {
                LNMsgType::Msg(msg) => self.generate_msg(msg, symbol_table),
                LNMsgType::SubType(sub_typ) => self.generate_subtype(sub_typ),
                LNMsgType::Tlv(_) => {
                    debug!("tlv on generate method ignore, and need to be ignore!");
                }
            }
        }
        self.post_generation();
    }
}
