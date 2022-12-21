//! Implementing the Code generator base on the CSV file.
use crate::gen::CodeGenMethod;
use codegen::codegen::CodeGen;
use codegen::python::PythonCodeGen;
use codegen::rust::RustCodeGen;
use frontend_csv::parser::parser::Parser;
use frontend_csv::scanner::scanner::Scanner;

use super::CodeGenError;

pub struct CSVCodeGen {
    pub lang: String,
}

impl CodeGenMethod for CSVCodeGen {
    type Error = CodeGenError;

    async fn generate(&self, bolt_content: &str) -> Result<String, Self::Error> {
        let mut scanner = Scanner::new();
        let mut parser = Parser::new();
        let source = bolt_content.chars().collect();
        let tokens = scanner.scan(&source);
        parser.parse(&tokens);

        let symbol_table = parser.symbol_table;

        let content = match self.lang.as_str() {
            "python" | "py" => {
                let mut backend = PythonCodeGen::new(&symbol_table);
                backend.generate(&symbol_table);
                backend.to_string()
            }
            "rust" | "rs" => {
                let mut backend = RustCodeGen::new(&symbol_table);
                backend.generate(&symbol_table);
                backend.to_string()
            }
            _ => panic!("Language not supported"),
        };

        Ok(content)
    }
}
