
#[derive(Copy, Clone, Debug)]
pub struct Matrix4 {
    matrix: [[f32; 4]; 4]
}

impl Matrix4 {

    fn new() -> Self {
        Matrix4 {
            matrix: [[0.0; 4]; 4]
        }
    }


    fn identity_matrix() -> Self {
        Matrix4 {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0]
            ]
        }
    }


    fn transpose(&self) -> Self {
        let mut result = Matrix4::new();

        for i in 0..self.matrix.len() {
            for j in 0..self.matrix[0].len() {
                result.matrix[i][j] = self.matrix[j][i]
            }
        }

        result
    }


    fn transpose_mut(&mut self) {
        for i in 0..self.matrix.len() {
            for j in i + 1..self.matrix[0].len() {
                (self.matrix[i][j], self.matrix[j][i]) = (self.matrix[j][i], self.matrix[i][j])
            }
        }
    }


    fn submatrix(&self, row_to_remove: usize, col_to_remove: usize) -> Matrix3 {
        let mut result = Matrix3::new();
        let mut submatrix_row = 0;

        for i in 0..4 {
            let mut submatrix_col = 0;

            if row_to_remove == i { continue; }

            for j in 0..4 {

                if col_to_remove == j { continue; }
                result.matrix[submatrix_row][submatrix_col] = self.matrix[i][j];
                submatrix_col += 1;
            }

            submatrix_row += 1;
        }

        result
    }

}

impl PartialEq for Matrix4 {
    fn eq(&self, other: &Self) -> bool {
        for (row_self, row_other) in self.matrix.iter().zip(other.matrix.iter()) {
            for (element_self, element_other) in row_self.iter().zip(row_other.iter()) {
                if element_self != element_other {
                    return false;
                }
            }
        }
        true
    }

    fn ne(&self, other: &Self) -> bool {
        for (row, row_other) in self.matrix.iter().zip(other.matrix.iter()) {
            for (element_self, element_other) in row.iter().zip(row_other.iter()) {
                if element_self == element_other {
                    return false;
                }
            }
        }
        true
    }
}

impl std::ops::Mul for Matrix4 {
    type Output = Matrix4;

    /*
    fn mul(self, rhs: Self) -> Self::Output {
        let mut result = Matrix4::new();

        for i in 0..result.matrix.len() {
            for j in 0..result.matrix[0].len() {
                for k in 0..result.matrix.len() {
                    result.matrix[i][j] += self.matrix[i][k] * rhs.matrix[k][j]
                }
            }
        }

        result
    }
    */

