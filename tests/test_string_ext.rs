

#[cfg(test)]
mod test_string_ext {

    use string_toolbox::string_ext::StringExt;    

    // ---- tests for static methods -------------------
    #[test]
    fn test_fmt_fill_zero() {
        let num = 3;
        let result = String::fmt_zero_filled(num, 3);
        assert_eq!(result, "003");
        let num = 12;
        let result = String::fmt_zero_filled(num, 5);
        assert_eq!(result, "00012");
    }

    #[test]
    fn test_join() {
        //let inputs: Vec<&str> = ["ab", "cd", "ef"].to_vec();
        let inputs: &[&str] = &["ab", "cd", "ef", "gh", "ij"];
        let delim = ",";
        let result = String::join(inputs, delim);
        assert_eq!("ab,cd,ef,gh,ij", result);
    }

    #[test]
    fn test_snake_case() {
        //let inputs: Vec<&str> = ["Ab", "Cd", "EF"].to_vec();
        let inputs: &[&str] = &["Ab", "Cd", "EF"];
        let result = String::snake_case(inputs);
        assert_eq!("ab_cd_ef", result);
    }

    #[test]
    fn test_repeat() {
        let text = "aaa";
        let sep = "-";
        let repeat = 3;
        let result = String::repeat(text, repeat, sep);
        assert_eq!("aaa-aaa-aaa", result);

        let result = String::repeat(text, repeat, "");
        assert_eq!("aaaaaaaaa", result);
        let result = String::repeat(text, repeat, " ");
        assert_eq!("aaa aaa aaa", result);


    }


    // ---- tests for string extension methods -------------------

    #[test]
    fn test_remove_all_spaces() {
        let input = "a b  c d e".to_string();
        let result = input.remove_all_spaces();
        assert_eq!("abcde", result);
    }
    
    #[test]
    fn test_replace_all() {
        let input = "XXXabcYYY".to_string();
        let pattern = "abc";
        let result = input.replace_all(pattern, "o");
        assert_eq!(result, "XXXoYYY");

        let input = " a b c d e ".to_string();
        let pattern = r"[　 ]+";
        let result = input.replace_all(pattern, "");
        assert_eq!(result, "abcde");
    }

    
    #[test]
    fn test_trim_redundant_spaces() {
        let input = "a  　b  c 　 d  e".to_string();
        let result = input.trim_redundant_spaces();
        println!("result='{}'", result);
        assert_eq!(result, "a b c d e");
    }
    
    #[test]
    fn test_substring() {
        let input = "abcdefghij".to_string();
        let result = input.substring(2, 5);
        assert_eq!("cde", result);
    }

    #[test]
    fn test_capitalize() {
        let input = "abc".to_string();
        let result = input.capitalize();
        assert_eq!("Abc", result);
    }
    
    #[test]
    fn test_reverse() {
        let input = "abcde".to_string();
        let result = input.reverse();
        assert_eq!("edcba", result);
    }

    #[test]
    fn test_truncate() {
        // max_widthが入力よりも長い場合はinputが戻る
        let input = "ab".to_string();
        let max_width = 4;
        let result = input.truncate(max_width);
        assert_eq!(input, result);

        let input = "abcdEfgHI".to_string();
        let max_width = 5;
        let result = input.truncate(max_width);
        assert_eq!("abcdE", result);
    }

    #[test]
    fn test_swap_case() {
        let input = "AbC dEf-GhI".to_string();
        let result = input.swap_case();
        assert_eq!("aBc DeF-gHi", result);
    }


}