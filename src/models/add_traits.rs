use itertools::Itertools;

/// Trait to facilitate the triming of white spaces
pub trait Trim {
    /// Trims the leading and trailing of a string
    fn trim_leading_trailing(&self) -> String;
}

/// Replace characters that can set issues for the dot file.
pub trait Replacable {
    /// Remove all non ascii chars or digits.
    fn replace_specials(&self) -> String;
    /// Remove all backquotes.
    fn replace_bq(&self) -> String;
}

/// Gets the last char
pub trait LastChar {
    /// Returns the last char
    fn get_last_char(&self) -> char;
}

/// Splits a string or str with a given vec
pub trait SplitVec {
    /// Splits vectorially a string
    ///
    /// # Arguments
    ///
    /// * `indexes` - Indexes to split
    fn split_vec(&self, indexes: Vec<usize>) -> Vec<&str>;
}

impl Trim for String {
    fn trim_leading_trailing(&self) -> String {
        self.trim_start().trim_end().to_string()
    }
}

impl Trim for str {
    fn trim_leading_trailing(&self) -> String {
        self.trim_start().trim_end().to_string()
    }
}

impl Replacable for str {
    fn replace_specials(&self) -> String {
        self.chars().filter(|c| c.is_ascii_alphanumeric() || c.is_ascii_whitespace()).collect::<String>()
    }
    fn replace_bq(&self) -> String {
        str::replace(self, "`", "")
    }
}

impl Replacable for String {
    fn replace_specials(&self) -> String {
        self.chars().filter(|c| c.is_ascii_alphanumeric() || c.is_ascii_whitespace()).collect::<String>()
    }
    fn replace_bq(&self) -> String {
        str::replace(self, "`", "")
    }
}

impl LastChar for String {
    fn get_last_char(&self) -> char {
        self.chars().last().unwrap()
    }
}

impl SplitVec for String {
    fn split_vec(&self, indexes : Vec<usize>) -> Vec<&str> {
        let self_len : usize = self.len();
        if indexes.is_empty() || self.is_empty() {
            return vec![self];
        }
        let mut cleaned_indexes : Vec<usize> = indexes.into_iter().unique().collect();
        cleaned_indexes.sort_unstable();
        if self_len <= cleaned_indexes.len() {
            panic!("A string can't be splitted more times than it has characters");
        } else if self_len <= cleaned_indexes.len() {
             panic!("A string can't be splitted at an index superior to its length");
        } else {
            let mut ret : Vec<&str> = Vec::new();
            cleaned_indexes.iter().enumerate().for_each(|(i, x)| {
                let slice : &str = match i {
                    0 => &self[0..*x],
                    _ => &self[cleaned_indexes[i-1]+1..*x]

                };
                if !slice.is_empty() {
                    ret.push(slice);
                }
            });
            let appendix : &str = &self[cleaned_indexes[cleaned_indexes.len()-1]+1..];
            if !appendix.is_empty(){
                ret.push(appendix);
            }
            ret
        }
    }
}



#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_trim_leading_trailing() {
        assert_eq!("  hello ".trim_leading_trailing(), "hello", "normal test case");
        assert_eq!(String::from("  hello ").trim_leading_trailing(), String::from("hello"), "normal test case");

        assert_eq!("  he llo ".trim_leading_trailing(), "he llo", "inside should't be trimmed");
        assert_eq!(String::from("  he llo ").trim_leading_trailing(), String::from("he llo"), "inside shouldn't be trimmed");

        assert_eq!(" \n\t\n he llo \t\n".trim_leading_trailing(), "he llo", "trim tabs and backtoline");
        assert_eq!(String::from(" \t\n\n he llo \t\n").trim_leading_trailing(), String::from("he llo"), "trim tabs and backtoline");
    }

    #[test]
    fn test_replace_specials() {
        assert_eq!("h*ù$$âe🔎,;:!)l&²l<o".replace_specials(), "hello");
       assert_eq!("\n\th ell o\t\n".replace_specials(), "\n\th ell o\t\n", "white spaces are preserved");
    }

    #[test]
    fn test_replace_backquotes() {
        assert_eq!("\n\th ell o\t\n".replace_bq(), "\n\th ell o\t\n", "no bq no rmval");
        assert_eq!("\n`\th `ell ``o\t`\n".replace_bq(), "\n\th ell o\t\n", "bq are removed");
        assert_eq!("\n`\th \"ell \"\"''``'o\t`\n".replace_bq(), "\n\th \"ell \"\"'''o\t\n", "other quotes aren't removed");
    }

    #[test]
    fn test_split_vec() {
        assert_eq!("foo bar".to_string().split_vec(vec![])[0], "foo bar", "no arg no split");
        assert_eq!("".to_string().split_vec(vec![])[0], "", "no input no split");
        assert!(std::panic::catch_unwind(|| {
            let test_input = "a".to_string();
            test_input.split_vec(vec![1]);
        }).is_err(), "can't split a single char");
        assert!(std::panic::catch_unwind(|| {
            let test_input = "ab".to_string();
            test_input.split_vec(vec![1, 2]);
        }).is_err(), "can't split as many times as there are characters");

        assert_eq!("abc".to_string().split_vec(vec![1]), vec!["a", "c"], "simple test");
        assert_eq!("abcd".to_string().split_vec(vec![1]), vec!["a", "cd"], "more than one character");
        assert_eq!("abcd".to_string().split_vec(vec![1, 1, 1, 1, 1]), vec!["a", "cd"], "same position repeated");
        assert_eq!("abcd".to_string().split_vec(vec![1, 3]), vec!["a", "c"], "same position repeated");
        assert_eq!("abcd".to_string().split_vec(vec![3, 1]), vec!["a", "c"], "ordering doesn't matter");
        assert_eq!("abcd".to_string().split_vec(vec![3, 2, 1]), vec!["a"], "ordering doesn't matter");
    }
}
