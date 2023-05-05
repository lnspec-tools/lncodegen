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
use std::collections::{BTreeMap, HashSet};

// FIXME: manage the current message in order to support
// multiple message defined in the same CSV file.
pub struct PythonCodeGen {
    curr_msg: Option<LNMsg>,
    /// Python class definition, is keep separate
    /// because we do not know how many filed we have inside the constructor
    class_definition: String,
    class_implementation: String,
    imports: String,
    /// All the filed name that are used
    /// to generate the constructor for the class.
    // FIXME: move this as a map, where we keep the information
    // if the following field need to be inside the constructor
    // or none by default!
    fields: Vec<String>,
    pub file_content: String,
    full_source: String,
    identation: u16,
    // Keep in memory the possible keywords that can be used
    // inside the
    keywords: HashSet<String>,
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

    fn add_space_between_blocks(&self, content: &mut String) {
        *content += "\n\n";
    }

    fn open_scope(&mut self) {
        self.identation += 4;
    }

    fn close_scope(&mut self) -> String {
        assert!(
            self.identation >= 4,
            "scope can not be close, space {} < 4",
            self.identation
        );
        self.identation -= 4;
        self.add_identation_to_code(&"# close scope".to_string())
    }

    fn build_msg_name(&mut self, msg: &LNMsg) -> String {
        self.curr_msg = Some(msg.to_owned());
        format!("{}Msg", msg.msg_name.to_case(Case::Title))
    }

    fn end_class_implementation(&mut self) {
        self.fields.clear();
        self.class_definition.clear();
        self.class_implementation.clear();
        self.curr_msg = None
    }

    fn generate_contructor(&mut self) -> String {
        let mut body = if self.fields.is_empty() {
            "pass".to_string()
        } else {
            String::new()
        };
        let mut signature = "def __init__(self, ".to_string();
        for value in &self.fields {
            let no_keyword = self.transmute_if_keyword(&value);
            signature += format!("{no_keyword}, ").as_str();
            body += format!("self.{value} = {no_keyword}\n").as_str();
        }
        signature = signature.trim_end().strip_suffix(",").unwrap().to_owned();
        signature += "):\n";
        let mut contructor = self.add_identation_to_code(&signature);
        self.open_scope();
        contructor += self.add_identation_to_code(&body).as_str();
        let tag = self.close_scope().to_owned();
        contructor += tag.as_str();
        self.add_space_between_blocks(&mut contructor);
        contructor
    }

    fn initialize_class(&mut self) -> String {
        let mut msg = self.curr_msg.to_owned().unwrap().clone();
        let class_name = self.build_msg_name(&mut msg);
        let mut params = String::new();
        for param in &self.fields {
            params += format!("{param}, ").as_str();
        }
        params = params.trim_end().strip_suffix(",").unwrap().to_owned();
        format!("{class_name}({params})")
    }

    fn build_msg_type(&mut self) -> String {
        let name = "msg_type";
        self.fields.push(name.to_owned());
        format!(
            "{}, raw_msg = U16Int.decode_with_hex_str(raw_msg)\n\n",
            name
        )
    }

    fn write_msg_type(&self) -> String {
        let name = "msg_type";
        format!("raw_msg += '{{}}'.format(self.{name}.encode())\n\n")
    }

    fn transmute_if_keyword(&self, var_name: &str) -> String {
        if self.keywords.contains(&var_name.to_string()) {
            return format!("no_keyword_{var_name}");
        }
        var_name.to_owned()
    }
}

impl std::fmt::Display for PythonCodeGen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.full_source)
    }
}

impl<'g> CodeGen<'g> for PythonCodeGen {
    fn new(_symbol_table: &'g BTreeMap<String, LNMsgType>) -> Self {
        let mut keywords = HashSet::new();
        keywords.insert("len".to_owned());
        PythonCodeGen {
            curr_msg: None,
            class_definition: String::new(),
            class_implementation: String::new(),
            imports: String::new(),
            file_content: String::new(),
            full_source: String::new(),
            identation: 0,
            fields: Vec::new(),
            keywords,
        }
    }

