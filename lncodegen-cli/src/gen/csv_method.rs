//! Implementing the Code generator base on the CSV file.
use crate::gen::CodeGenMethod;

use codegen::codegen::CodeGen;
use codegen::rust::RustCodeGen;
use csvlang::parser::parser::Parser;
use csvlang::scanner::scanner::Scanner;

pub struct CSVCodeGen {
    pub lang: String,
}

impl CodeGenMethod for CSVCodeGen {
    fn generate(&self, bolt_content: &str) -> anyhow::Result<String> {
        let mut scanner = Scanner::new();
        let mut parser = Parser::new();
        let tokens = scanner.scan(bolt_content);
        parser.parse(&tokens);

        let symbol_table = parser.symbol_table;

        let content = match self.lang.as_str() {
            "rust" | "rs" => {
                let mut backend = RustCodeGen::new(&symbol_table);
                backend.generate(&symbol_table);
                backend.to_string()
            }
            _ => anyhow::bail!("Language not supported"),
        };

        Ok(content)
    }
}
