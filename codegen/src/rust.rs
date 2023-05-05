//! Rust code generation for the lightning
//! network specification.
use super::codegen::CodeGen;
use convert_case::{Case, Casing};
use frontend_csv::parser::ast::{LNMsData, LNMsg, LNMsgType, LNTlvRecord};
use std::{collections::BTreeMap, fmt::Display};

pub struct RustCodeGen {
    #[allow(dead_code)]
    symbol_table: BTreeMap<String, LNMsgType>,
    file_content: String,
    identation: u16,
}

macro_rules! fmt_struct_filed {
    ($name:expr, $ty:expr) => {
        format!("{}: {},\n", $name.as_str(), $ty)
    };
}

macro_rules! fmt_struct_filed_with_attr {
    ($name: expr, $ty: expr, $attr: expr) => {
        format!("#[{}]\n{}: {},\n", $attr, $name.as_str(), $ty)
    };
}

impl RustCodeGen {
    fn add_identation_to_code(&self, content: &String) -> String {
        let clean_content = String::new();
        content
            .trim()
            .split("\n")
            .fold(clean_content, |mut content, line| {
                let formatted_str = "\n".to_owned() + &" ".repeat(self.identation.into()) + line;
                content.push_str(&formatted_str);
                content
            })
    }

    fn open_scope(&mut self) {
        self.file_content += "{";
        self.identation += 4;
    }

    fn close_scope(&mut self) -> String {
        assert!(
            self.identation >= 4,
            "scope can not be close, space {} < 4",
            self.identation
        );
        self.identation -= 4;
        self.add_identation_to_code(&"\n}".to_string())
    }
}

impl<'g> CodeGen<'g> for RustCodeGen {
    fn new(symbol_table: &'g BTreeMap<String, LNMsgType>) -> Self {
        RustCodeGen {
            symbol_table: symbol_table.to_owned(),
            file_content: String::new(),
            identation: 0,
        }
    }

    fn pre_generation(&mut self) {
        let mut code = "// code generated with the lngen, please not edit this file.\n".to_owned();
        code += "use std::io::{Read, Write};\n\n";
        code += "use lnspec_derive::{DecodeWire, EncodeWire};\n\n";
        code += "use crate::core::{FromWire, ToWire};\n";
        code += "use crate::prelude::*;\n";
        self.file_content += code.as_str();
        self.file_content += "\n\n";
    }

    fn build_msg(&mut self, msg: &LNMsg) {
        let mut code = String::from("#[derive(DecodeWire, EncodeWire)]\n");
        code += &format!("pub struct {} ", msg.msg_name.to_case(Case::Pascal));
        self.file_content += &code;
        self.open_scope();
        self.file_content += self
            .add_identation_to_code(&"#[warn(dead_code)]".to_owned())
            .as_str();
        let attr = format!("msg_type={}", msg.msg_typ);
        let code = fmt_struct_filed_with_attr!("ty".to_owned(), "u16", attr);
        self.file_content += self.add_identation_to_code(&code).as_str();
    }

    fn end_msg(&mut self, _: &LNMsg) {
        let code = self.close_scope();
        self.file_content += code.as_str();
        self.file_content += "\n\n";
    }

    fn build_encode_fn(&mut self) {}

    fn end_encode_fn(&mut self) {}

    fn build_decode_fun(&mut self) {}

    fn end_decode_fn(&mut self) {}

    fn build_u16(&mut self, field: &LNMsData) {
        if let LNMsData::Uint16(name) = field {
            let code = fmt_struct_filed!(name, "u16");
            self.file_content += self.add_identation_to_code(&code).as_str();
        }
    }

    fn write_u16(&mut self, _: &LNMsData) {}

    fn build_u32(&mut self, field: &LNMsData) {
        if let LNMsData::Uint32(name) = field {
            let code = fmt_struct_filed!(name, "u32");
            self.file_content += self.add_identation_to_code(&code).as_str();
        }
    }

    fn write_u32(&mut self, _: &LNMsData) {}

    fn build_u64(&mut self, field: &frontend_csv::parser::ast::LNMsData) {
        if let LNMsData::Uint64(name) = field {
            let code = fmt_struct_filed!(name, "u64");
            self.file_content += self.add_identation_to_code(&code).as_str();
        }
    }

    fn write_u64(&mut self, _: &LNMsData) {}

    fn build_chain_hash(&mut self, field: &LNMsData) {
        if let LNMsData::ChainHash(name, _) = field {
            let code = fmt_struct_filed!(name, "ChainHash");
            self.file_content += self.add_identation_to_code(&code).as_str();
        }
    }

    fn write_chain_hash(&mut self, _: &LNMsData) {}

    fn build_channel_id(&mut self, filed: &LNMsData) {
        if let LNMsData::ChannelId(name) = filed {
            let code = fmt_struct_filed!(name, "ChannelId");
            self.file_content += self.add_identation_to_code(&code).as_str();
        }
    }

    fn write_channel_id(&mut self, _: &LNMsData) {}

    fn build_short_channel_id(&mut self, filed: &LNMsData) {
        if let LNMsData::ShortChannelId(name) = filed {
            let code = fmt_struct_filed!(name, "ShortChannelId");
            self.file_content += self.add_identation_to_code(&code).as_str();
        }
    }

    fn write_short_channel_id(&mut self, _: &LNMsData) {}

    fn build_point(&mut self, field: &LNMsData) {
        if let LNMsData::Point(name) = field {
            let code = fmt_struct_filed!(name, "Point");
            self.file_content += self.add_identation_to_code(&code).as_str();
        }
    }

    fn write_point(&mut self, _: &LNMsData) {}

    fn build_signature(&mut self, filed: &LNMsData) {
        if let LNMsData::Signature(name) = filed {
            let code = fmt_struct_filed!(name, "Signature");
            self.file_content += self.add_identation_to_code(&code).as_str();
        }
    }

    fn write_signature(&mut self, _: &LNMsData) {}

    fn build_tlv_stream(&mut self, _field: &LNTlvRecord) {}

    fn write_tlv_stream(&mut self, _: &LNTlvRecord) {}

    fn build_bitfield(&mut self, _filed: &LNMsData) {}

    fn write_bitfiled(&mut self, _: &LNMsData) {}
}

impl Display for RustCodeGen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.file_content)
    }
}
