#[macro_export]
macro_rules! matrix {
    [ $( [ $( $cell:expr ),* $(,)? ] ),* $(,)? ] => {
        vec![
            $(
                vec![ $( $cell ),* ]
            ),*
        ]
    };
    [ $cell:expr ; $cols:expr ; $rows:expr ] => {
        vec![vec![$cell;$cols];$rows]
    };
}

#[cfg(test)]
mod test {
    #[test]
    fn empty() {
        let m1: Vec<Vec<i32>> = matrix![];
        let m2: Vec<Vec<i32>> = vec![];
        assert_eq!(m1, m2);
    }

    #[test]
    fn simple() {
        assert_eq!(matrix![[1, 2], [3, 4]], vec![vec![1, 2], vec![3, 4]]);
    }

    #[test]
    fn trailing_comma() {
        assert_eq!(
            matrix![
                [
                    111111111, 222222222, 333333333, 444444444, 555555555,
                    666666666,
                ],
                [
                    222222222, 333333333, 444444444, 555555555, 666666666,
                    777777777,
                ],
            ],
            vec![
                vec![
                    111111111, 222222222, 333333333, 444444444, 555555555,
                    666666666,
                ],
                vec![
                    222222222, 333333333, 444444444, 555555555, 666666666,
                    777777777,
                ]
            ]
        );
    }
}
