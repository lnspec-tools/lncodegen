//! Take `AST (-> Verify the validity of the message) -> Generate code`
pub mod codegen;
pub mod rust;

#[cfg(test)]
mod test {
    #![allow(dead_code)]
    use std::sync::Once;

    static INIT: Once = Once::new();

    fn init() {
        // ignore error
        INIT.call_once(|| {
            env_logger::init();
        });
    }
}
