mod scanner;
/// Scanner implementation!
pub mod token;

#[cfg(test)]
mod test {
    use std::env;

    #[test]
    fn scan_simple_one() {
        let path_file = std::env::vars()["CSV_PATH"];
        assert!(true);
    }
}
