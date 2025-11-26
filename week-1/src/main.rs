use std::collections::HashMap;

/// Solusi dari Leetcode Longest Palindrome
/// https://leetcode.com/problems/longest-palindrome/
fn longest_palindrome(s: String) -> i32 {
    let mut length = 0;
    let mut charmap: HashMap<char, usize> = HashMap::with_capacity(52);
    for ch in s.chars() {
        charmap
            .entry(ch)
            .and_modify(|counter| {
                *counter += 1;
                if counter.is_multiple_of(2) {
                    length += 2;
                }
            })
            .or_insert(1);
    }
    for occurence in charmap.values() {
        if occurence % 2 == 1 {
            length += 1;
            break;
        }
    }
    length
}

fn main() {
    let test_cases = HashMap::from([("abccccdd", 7), ("a", 1)]);
    for (k, v) in test_cases.iter() {
        let val = longest_palindrome(k.to_string());
        if val != *v {
            println!("Gagal di {}:\n Harusnya {}, malah {}", k, v, val);
            break;
        }
    }
    println!("Semua tes berhasil");
}
