#[macro_export]
macro_rules! string_vec {
    [ $( $cell:expr ),* $(,)? ] => {
        vec![
            $( String::from($cell) ),*
        ]
    };
    [ $cell:expr ; $count:expr ] => {
        vec![String::from($cell);$count]
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn empty() {
        let v1: Vec<String> = string_vec![];
        let v2: Vec<String> = vec![];
        assert_eq!(v1, v2);
    }

    #[test]
    fn simple() {
        let v1 = string_vec!["abc", "bcd"];
        let v2 = vec!["abc".to_string(), "bcd".to_string()];
        assert_eq!(v1, v2);
    }

    #[test]
    fn repeated() {
        let v1 = string_vec!["abc"; 3];
        let v2 = vec!["abc".to_string(); 3];
        assert_eq!(v1, v2);
    }
}
