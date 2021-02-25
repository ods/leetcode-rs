struct Solution;

use std::path::{
    Component::{Normal, ParentDir},
    Path,
};

impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut parts = Vec::new();
        for component in Path::new(&path).components() {
            match component {
                Normal(s) => {
                    parts.push(s.to_str().unwrap());
                }
                ParentDir => {
                    parts.pop();
                }
                _ => {}
            }
        }
        if parts.is_empty() {
            String::from("/")
        } else {
            let mut res = String::new();
            for part in parts {
                res.push('/');
                res.push_str(part)
            }
            res
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn check(src: &str, expected: &str) {
        assert_eq!(
            Solution::simplify_path(String::from(src)),
            String::from(expected)
        );
    }

    #[test]
    fn example1() {
        check("/home/", "/home");
    }

    #[test]
    fn example2() {
        check("/../", "/");
    }

    #[test]
    fn example3() {
        check("/home//foo/", "/home/foo");
    }

    #[test]
    fn example4() {
        check("/a/./b/../../c/", "/c");
    }
}
