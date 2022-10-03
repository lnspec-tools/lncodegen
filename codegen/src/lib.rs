///! Take AST (-> Verify the validity of the message)? -> Generate code
pub mod codegen;
pub mod python;

#[cfg(test)]
mod test {
    use frontend_csv::parser::parser::Parser;
    use frontend_csv::scanner::scanner::Scanner;
    use std::fs;

    use crate::codegen::CodeGen;
    use crate::python::PythonCodeGen;

    #[test]
    fn init_code_py_gen() {
        let path_file = std::env::var_os("CSV_PATH").unwrap();
        let contents = fs::read_to_string(format!("{}/bolt1.csv", path_file.to_str().unwrap()))
            .expect("Something went wrong reading the file");
        let mut scanner = Scanner::new();
        let mut parser = Parser::new();
        let char_vec: Vec<char> = contents.chars().collect();
        let tokens = scanner.scan(&char_vec);
        parser.parse(&tokens);
        let mut python_gen = PythonCodeGen::new(&parser.symbol_table);
        python_gen.generate(&parser.symbol_table);
        println!("------> {}", python_gen.file_content);
        assert!(python_gen.file_content.contains("InitMsg"));
    }
}