    fn mul(self, rhs: Self) -> Self::Output {
        let mut result = Matrix4::new();

        for i in 0..4 {
            for j in 0..4 {
                result.matrix[i][j] =
                    self.matrix[i][0] * rhs.matrix[0][j]
                        + self.matrix[i][1] * rhs.matrix[1][j]
                        + self.matrix[i][2] * rhs.matrix[2][j]
                        + self.matrix[i][3] * rhs.matrix[3][j];
            }
        }

        result
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Matrix3 {
    matrix: [[f32; 3]; 3]
}

impl Matrix3 {
    fn new() -> Self {
        Matrix3 {
            matrix: [[0.0; 3]; 3]
        }
    }


    fn submatrix(&self, row_to_remove: usize, col_to_remove: usize) -> Matrix2 {
        let mut result = Matrix2::new();
        let mut submatrix_row = 0;

        for i in 0..3 {
            let mut submatrix_col = 0;

            if row_to_remove == i { continue; }

            for j in 0..3 {

                if col_to_remove == j { continue; }
                result.matrix[submatrix_row][submatrix_col] = self.matrix[i][j];
                submatrix_col += 1;
            }

            submatrix_row += 1;
        }

        result
    }
}


#[derive(Copy, Clone, Debug)]
pub struct Matrix2 {
    matrix: [[f32; 2]; 2]
}

impl Matrix2 {
    fn new() -> Self {
        Matrix2 {
            matrix: [[0.0; 2]; 2]
        }
    }

    fn determinant(&self) -> f32 {
        self.matrix[0][0] * self.matrix[1][1] - self.matrix[0][1] * self.matrix[1][0]
    }
}

impl PartialEq for Matrix2 {
    fn eq(&self, other: &Self) -> bool {
        for (row_self, row_other) in self.matrix.iter().zip(other.matrix.iter()) {
            for (element_self, element_other) in row_self.iter().zip(row_other.iter()) {
                if element_self != element_other {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use crate::matrices::{Matrix2, Matrix3, Matrix4};

    #[test]
    fn create_4x4() {
        let matrix4 = Matrix4::new();
        let mut matrix = matrix4.matrix;

        matrix[0][0] = 1.5;

        let expected = 1.5;

        assert_eq!(matrix[0][0], expected)
    }

    #[test]
    fn matrix_eq() {
        let matrix_one = Matrix4 { matrix: [
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0]
        ] };

        let matrix_two = Matrix4 { matrix: [
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0]
        ] };

        assert_eq!(matrix_one, matrix_two)
    }


    #[test]
    fn matrix_ne() {
        let matrix_one = Matrix4 { matrix: [
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0]
        ] };

        let matrix_two = Matrix4 { matrix: [
            [2.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0]
        ] };

        assert_ne!(matrix_one, matrix_two)
    }


    #[test]
    fn matrix_mul() {
        let matrix_one = Matrix4 { matrix: [
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0]
        ] };

        let matrix_two = Matrix4 { matrix: [
            [-2.0, 1.0, 2.0, 3.0],
            [3.0, 2.0, 1.0, -1.0],
            [4.0, 3.0, 6.0, 5.0],
            [1.0, 2.0, 7.0, 8.0]
        ] };


         let expected = Matrix4 { matrix: [
            [20.0, 22.0, 50.0, 48.0],
            [44.0, 54.0, 114.0, 108.0],
            [40.0, 58.0, 110.0, 102.0],
            [16.0, 26.0, 46.0, 42.0]
        ]};

        assert_eq!(matrix_one * matrix_two, expected)
    }


    #[test]
    fn identity_matrix_mul() {
        let matrix = Matrix4 { matrix: [
            [0.0, 1.0, 2.0, 4.0],
            [1.0, 2.0, 4.0, 8.0],
            [2.0, 4.0, 8.0, 16.0],
            [4.0, 8.0, 16.0, 32.0]
        ] };

        let identity_matrix = Matrix4::identity_matrix();

        assert_eq!(matrix * identity_matrix, matrix)
    }

    #[test]
    fn transpose_matrix() {
        let matrix = Matrix4 { matrix: [
            [0.0, 9.0, 3.0, 0.0],
            [9.0, 8.0, 0.0, 8.0],
            [1.0, 8.0, 5.0, 3.0],
            [0.0, 0.0, 5.0, 8.0]
        ] };

        let expected = Matrix4 { matrix: [
            [0.0, 9.0, 1.0, 0.0],
            [9.0, 8.0, 8.0, 0.0],
            [3.0, 0.0, 5.0, 5.0],
            [0.0, 8.0, 3.0, 8.0]
        ]};

        let actual = matrix.transpose();

        assert_eq!(actual, expected)
    }


    #[test]
    fn transpose_identity_matrix() {
        let mut identity_matrix = Matrix4::identity_matrix();

        let expected = Matrix4::identity_matrix();

        identity_matrix.transpose_mut();

        assert_eq!(identity_matrix, expected)
    }

    #[test]
    fn determinant_matrix2() {
        let matrix = Matrix2 { matrix: [
            [1.0, 5.0],
            [-3.0, 2.0]
        ]};

        let determinant = matrix.determinant();

        let expected = 17.0;

        assert_eq!(determinant, expected);
    }

    #[test]
    fn submatrix() {
        let matrix = Matrix3 { matrix: [
            [1.0, 5.0, 0.0],
            [-3.0, 2.0, 7.0],
            [0.0, 6.0, -3.0]
        ]};

        let expected = Matrix2 { matrix: [
            [-3.0, 2.0],
            [0.0, 6.0]
        ]};

        let actual = matrix.submatrix(0, 2);

        assert_eq!(actual, expected)
    }
}