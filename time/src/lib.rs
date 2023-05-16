#[cfg(test)]
mod tests {
    use regex::Regex;

    #[test]
    fn it_works() {
        let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
        assert!(re.is_match("2014-01-01"));
    }
}
