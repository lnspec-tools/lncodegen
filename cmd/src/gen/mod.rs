//! Gen logic to dispach the different way to generate
//! the lightning network spec.
use std::fmt::Display;
pub(crate) mod csv_method;

pub trait CodeGenMethod {
    type Error = CodeGenError;
    /// Take the content of the file where the bolt
    /// is defined and generate the python code that contains
    /// all the message define inside it
    async fn generate(&self, bolt_content: &str) -> Result<String, Self::Error>;
}

pub struct CodeGenError {
    cause: String,
}

impl CodeGenError {
    pub fn new(cause: &str) -> Self {
        CodeGenError {
            cause: cause.to_owned(),
        }
    }
}

impl Display for CodeGenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "codegen error: {}", self.cause)
    }
}