    /// callback called before to generate the file.
    /// A good place to pt the import of the files.
    fn pre_generation(&mut self) {
        self.imports = indoc! {"
# code generated by the lncodegen.rs please do not edit\n
from lnspec_py.basic_type.int import U16Int, U32Int
from lnspec_py.basic_type.hex_type import ChannelId
from lnspec_py.basic_type.bitmask import Bitfield
from lnspec_py.basic_type.tvl_record import TVLRecord
"}
        .to_owned();
    }

    fn post_generation(&mut self) {
        self.full_source += self.imports.as_str();
        self.full_source += self.file_content.as_str();
    }

    fn build_msg(&mut self, msg: &LNMsg) {
        let class_name = self.build_msg_name(msg);
        let class_msg = format!("class {}: \n", class_name);
        self.class_definition += class_msg.as_str();
        self.open_scope();
    }

    fn end_msg(&mut self, _msg: &LNMsg) {
        let tag = self.close_scope().to_owned();
        self.class_implementation += format!("{tag}\n").as_str();
        self.file_content += self.class_definition.as_str();
        self.open_scope();
        let contructor = self.generate_contructor().to_owned();
        self.file_content += contructor.as_str();
        self.file_content += self.class_implementation.as_str();
        self.close_scope();
        self.end_class_implementation();
    }

    fn build_decode_fun(&mut self) {
        let mut code = indoc! {"
            @staticmethod
            def decode(raw_msg: str):"}
        .to_string();
        let build_msg_ty = self.build_msg_type();
        code += self.add_identation_to_code(&build_msg_ty).as_str();
        self.class_implementation += self.add_identation_to_code(&code.to_string()).as_str();
        self.open_scope();
    }

    fn end_decode_fn(&mut self) {
        let init_class = self.initialize_class();
        let code = format!("return {init_class}");
        self.class_implementation += self.add_identation_to_code(&code).as_str();
        let tag = self.close_scope().to_owned();
        self.file_content += tag.as_str();
        self.file_content += "\n\n";
    }

    fn build_encode_fn(&mut self) {
        let mut code = indoc! {"
            def encode(self) -> str:
                raw_msg = ''"}
        .to_string();
        let write_msg_ty = self.write_msg_type();
        code += self.add_identation_to_code(&write_msg_ty).as_str();
        self.class_implementation += self.add_identation_to_code(&code.to_string()).as_str();
        self.open_scope();
    }

    fn end_encode_fn(&mut self) {
        let code = "return raw_msg";
        self.class_implementation += self.add_identation_to_code(&code.to_string()).as_str();
        let tag = self.close_scope().to_owned();
        self.class_implementation += tag.as_str();
        self.class_implementation += "\n\n";
    }

    fn build_u16(&mut self, field: &frontend_csv::parser::ast::LNMsData) {
        if let LNMsData::Uint16(name) = field {
            let name = self.transmute_if_keyword(&name);
            let code = format!("{}, raw_msg = U16Int.decode_with_hex_str(raw_msg)", name);
            self.class_implementation += self.add_identation_to_code(&code.to_string()).as_str();
            self.fields.push(name.to_owned());
        }
    }

    fn write_u16(&mut self, field: &frontend_csv::parser::ast::LNMsData) {
        if let LNMsData::Uint16(name) = field {
            let name = self.transmute_if_keyword(&name);
            let code = format!("raw_msg += '{{}}'.format(self.{}.encode())", name);
            self.class_implementation += self.add_identation_to_code(&code.to_string()).as_str();
        }
    }

    fn build_u32(&mut self, _field: &frontend_csv::parser::ast::LNMsData) {
        todo!()
    }

    fn write_u32(&mut self, _field: &frontend_csv::parser::ast::LNMsData) {
        todo!()
    }

    fn build_u64(&mut self, _field: &frontend_csv::parser::ast::LNMsData) {
        todo!()
    }

    fn write_u64(&mut self, _field: &frontend_csv::parser::ast::LNMsData) {
        todo!()
    }

    fn write_bitfiled(&mut self, field: &frontend_csv::parser::ast::LNMsData) {
        if let LNMsData::BitfieldStream(name, _) = field {
            let name = self.transmute_if_keyword(&name);
            let code = format!(
                "if len(self.{}.bitfield) > 0:\n \
                 \t raw_msg += Bitfield.encode(self.{}.bitfield)",
                name, name
            );
            self.class_implementation += self.add_identation_to_code(&code.to_string()).as_str();
        }
    }

    fn build_bitfield(&mut self, field: &frontend_csv::parser::ast::LNMsData) {
        if let LNMsData::BitfieldStream(name, _) = field {
            let name = self.transmute_if_keyword(&name);
            let code = format!("{name}, raw_msg = Bitfield.decode_with_len(raw_msg)");
            self.class_implementation += self.add_identation_to_code(&code.to_string()).as_str();
            self.fields.push(name.to_owned());
        }
    }

    fn write_point(&mut self, _field: &frontend_csv::parser::ast::LNMsData) {
        todo!()
    }

    fn build_point(&mut self, _field: &frontend_csv::parser::ast::LNMsData) {
        todo!()
    }

    fn build_chain_hash(&mut self, _field: &frontend_csv::parser::ast::LNMsData) {
        todo!()
    }

    fn write_chain_hash(&mut self, _filed: &frontend_csv::parser::ast::LNMsData) {
        todo!()
    }

    fn build_channel_id(&mut self, filed: &frontend_csv::parser::ast::LNMsData) {
        if let LNMsData::ChannelId(name) = filed {
            let name = self.transmute_if_keyword(&name);
            let code = format!("{name}, raw_msg = ChannelId.decode_from_hex(raw_msg)");
            self.class_implementation += self.add_identation_to_code(&code).as_str();
            self.fields.push(name.to_owned());
        }
    }

    fn write_channel_id(&mut self, field: &frontend_csv::parser::ast::LNMsData) {
        if let LNMsData::ChannelId(name) = field {
            let name = self.transmute_if_keyword(&name);
            let code = format!("raw_msg += self.{name}.encode()");
            self.class_implementation += self.add_identation_to_code(&code).as_str();
        }
    }
    fn build_short_channel_id(&mut self, _filed: &LNMsData) {
        todo!()
    }

    fn write_short_channel_id(&mut self, _: &LNMsData) {
        todo!()
    }

    fn build_signature(&mut self, _filed: &frontend_csv::parser::ast::LNMsData) {
        todo!()
    }

    fn write_signature(&mut self, _field: &frontend_csv::parser::ast::LNMsData) {
        todo!()
    }

    // TODO: make a double check if the API are safe
    fn build_tlv_stream(&mut self, field: &frontend_csv::parser::ast::LNTlvRecord) {
        let mut source = format!("{} = TVLRecord(raw_msg)\n", field.stream_name);
        source += format!("{}.decode()", field.stream_name).as_str();
        self.class_implementation += self.add_identation_to_code(&source).as_str();
        self.fields.push(field.stream_name.to_owned());
    }

    // TODO: make a double check if the API are safe
    fn write_tlv_stream(&mut self, field: &frontend_csv::parser::ast::LNTlvRecord) {
        let source = format!("raw_msg += self.{}.encode()", field.stream_name);
        self.class_implementation += self.add_identation_to_code(&source).as_str();
    }
}
