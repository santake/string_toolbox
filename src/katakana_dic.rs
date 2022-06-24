use std::collections::HashMap;

use crate::string_util;


static HAN_KANA_ALL:&str = "ｱｲｳｴｵｶｷｸｹｺｻｼｽｾｿﾀﾁﾂﾃﾄﾅﾆﾇﾈﾉﾊﾋﾌﾍﾎﾏﾐﾑﾒﾓﾔﾕﾖﾗﾘﾙﾚﾛﾜｦﾝｧｨｩｪｫｯｬｭｮｰ";
static ZEN_KANA_ALL:&str = "アイウエオカキクケコサシスセソタチツテトナニヌネノハヒフヘホマミムメモヤユヨラリルレロワヲンァィゥェォッャュョー";
static HAN_DAKU_ALL:[&str; 26] = [
    "ｶﾞ","ｷﾞ","ｸﾞ","ｹﾞ","ｺﾞ","ｻﾞ","ｼﾞ","ｽﾞ","ｾﾞ","ｿﾞ",
    "ﾀﾞ","ﾁﾞ","ﾂﾞ","ﾃﾞ","ﾄﾞ","ﾊﾞ","ﾋﾞ","ﾌﾞ","ﾍﾞ","ﾎﾞ",
    "ﾊﾟ","ﾋﾟ","ﾌﾟ","ﾍﾟ","ﾎﾟ","ｳﾞ"
];
static ZEN_DAKU_ALL:[&str; 26] = [
    "ガ","ギ","グ","ゲ","ゴ","ザ","ジ","ズ","ゼ","ゾ",
    "ダ","ヂ","ヅ","デ","ド","バ","ビ","ブ","ベ","ボ",
    "パ","ピ","プ","ペ","ポ","ヴ"
];

///
/// 
pub struct KatakanaDic {
    zen_han_dic: HashMap<String, String>,
    han_zen_dic: HashMap<String, String>,
}

///
/// Dictionary class; and additional methods for Katakana 
/// 
impl KatakanaDic {

    /// Default constructor
    pub fn new() -> Self {
        let mut hash_zen_han = HashMap::new();
        let mut hash_han_zen = HashMap::new();

        let han_chars: Vec<char> = HAN_KANA_ALL.chars().collect();
        let zen_chars: Vec<char> = ZEN_KANA_ALL.chars().collect();
        for i in 0..han_chars.len() {
            hash_zen_han.insert(zen_chars[i].to_string(), han_chars[i].to_string());
            hash_han_zen.insert(han_chars[i].to_string(), zen_chars[i].to_string());
        }
        // 半角の濁音撥音の処理は別に行う
        for i in 0..HAN_DAKU_ALL.len() {
            hash_zen_han.insert(ZEN_DAKU_ALL[i].to_string(), HAN_DAKU_ALL[i].to_string());
            //半角->全角は濁音は個別処理するのでmap不要
        }
        Self {
            zen_han_dic: hash_zen_han,
            han_zen_dic: hash_han_zen,
        }
    } //new()


    /// Convert Zenkaku katakana string with Hankaku 
    /// 
    pub fn zenkaku_to_hankaku(&self, input: &str) -> String {
        let tmp = input.trim();
        let chars: Vec<char> = tmp.chars().collect();
        let mut result = String::new();
        chars.into_iter().for_each(|ch| {
            let newch = self.zen_to_han(&ch.to_string());
            result.push_str(&newch);
        });
        return result;
    }

    
    /// Convert Hankaku katakana string with Zenkaku
    /// 
    pub fn hankaku_to_zenkaku(&self, input: &str) -> String {
        // 先に濁音などを全角に置換(半角の場合濁音が2文字に展開されているので正しく処理不可能)
        let tmp = &self.replace_hankaku_dakuon(input.trim());

        let chars: Vec<char> = tmp.chars().collect();
        let mut result = String::new();
        chars.into_iter().for_each(|ch| {
            let newch = self.han_to_zen(&ch.to_string());
            result.push_str(&newch);
        });
        return result;
    }

    /// Replace hankaku dakuon(han-dakuon) with zenkaku
    /// (hankaku dakuon is made with two characters)
    /// 
    fn replace_hankaku_dakuon(&self, input:&str) -> String {
        let mut tmp = input.to_string();
        for i in 0..HAN_DAKU_ALL.len() {
            tmp = string_util::replace_all(&tmp, HAN_DAKU_ALL[i], ZEN_DAKU_ALL[i]);

            //let regex = Regex::new(HAN_DAKU_ALL[i]).unwrap();
            //tmp = regex.replace_all(&tmp, ZEN_DAKU_ALL[i]).to_string();

        }
        return tmp;
    }

    /// convert char hankaku -> zenkaku
    fn han_to_zen(&self, ch:&str) -> String {
        match self.han_zen_dic.get(ch) {
            Some(zen) => zen.to_string(),
            None => ch.to_string()
        }
    }// han => zen

    /// convert char zenkaku -> hankaku
    fn zen_to_han(&self, ch:&str) -> String {
        match self.zen_han_dic.get(ch) {
            Some(han) => han.to_string(),
            None => ch.to_string()
        }
    }// zen => han


}