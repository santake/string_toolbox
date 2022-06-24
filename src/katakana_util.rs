use super::katakana_dic::KatakanaDic;



/// Convert zenkaku katakana string with hankaku
/// 
pub fn zenkaku_to_hankaku(input: &str) -> String {
    let kana_dic = KatakanaDic::new();
    return kana_dic.zenkaku_to_hankaku(input);
}

/// Convert hankaku katakana string with zenkaku
/// 
pub fn hankaku_to_zenkaku(input: &str) -> String {
    let kana_dic = KatakanaDic::new();
    return kana_dic.hankaku_to_zenkaku(input);
}

