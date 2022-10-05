//! Python Lightning Network Message Generation Implementation.
//!
//! Welcome in this python implementation of the Lightning Network
//! code gen.
//!
//! In this file will go to implement the code generator for
//! lightning network to generate lightning network message
//! with the possibility to encode and decode a message.
//!
//! author: Vincenzo Palazzo <vincenzopalazzodev@gmail.com>
use crate::codegen::CodeGen;
use convert_case::{Case, Casing};
use frontend_csv::parser::ast::{LNMsData, LNMsg, LNMsgType};
use indoc::indoc;
use std::collections::BTreeMap;

pub struct PythonCodeGen {
    pub file_content: String,
    identation: u16,
}

impl PythonCodeGen {
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
        self.identation += 4;
    }

    fn close_scope(&mut self) {
        assert!(
            self.identation >= 4,
            "scope can not be close, space {} < 4",
            self.identation
        );
        self.identation -= 4;
        self.file_content += self
            .add_identation_to_code(&"# close scope".to_string())
            .as_str()
    }

    fn build_msg_name(&self, msg: &LNMsg) -> String {
        format!("{}Msg", msg.msg_name.to_case(Case::Title))
    }
}

impl<'g> CodeGen<'g> for PythonCodeGen {
    fn new(_symbol_table: &'g BTreeMap<String, LNMsgType>) -> Self {
        PythonCodeGen {
            file_content: String::new(),
            identation: 0,
        }
    }

    fn pre_generation(&self) {}

    fn build_msg(&mut self, msg: &LNMsg) {
        let class_name = self.build_msg_name(msg);
        let class_msg = format!("class {}: \n", class_name);
        self.file_content += class_msg.as_str();
        self.open_scope();
    }

    fn end_msg(&mut self, _msg: &LNMsg) {
        self.close_scope();
    }

    fn build_decode_fun(&mut self) {
        let code = indoc! {"
            @staticmethod
            def decode(raw_msg: str):
        "};
        self.file_content += self.add_identation_to_code(&code.to_string()).as_str();
        self.open_scope();
    }

    fn end_decode_fn(&mut self) {
        self.close_scope();
        self.file_content += "\n\n";
    }

    fn build_encode_fn(&mut self) {
        let code = indoc! {"
            def encode(self) -> str:
                raw_msg = ''
        "};
        self.file_content += self.add_identation_to_code(&code.to_string()).as_str();
        self.open_scope();
    }

    fn end_encode_fn(&mut self) {
        let code = "return raw_msg";
        self.file_content += self.add_identation_to_code(&code.to_string()).as_str();
        self.close_scope();
        self.file_content += "\n\n";
    }

    fn build_u16(&mut self, field: &frontend_csv::parser::ast::LNMsData) {
        if let LNMsData::Uint16(name) = field {
            let code = format!(
                "self.__u16_{}, raw_msg = U16Int.decode_with_hex_str(raw_msg)",
                name
            );
            self.file_content += self.add_identation_to_code(&code.to_string()).as_str();
        }
    }

    fn write_u16(&mut self, field: &frontend_csv::parser::ast::LNMsData) {
        if let LNMsData::Uint16(name) = field {
            let code = format!("raw_msg += self.__u16_{}.encode()", name);
            self.file_content += self.add_identation_to_code(&code.to_string()).as_str();
        }
    }

    fn build_u32(&mut self, field: &frontend_csv::parser::ast::LNMsData) {
        todo!()
    }

    fn write_u32(&mut self, field: &frontend_csv::parser::ast::LNMsData) {
        todo!()
    }

    fn build_u64(&mut self, field: &frontend_csv::parser::ast::LNMsData) {
        todo!()
    }

    fn write_u64(&mut self, field: &frontend_csv::parser::ast::LNMsData) {
        todo!()
    }

    fn write_bitfiled(&mut self, field: &frontend_csv::parser::ast::LNMsData) {
        if let LNMsData::BitfieldStream(name, _) = field {
            let code = format!(
                "if self.__bitf_{} => 0:\n \
                 \t raw_msg = Bitfield.encode(self.__bitf_{})",
                name, name
            );
            self.file_content += self.add_identation_to_code(&code.to_string()).as_str();
        }
    }

    fn build_bitfield(&mut self, field: &frontend_csv::parser::ast::LNMsData) {
        if let LNMsData::BitfieldStream(name, _) = field {
            let code = format!(
                "self.__bitf_{}, raw_msg = Bitfield.decode_with_len(raw_msg)",
                name
            );
            self.file_content += self.add_identation_to_code(&code.to_string()).as_str();
        }
    }

    fn write_point(&mut self, field: &frontend_csv::parser::ast::LNMsData) {
        todo!()
    }

    fn build_point(&mut self, field: &frontend_csv::parser::ast::LNMsData) {
        todo!()
    }

    fn build_chain_hash(&mut self, field: &frontend_csv::parser::ast::LNMsData) {
        todo!()
    }

    fn write_chain_hash(&mut self, filed: &frontend_csv::parser::ast::LNMsData) {
        todo!()
    }

    fn build_channel_id(&mut self, filed: &frontend_csv::parser::ast::LNMsData) {
        todo!();
    }

    fn write_channel_id(&mut self, field: &frontend_csv::parser::ast::LNMsData) {
        todo!()
    }

    fn build_signature(&mut self, filed: &frontend_csv::parser::ast::LNMsData) {
        todo!()
    }

    fn write_signature(&mut self, field: &frontend_csv::parser::ast::LNMsData) {
        todo!()
    }
}
