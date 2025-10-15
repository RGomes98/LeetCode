use std::collections::{HashMap, HashSet};

pub struct Solutions {
    pub longest_common_prefix: fn(strs: &[String]) -> String,
    pub valid_parentheses: fn(s: String) -> bool,
    pub search_insert: fn(nums: Vec<i32>, target: i32) -> i32,
}

impl Solutions {
    pub fn longest_common_prefix<'a>(strs: &'a [String]) -> String {
        let prefix_len: usize = strs.first().unwrap_or(&String::new()).len() + 1;
        let mut result: &'a str = "";

        for i in 1..prefix_len {
            let mut is_valid: bool = true;
            let mut curr: &str = "";

            for (j, str) in strs.iter().enumerate() {
                let s: Option<&str> = str.get(0..i);

                match s {
                    Some(s) if j == 0 => curr = s,
                    Some(s) if curr != s => is_valid = false,
                    None => is_valid = false,
                    _ => {}
                }
            }

            if is_valid {
                result = curr;
            } else {
                break;
            }
        }

        result.to_string()
    }

    pub fn valid_parentheses(str: String) -> bool {
        let map: HashMap<char, char> = HashMap::from([('}', '{'), (')', '('), (']', '[')]);
        let opening_brackets: HashSet<&char> = map.values().collect::<HashSet<&char>>();
        let closing_brackets: HashSet<&char> = map.keys().collect::<HashSet<&char>>();
        let mut stack: Vec<char> = vec![];

        for char in str.chars() {
            if let Some(opening_bracket) = opening_brackets.get(&char) {
                stack.push(**opening_bracket);
            }

            if let Some(closing_bracket) = closing_brackets.get(&char) {
                match (stack.last(), map.get(closing_bracket)) {
                    (Some(last_char), Some(opening_bracket)) if opening_bracket == last_char => {
                        stack.pop();
                    }
                    _ => return false,
                }
            }
        }

        stack.is_empty()
    }

    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut left: i32 = 0;
        let mut right: i32 = (nums.len() - 1) as i32;

        while left <= right {
            let middle: i32 = left + (right - left) / 2;
            let value: i32 = nums[middle as usize];

            if value < target {
                left = middle + 1;
            } else if value > target {
                right = middle - 1;
            } else {
                return middle;
            }
        }

        left
    }
}

fn main() {
    let strs: &[String; 3] = &["flower", "flow", "flight"].map(|str: &'static str| str.to_string());
    let longest_common_prefix: String = Solutions::longest_common_prefix(strs);
    assert!(longest_common_prefix == "fl");

    let valid_parentheses: bool = Solutions::valid_parentheses("([])".to_string());
    assert!(valid_parentheses);

    let search_insert: i32 = Solutions::search_insert(vec![1, 3, 5, 6], 5);
    assert!(search_insert == 2);
}
