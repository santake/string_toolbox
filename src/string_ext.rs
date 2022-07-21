use regex::Regex;

pub trait StringExt {
    
    /// (static method)
    ///  format number to multi-digit-string with specifying the digits
    /// e.g. 
    /// ```text
    /// 3 (2 digits) --> '03'
    /// 11 (4 digits) --> '0011'
    /// 134 (5 digits) --> '00134'
    /// ```
    fn fmt_zero_filled(num: u32, digit: u32) -> String;

    
    /// (static method)
    /// Join the chunks of strings to one string with delimiter character
    /// 
    fn join(inputs:&[&str], delimiter: &str) -> String;

    /// (static method)
    /// Join the chunk of strings as 'snake-case' that Rust likes.
    /// 
    fn snake_case(inputs:&[&str]) -> String;

    
    /// Remove all space characters from input string (&str) including Japanese Zenkaku-space. 
    /// (this also invokes 'String.trim()' at first)
    /// 
    fn remove_all_spaces(&self) -> Self;
    
    /// Replace all substring that matches the given regex = '`pattern`' with the given replacement = '`replacement`'
    /// from the '`input`' string.
    /// 
    fn replace_all(&self, pattern:&str, replacement:&str) -> Self;
    
    /// Trim all 'multiple' spaces into one space (including japanese zenkaku-space)
    /// e.g. 
    /// ```text
    /// "a   b" --> "a b"
    /// ```
    /// 
    fn trim_redundant_spaces(&self) -> Self;

    /// Substring a string with begin and end inex
    /// 
    fn substring(&self, begin_index: u32,  end_index: u32) -> Self;

    /// Capitalize a string changing the first character to title case (upper case)
    /// 
    fn capitalize(&self) -> Self;

    /// Truncate (substring) a string with specified length/width == max_width.
    /// 
    fn truncate(&self, max_width:u32) -> Self;

    /// Reverse all characters in a string
    /// 
    fn reverse(&self) -> Self;

    /// Concatenate two strings into one. The first one should be the self, and add the second one.
    /// 
    fn concat(&self, second:&str) -> Self;

}



impl StringExt for String {
    
    // -- (static methods) -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --     
    fn fmt_zero_filled(num: u32, digit: u32) -> String {
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

    fn join(inputs:&[&str], delimiter: &str) -> String {
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
    
    fn snake_case(inputs:&[&str]) -> String {
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
    // -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- 



    fn remove_all_spaces(&self) -> Self {
        if self.is_empty() {
            return self.to_string();
        }

        let tmp = self.trim();
        let chars: Vec<char> = tmp.chars().collect();
        let mut result = String::new();
        for ch in chars {
            if  ch != ' ' && ch != '　' {
                result.push(ch);
            }
        }
        return result;
    }

    fn replace_all(&self, pattern:&str, replacement:&str) -> Self {
        if self.is_empty() {
            return self.to_string();
        }
        let regex = Regex::new(pattern).unwrap();
        return regex.replace_all(self, replacement).to_string();
    }


    fn trim_redundant_spaces(&self) -> Self {
        if self.is_empty() {
            return self.to_string();
        }
        return self.trim().to_string().replace_all(r"[　 ]+", " ");
    }

    fn substring(&self, begin_index: u32,  end_index: u32) -> Self {
        if self.is_empty() {
            return self.to_string();
        }
        let tmp = self.trim();
        let chars = tmp.chars().into_iter();
        let mut result = String::new();
        for (i, ch) in chars.enumerate() {
            if begin_index <= (i as u32) && (i as u32) < end_index {
                result.push(ch);
            }
        }
        return result.to_string();
    }

    fn capitalize(&self) -> Self {
        if self.is_empty() {
            return self.to_string();
        }
        let first = self.substring(0, 1);
        let rest = self.substring( 1, self.len() as u32);
        let mut upper = first.to_uppercase();    
        upper.push_str(&rest);
        return upper;
    }

    fn truncate(&self, max_width:u32) -> Self {
        if (self.len() as u32) <= max_width {
            return self.to_string();
        }
        return self.substring(0, max_width);
    }

    fn reverse(&self) -> Self {
        let mut chnk:Vec<char> = self.chars().collect();
        chnk.reverse();
        let mut result = String::new();
        for c in chnk {
            result.push(c);
        }
        return result;
    }


    fn concat(&self, second:&str) -> Self {
        let mut result = String::new();
        result.push_str(self);
        result.push_str(second);
        return result;
    }
    
}