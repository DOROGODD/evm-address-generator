pub fn validate_string<'a, 'b>(value: &'a String) -> Result<(), &'b str> {
    if value.chars().all(|char| char.is_ascii_hexdigit()) {
        Ok(())
    } else {
        Err("input contains invalid characters")
    }
}
