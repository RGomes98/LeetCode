pub struct Solutions {
    pub longest_common_prefix: fn(strs: Vec<String>) -> String,
}

impl Solutions {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let prefix_len: usize = strs.first().map(|s| s.len()).unwrap_or(0) + 1;
        let mut result = "";

        for i in 1..prefix_len {
            let mut is_valid: bool = true;
            let mut curr: &str = "";

            for (j, str) in strs.iter().enumerate() {
                let s: Option<&str> = str.get(0..i);

                match s {
                    Some(s) if j == 0 => curr = s,
                    Some(s) if curr != s => is_valid = false,
                    None => is_valid = false,
                    Some(_) => {}
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
}

fn main() {
    let longest_common_prefix: String = Solutions::longest_common_prefix(vec![
        "flower".to_string(),
        "flow".to_string(),
        "flight".to_string(),
    ]);

    assert!(longest_common_prefix == "fl")
}
