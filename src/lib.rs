// ------------------------------------------------------------------------------------------
/*!
String utilities (toolbox) that are not provided by `std::string`.
It also has Japanese 'Katakana' character utilities.
*/
// ------------------------------------------------------------------------------------------


///
/// A trait for standard 'String' to be able to use additional utility methods.
/// You have to import string_toolbox:string_ext::StringExt trait in your scope,
/// and you will be able to use additional methods like:
/// 
/// e.g.
/// ```
/// use string_toolbox::string_ext::StringExt;    
///
/// let num = 3;
/// let result = String::fmt_zero_filled(num, 3);
/// assert_eq!(result, "003");
/// 
/// let input = "a b  c d e".to_string();
/// let result = input.remove_all_spaces();
/// ```
/// 
pub mod string_ext;


///
/// Utility methods for Japanese 'Katakana' String
/// e.g.
/// ```rust
/// let input = "ｱｷﾊﾊﾞﾗ";
/// let result = string_toolbox::katakana_util::hankaku_to_zenkaku(input);
/// println!("result: {}", result);
/// assert_ne!(input, result);
/// assert_eq!("アキハバラ", result);
/// ```
pub mod katakana_util;


/// (internal) Katakana Dictionary
mod katakana_dic;




