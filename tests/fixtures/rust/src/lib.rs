pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds() {
        assert_eq!(add(2, 2), 4);
    }

    #[test]
    fn database_url_well_formed() {
        if let Ok(url) = std::env::var("DATABASE_URL") {
            assert!(url.starts_with("postgres://"));
        }
    }
}
