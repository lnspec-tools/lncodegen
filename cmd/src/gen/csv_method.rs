//! Implementing the Code generator base on the CSV file.
use crate::gen::CodeGenMethod;
use codegen::codegen::CodeGen;
use codegen::python::PythonCodeGen;
use frontend_csv::parser::parser::Parser;
use frontend_csv::scanner::scanner::Scanner;

pub struct CSVCodeGen {
    pub lang: String,
}

impl CodeGenMethod for CSVCodeGen {
    type Error = String;
    async fn generate(&self, bolt_content: &str) -> Result<String, Self::Error> {
        let content = String::new();
        let mut scanner = Scanner::new();
        let mut parser = Parser::new();
        let source = bolt_content.chars().collect();
        let tokens = scanner.scan(&source);
        parser.parse(&tokens);

        let symbol_table = parser.symbol_table;

        let mut codegen = match self.lang.as_str() {
            "python" | "py" => PythonCodeGen::new(&symbol_table),
            _ => panic!("Language not supported"),
        };

        codegen.generate(&symbol_table);
        let content = codegen.to_string();
        Ok(content)
    }
}
