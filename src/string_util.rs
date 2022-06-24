use regex::Regex;

/// Check if the input string starts w/ the 'ptrn' (pattern) or not. 
/// 
pub fn starts_with(input:&str, ptrn:&str) -> bool {
    input.starts_with(ptrn)
}

/// Check if the input string ends w/ the 'ptrn' (pattern) or not
/// 
pub fn ends_with(input:&str, ptrn:&str) -> bool {
    input.ends_with(ptrn)
}

/// Remove all space characters from input string (&str) including Japanese Zenkaku-space. 
/// (this also invokes 'String.trim()' at first)
/// 
pub fn remove_all_spaces(input:&str) -> String {
    let tmp = input.trim();

    let chars: Vec<char> = tmp.chars().collect();
    let mut result = String::new();
    for ch in chars {
        if  ch != ' ' && ch != '　' {
            result.push(ch);
        }
    }
    return result;
}


/// Trim all 'multiple' spaces into one space (including japanese zenkaku-space)
/// 
/// e.g. 
/// ```text
/// "a   b" --> "a b"
/// ```
/// 
pub fn trim_redundant_spaces(input:&str) -> String {
    let tmp = input.trim();
    let regex = Regex::new(r"[　 ]+").unwrap();
    return regex.replace_all(tmp, " ").to_string();
} 


/// convert number to multi-digit-string with specifying the digits
/// 
/// e.g. 
/// ```text
/// 3 (2 digits) --> '03'
/// 11 (4 digits) --> '0011'
/// 134 (5 digits) --> '00134'
/// ```
/// 
pub fn to_zero_filled_string(num: u32, digit: u32) -> String {
    let str = num.to_string();

    let mut result = String::new();
    if (str.len() as u32) < digit {
        let dif = digit - (str.len() as u32);
        for _ in 0..dif {
            result.push_str(&"0");
        }
    }
    result.push_str(&str);
    return result;
}
