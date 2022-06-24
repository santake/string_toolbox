// ------------------------------------------------------------------------------------------
/*!
String utilities (toolbox) that are not provided by `std::string`.
It also has Japanese 'Katakana' character utilities.

e.g.
```rust
// Removing all redundant spaces:
let input = "a b c d e ";
let result = string_toolbox::string_util::remove_all_spaces(input);
assert_eq!(result, "abcde");
```

e.g.
```rust
// convert Hankaku-Katakana w/ Zenkaku-Katakana:
let input = "ｱｷﾊﾊﾞﾗ";
let result = string_toolbox::katakana_util::hankaku_to_zenkaku(input);
assert_eq!("アキハバラ", result);
```

*/
// ------------------------------------------------------------------------------------------

///
/// Utiltiy methods for general String
/// e.g.
/// ```rust
/// let input = "a b c d e ";
/// let result = string_toolbox::string_util::remove_all_spaces(input);
/// ```
pub mod string_util;

///
/// Utility methods for Japanese 'Katakana' String
/// e.g.
/// ```rust
///  let input = "ｱｷﾊﾊﾞﾗ";
/// let result = string_toolbox::katakana_util::hankaku_to_zenkaku(input);
/// println!("result: {}", result);
/// assert_ne!(input, result);
/// assert_eq!("アキハバラ", result);
/// ```
pub mod katakana_util;


/// (internal) Katakana Dictionary
mod katakana_dic;