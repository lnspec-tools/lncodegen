//! Gen logic to dispach the different way to generate
//! the lightning network spec.
pub(crate) mod csv_method;

pub trait CodeGenMethod {
    type Error = String;
    /// Take the content of the file where the bolt
    /// is defined and generate the python code that contains
    /// all the message define inside it
    async fn generate(&self, bolt_content: &str) -> Result<String, Self::Error>;
}
