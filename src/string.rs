pub fn delete_word(s: &str) -> String {
    s.trim_end()
        .rsplit_once(' ')
        .map(|(rem, _)| rem.to_string() + " ")
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_word() {
        assert_eq!(delete_word("aaa  bb ccc"), "aaa  bb ".to_string());
        assert_eq!(delete_word("aaa  bb ccc   "), "aaa  bb ".to_string());
        assert_eq!(delete_word("   "), String::new());
        assert_eq!(delete_word("#.- {}()   "), "#.- ".to_string());
    }
}
