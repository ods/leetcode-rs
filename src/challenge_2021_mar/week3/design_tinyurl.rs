// Encode and Decode TinyURL
// https://leetcode.com/explore/challenge/card/march-leetcoding-challenge-2021/590/week-3-march-15th-march-21st/3673/

use std::{
    cell::{RefCell, RefMut},
    collections::HashMap,
    rc::Rc,
};

pub struct Codec(RefCell<(HashMap<Rc<String>, usize>, Vec<Rc<String>>)>);

const ALPHABET: &[u8; 62] =
    b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn base62_encode(buf: &mut String, mut num: usize) {
    if num == 0 {
        buf.push('0');
    } else {
        let mut code = Vec::new();
        while num > 0 {
            code.push(ALPHABET[num % 62]);
            num /= 62;
        }
        code.reverse();
        buf.push_str(std::str::from_utf8(code.as_slice()).unwrap());
    }
}

fn base62_decode(s: &str) -> usize {
    let mut num = 0;
    for c in s.chars() {
        num = num * 62
            + match c {
                '0'..='9' => c as usize - '0' as usize,
                'a'..='z' => c as usize - 'a' as usize + 10,
                'A'..='Z' => c as usize - 'A' as usize + 36,
                _ => panic!("Invalid code"),
            }
    }
    num
}

const SHORT_PREFIX: &str = "http://tinyurl.com/";

impl Codec {
    pub fn new() -> Self {
        Self(RefCell::new((HashMap::new(), Vec::new())))
    }

    // Encodes a URL to a shortened URL.
    pub fn encode(&self, long: String) -> String {
        let (mut by_long, mut by_idx) =
            RefMut::map_split(self.0.borrow_mut(), |borrow| {
                (&mut borrow.0, &mut borrow.1)
            });
        let long = Rc::new(long);
        let idx = *by_long.entry(long.clone()).or_insert_with(|| {
            by_idx.push(long);
            by_idx.len() - 1
        });

        let mut short = String::from(SHORT_PREFIX);
        base62_encode(&mut short, idx);
        short
    }

    // Decodes a shortened URL to its original URL.
    pub fn decode(&self, short: String) -> String {
        // Leetcode's version of rust have no `strip_prefix()`
        // let idx = base62_decode(short.strip_prefix(SHORT_PREFIX).unwrap());
        let idx = base62_decode(&short[SHORT_PREFIX.len()..]);
        (*self.0.borrow().1[idx]).clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base62_0() {
        let mut buf = String::new();
        base62_encode(&mut buf, 0);
        assert_eq!(buf, "0");
        assert_eq!(base62_decode("0"), 0);
    }

    #[test]
    fn test_base62_big() {
        let mut buf = String::new();
        base62_encode(&mut buf, 1234567890);
        assert_eq!(base62_decode(&buf), 1234567890);
    }

    #[test]
    fn simple_round() {
        let codec = Codec::new();
        let long1 = "https://leetcode.com/problems/encode-and-decode-tinyurl/";
        let long2 = "https://leetcode.com/problems/encode-and-decode-strings/";
        let short1 = codec.encode(long1.to_string());
        let short1_again = codec.encode(long1.to_string());
        assert_eq!(short1, short1_again);
        let short2 = codec.encode(long2.to_string());
        assert_eq!(codec.decode(short1), long1);
        assert_eq!(codec.decode(short2), long2);
    }
}
