fn high(input: &str) -> &str {
    input
        .split_whitespace()
        .min_by_key(|word| -word.chars().map(|c| (c as i32) - 96).sum::<i32>())
        .unwrap_or("")
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn print() {
    //     high("ehehehe ehe eheh");
    // }
    #[test]
    fn test_basic() {
        assert_eq!(high("man i need a taxi up to ubud"), "taxi");
        assert_eq!(high("what time are we climbing up the volcano"), "volcano");
        assert_eq!(high("take me to semynak"), "semynak");
        assert_eq!(high("massage yes massage yes massage"), "massage");
        assert_eq!(high("take two bintang and a dance please"), "bintang");
        assert_eq!(high("aa b"), "aa");
        assert_eq!(high("b aa"), "b");
        assert_eq!(high("bb d"), "bb");
        assert_eq!(high("d bb"), "d");
        assert_eq!(high("aaa b"), "aaa");
    }
}
