///! Take AST (-> Verify the validity of the message)? -> Generate code
pub mod codegen;

#[cfg(test)]
mod test {
    use frontend_csv::parser::parser::Parser;
    use frontend_csv::scanner::scanner::Scanner;
    use std::fs;

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
        // FIXIME: init python codegene
    }
}
