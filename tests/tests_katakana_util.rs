
#[cfg(test)]
mod tests_katakana_util {

    #[test]
    fn test_zenkaku_to_hankaku() {
        let input = "アイウエオ";
        let result = string_toolbox::katakana_util::zenkaku_to_hankaku(input);
        println!("result: {}", result);
        assert_ne!(input, result);
        assert_eq!("ｱｲｳｴｵ", result);
    }

    #[test]
    fn test_zenkaku_to_hankaku_dakuon() {
        let input = "アキハバラ";
        let result = string_toolbox::katakana_util::zenkaku_to_hankaku(input);
        println!("result: {}", result);
        assert_ne!(input, result);
        assert_eq!("ｱｷﾊﾊﾞﾗ", result);
    }

    #[test]
    fn test_hankaku_to_zenkaku() {
        let input = "ｱｲｳｴｵ";
        let result = string_toolbox::katakana_util::hankaku_to_zenkaku(input);
        println!("result: {}", result);
        assert_ne!(input, result);
        assert_eq!("アイウエオ", result);
    }

    #[test]
    fn test_hankaku_to_zenkaku_dakuon() {
        let input = "ｱｷﾊﾊﾞﾗ";
        let result = string_toolbox::katakana_util::hankaku_to_zenkaku(input);
        println!("result: {}", result);
        assert_ne!(input, result);
        assert_eq!("アキハバラ", result);
    }

}

