pub static LAYER_X_DEPTH: usize = 3;
pub static LAYER_Y_DEPTH: usize = 4;
pub static LAYER_Z_DEPTH: usize = 3;

/// merge_2_matrix(matrix1, matrix2)
///
/// will merge 2 matrix with same COL_SIZE and ROW_SIZE
/// you need to specify ROW_SIZE, COL_SIZE, and MERGED_COL_SIZE manually
///
/// for example if we got 2 matrix like below
///
/// ```rust
///  const COL_NUM: usize = 7;
///  const ROW_NUM: usize = 2;
///  const MERGED_COL_NUM: usize = COL_NUM * 2;
///
///  let matrix1 = [[1; COL_NUM]; ROW_NUM];
///  let matrix2 = [[2; COL_NUM]; ROW_NUM];
///
///  let merged = merge_2_matrix::<ROW_NUM, COL_NUM, MERGED_COL_NUM>(matrix1, matrix2);
///
///  println!("{merged:?}");
/// ```
///
/// that will return this data (look at the test )
/// ```rust
///  [
///     [1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2],
///     [1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2]
///  ]
/// ```
///
pub fn merge_2_matrix<
    const ROW_SIZE: usize,
    const COL_SIZE: usize,
    const MERGED_COL_SIZE: usize,
>(
    matrix1: [[u8; COL_SIZE]; ROW_SIZE],
    matrix2: [[u8; COL_SIZE]; ROW_SIZE],
) -> [[u8; MERGED_COL_SIZE]; ROW_SIZE] {
    let mut merged_matrix = [[0; MERGED_COL_SIZE]; ROW_SIZE];

    for (index, m1) in matrix1.iter().enumerate() {
        let m2 = &matrix2[index];
        merged_matrix[index][..COL_SIZE].copy_from_slice(m1);
        merged_matrix[index][COL_SIZE..].copy_from_slice(m2);
    }

    merged_matrix
}

/*
* Need to make api for registering
* Layer easily, example:
*   - Layer[0] = [
*       [1,2,3],
*       [1,2,3],
*       [1,2,3],
*   ]
*
*   - Layer[1] = [
*       [1,2,3],
*       [1,2,3],
*       [1,2,3],
*   ]
*
*   - Layer[2] = [
*       [1,2,3],
*       [1,2,3],
*       [1,2,3],
*   ]
*
* */

#[rustfmt::skip]
pub static LAYERS: [[[u8; LAYER_X_DEPTH]; LAYER_Y_DEPTH]; LAYER_Z_DEPTH] = 
[
  [
    [1, 2, 3],
    [4, 5, 6],
    [7, 8, 9],
    [10, 11, 12],
  ],
  [
    [13, 14, 15],
    [16, 17, 18],
    [19, 20, 21],
    [22, 23, 24],
  ],
  [
    [25, 26, 27],
    [28, 29, 30],
    [31, 32, 33],
    [34, 35, 36],
  ]
];

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn layer_accessing_test() {
        println!("20 is z=1, y=2, x=1 => {0}", LAYERS[1][2][1]);
        println!("36 is z=2, y=3, x=2 => {0}", LAYERS[2][3][2]);
    }

    #[test]
    fn merge_2_matrix_test() {
        const COL_NUM: usize = 7;
        const ROW_NUM: usize = 2;
        const MERGED_COL_NUM: usize = COL_NUM * 2;

        let matrix1 = [[1; COL_NUM]; ROW_NUM];
        let matrix2 = [[2; COL_NUM]; ROW_NUM];

        let merged = merge_2_matrix::<ROW_NUM, COL_NUM, MERGED_COL_NUM>(matrix1, matrix2);

        println!("{merged:?}");
    }

    #[test]
    fn two_matrix() {
        let mat1 = [1, 2, 3];
        let mat2 = [4, 5, 6];

        let mut merged_mat = [0; 6];

        for (index, m1) in mat1.iter().enumerate() {
            merged_mat[index] = *m1;
        }

        for (index, m2) in mat2.iter().enumerate() {
            merged_mat[index + mat1.len()] = *m2;
        }

        println!("merged_matrix {merged_mat:#?}");
    }

    #[test]
    fn two_dimension_matrix() {
        let mat1 = [[1, 2, 3], [4, 5, 6]];
        let mat2 = [[7, 8, 9], [10, 11, 12]];

        let mut merged_mat = [[0; 6]; 2];

        for (index, m1) in mat1.iter().enumerate() {
            let m2 = &mat2[index];
            merged_mat[index][..3].copy_from_slice(m1);
            merged_mat[index][3..].copy_from_slice(m2);
        }

        println!("merged_matrix {merged_mat:?}");
    }
}
