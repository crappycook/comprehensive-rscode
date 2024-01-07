#[allow(unused)]
pub fn gen_array_and_change() -> [i32; 10] {
    let mut arr = [10; 10];
    // Do something with arr here
    arr[2] = 2;
    println!("{:?}", arr);
    arr
}

#[allow(unused)]
fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let m = matrix.len();
    let n = matrix[0].len();
    let mut res = [[0; 3]; 3];
    for i in 0..m {
        for j in 0..n {
            res[i][j] = matrix[j][i]
        }
    }
    res
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_modify_array() {
        let expected = [10, 10, 2, 10, 10, 10, 10, 10, 10, 10];
        let result = super::gen_array_and_change();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_transpose() {
        let matrix = [
            [101, 102, 103], // <-- the comment makes rustfmt add a newline
            [201, 202, 203],
            [301, 302, 303],
        ];
        assert_eq!(
            super::transpose(matrix),
            [
                [101, 201, 301], // <-- the comment makes rustfmt add a newline
                [102, 202, 302],
                [103, 203, 303]
            ]
        );
    }
}
