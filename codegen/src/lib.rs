///! Take AST (-> Verify the validity of the message)? -> Generate code
pub mod codegen;
pub mod python;

#[cfg(test)]
mod test {
    use frontend_csv::parser::parser::Parser;
    use frontend_csv::scanner::scanner::Scanner;

    use crate::codegen::CodeGen;
    use crate::python::PythonCodeGen;
    use log::trace;
    use std::sync::Once;

    static INIT: Once = Once::new();

    fn init() {
        // ignore error
        INIT.call_once(|| {
            env_logger::init();
        });
    }

    #[test]
    fn init_code_py_gen() {
        init();
        let contents = "msgtype,init,16\n \
                        msgdata,init,gflen,u16,\n";
        let mut scanner = Scanner::new();
        let mut parser = Parser::new();
        let char_vec: Vec<char> = contents.chars().collect();
        let tokens = scanner.scan(&char_vec);
        parser.parse(&tokens);
        let mut python_gen = PythonCodeGen::new(&parser.symbol_table);
        python_gen.generate(&parser.symbol_table);
        trace!("\n{}", python_gen.file_content);
        assert!(python_gen.file_content.contains("InitMsg"));
    }

    #[test]
    fn init_code_py_gen_with_bitfiled() {
        init();
        let contents = "msgtype,init,16\n \
                        msgdata,init,gflen,u16,\n \
                       msgdata,init,globalfeatures,byte,gflen\n \
                       msgdata,init,flen,u16,\n \
                       msgdata,init,features,byte,flen\n";
        let mut scanner = Scanner::new();
        let mut parser = Parser::new();
        let char_vec: Vec<char> = contents.chars().collect();
        let tokens = scanner.scan(&char_vec);
        parser.parse(&tokens);
        let mut python_gen = PythonCodeGen::new(&parser.symbol_table);
        python_gen.generate(&parser.symbol_table);
        trace!("\n{}", python_gen.file_content);
        assert!(python_gen.file_content.contains("InitMsg"));
    }

    #[test]
    fn init_code_py_gen_with_tlv_stream() {
        init();
        let contents = "msgtype,init,16\n \
                        msgdata,init,gflen,u16,\n \
                       msgdata,init,globalfeatures,byte,gflen\n \
                       msgdata,init,flen,u16,\n \
                       msgdata,init,features,byte,flen\n \
                       tlvtype,init_tlvs,networks,1\n \
                       tlvdata,init_tlvs,networks,chains,chain_hash,...\n \
                       tlvtype,init_tlvs,remote_addr,3\n \
                       tlvdata,init_tlvs,remote_addr,data,byte,...\n";
        trace!("{}", contents);
        let mut scanner = Scanner::new();
        let mut parser = Parser::new();
        let char_vec: Vec<char> = contents.chars().collect();
        let tokens = scanner.scan(&char_vec);
        parser.parse(&tokens);
        let mut python_gen = PythonCodeGen::new(&parser.symbol_table);
        python_gen.generate(&parser.symbol_table);
        trace!("\n{}", python_gen.file_content);
        assert!(python_gen.file_content.contains("InitMsg"));
    }
}
