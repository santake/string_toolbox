#[cfg(test)]
mod tests_string_util {
    #[test]
    fn test_starts_with() {
        let val = string_toolbox::string_util::starts_with("abcde", "ab");
        assert_eq!(val, true);
        let val = string_toolbox::string_util::starts_with("abcde", "bc");
        assert_ne!(val, true);
    }

    #[test]
    fn test_ends_with() {
        let val = string_toolbox::string_util::ends_with("abcde", "de");
        assert_eq!(val, true);
        let val = string_toolbox::string_util::ends_with("abcde", "ab");
        assert_ne!(val, true);
    }

    #[test]
    fn test_remove_all_spaces() {
        let input = "a b c d e ";
        let result = string_toolbox::string_util::remove_all_spaces(input);
        assert_eq!(result, "abcde");
        // 全角スペース:
        let input = "a　b　c　d　e　";
        let result = string_toolbox::string_util::remove_all_spaces(input);
        assert_eq!(result, "abcde")
    }

    #[test]
    fn test_trim_redundant_spaces() {
        let input = "a  　b  c 　 d  e";
        let result = string_toolbox::string_util::trim_redundant_spaces(input);
        println!("result='{}'", result);
        assert_eq!(result, "a b c d e");
    }
    
    #[test]
    fn test_to_zero_fill_string() {
        let num = 3;
        let result = string_toolbox::string_util::to_zero_filled_string(num, 3);
        assert_eq!(result, "003");
        let num = 12;
        let result = string_toolbox::string_util::to_zero_filled_string(num, 5);
        assert_eq!(result, "00012");
    }

}
