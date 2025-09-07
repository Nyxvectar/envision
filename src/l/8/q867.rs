/**
 * Author:  Nyxvectar Yan 
 * Repo:    rustHello
 * Created: 09/07/2025
 */

struct Solution;

fn main() {
    let result = 1;
    println!("{:?}", result);
}

impl Solution {
    pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let origin_rows = matrix.len();
        let origin_columns = matrix[0].len();
        let mut result = vec![vec![0; origin_rows]; origin_columns];
        for (inter_row, row) in matrix.into_iter().enumerate() {
            for (inter_column, x) in row.into_iter().enumerate() {
                result[inter_column][inter_row] = x;
            }
        }
        result
    }
}
