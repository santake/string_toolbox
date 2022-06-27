use regex::Regex;


/// Check if the input string starts w/ the 'ptrn' (pattern) or not. 
/// 
pub fn starts_with(input:&str, ptrn:&str) -> bool {
    if input.is_empty() {
        false
    } else {
        input.starts_with(ptrn)
    }
}

/// Check if the input string ends w/ the 'ptrn' (pattern) or not
/// 
pub fn ends_with(input:&str, ptrn:&str) -> bool {
    if input.is_empty() {
        false
    } else {
        input.ends_with(ptrn)
    }
    
}


/// Remove all space characters from input string (&str) including Japanese Zenkaku-space. 
/// (this also invokes 'String.trim()' at first)
/// 
pub fn remove_all_spaces(input:&str) -> String {
    if input.is_empty() {
        return input.to_string();
    }
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
    if input.is_empty() {
        return input.to_string();
    }
    return replace_all(input.trim(), r"[　 ]+", " ");
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


/// Replace all substring that matches the given regex = '`pattern`' with the given replacement = '`replacement`'
/// from the '`input`' string.
/// 
pub fn replace_all(input:&str, pattern:&str, replacement:&str) -> String {
    if input.is_empty() {
        return input.to_string();
    }
    let regex = Regex::new(pattern).unwrap();
    return regex.replace_all(input, replacement).to_string();
}


/// Capitalizes a String changing the first character to title case (upper case)
/// 
pub fn capitalize(input: &str) -> String {
    if input.is_empty() {
        return input.to_string();
    }
    let first = substring(input, 0, 1);
    let rest = substring(input, 1, input.len() as u32);
    let mut upper = first.to_uppercase();    
    
    upper.push_str(&rest);

    return upper;
}


/// Substring a String with begin and end index
/// 
pub fn substring(input: &str, begin_index: u32,  end_index: u32) -> String {
    if input.is_empty() {
        return input.to_string();
    }
    let tmp = input.trim();
    let chars = tmp.chars().into_iter();
    let mut result = String::new();
    for (i, ch) in chars.enumerate() {
        if begin_index <= (i as u32) && (i as u32) < end_index {
            result.push(ch);
        }
    }
    return result.to_string();
}

/// Simply concatenate two string into one string
/// (wrapper of push_str)
/// 
pub fn concat(first:&str, second:&str) -> String {
    let mut result = String::new();
    result.push_str(first);
    result.push_str(second);
    return result;
}

/// Join the chunks of strings to one string with delimiter character
/// 
pub fn join(inputs:Vec<&str>, delimiter: &str) -> String {
    let size = inputs.len();
    let mut result = String::new();
    for (i, str) in inputs.iter().enumerate() {
        result.push_str(str);
        if 0 <= i as i32 && i < (size - 1) {
            result.push_str(delimiter);
        }
    }
    return result.to_string();
}


/// Join the chunk of strings as 'snake-case' that Rust likes.
/// 
pub fn snake_case(inputs:Vec<&str>) -> String {
    let delimiter = "_";
    let size = inputs.len();
    let mut result = String::new();
    for (i, str) in inputs.iter().enumerate() {
        result.push_str(&str.to_owned().to_lowercase());
        if 0 <= i as i32 && i < (size - 1) {
            result.push_str(delimiter);
        }
    }
    return result.to_string();

}