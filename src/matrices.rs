use crate::comparison::ApproxEq;
use crate::Tuple;


#[derive(Copy, Clone, Debug)]
pub struct Matrix4 {
    pub matrix: [[f64; 4]; 4],
}

impl Matrix4 {

    pub fn new() -> Self {
        Matrix4 {
            matrix: [[0.0; 4]; 4],
        }
    }


    pub fn identity_matrix() -> Self {
        Matrix4 {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0]
            ]
        }
    }


    pub fn transpose(&self) -> Self {
        let mut result = Matrix4::new();

        for i in 0..self.matrix.len() {
            for j in 0..self.matrix[0].len() {
                result.matrix[i][j] = self.matrix[j][i]
            }
        }

        result
    }


    pub fn transpose_mut(&mut self) {
        for i in 0..self.matrix.len() {
            for j in i + 1..self.matrix[0].len() {
                (self.matrix[i][j], self.matrix[j][i]) = (self.matrix[j][i], self.matrix[i][j])
            }
        }
    }


    pub fn submatrix(&self, row_to_remove: usize, col_to_remove: usize) -> Matrix3 {
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


    pub fn minor(&self, row: usize, col: usize) -> f64 {
        self.submatrix(row, col).determinant()
    }


    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        let minor = self.minor(row, col);
        if (row + col) % 2 == 0 {
            minor
        } else {
            -minor
        }
    }


    pub fn determinant(&self) -> f64 {
        let mut result = 0.0;
        for i in 0..4 {
            result += self.matrix[0][i] * self.cofactor(0, i)
        }

        result
    }


    pub fn inverse(&self) -> Option<Self> {

        let determinant = self.determinant();

        if determinant.approx_eq(0.0) {
            return None
        }

        let mut result = Self::new();

        for row in 0..self.matrix.len() {
            for col in 0..self.matrix[0].len() {
                let c = self.cofactor(row, col);

                result.matrix[col][row] = c / determinant;
            }
        }

        Some(result)
    }
}

impl PartialEq for Matrix4 {
    fn eq(&self, other: &Self) -> bool {
        for (row_self, row_other) in self.matrix.iter().zip(other.matrix.iter()) {
            for (element_self, element_other) in row_self.iter().zip(row_other.iter()) {
                if !element_self.approx_eq_low_precision(*element_other) {
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
}


impl std::ops::Mul<Matrix4> for Tuple {
    type Output = Tuple;

    fn mul(self, rhs: Matrix4) -> Self::Output {
        Tuple::new(
            rhs.matrix[0][0] * self.x() +
                rhs.matrix[0][1] * self.y() +
                rhs.matrix[0][2] * self.z() +
                rhs.matrix[0][3] * self.w(),
            rhs.matrix[1][0] * self.x() +
                rhs.matrix[1][1] * self.y() +
                rhs.matrix[1][2] * self.z() +
                rhs.matrix[1][3] * self.w(),
            rhs.matrix[2][0] * self.x() +
                rhs.matrix[2][1] * self.y() +
                rhs.matrix[2][2] * self.z() +
                rhs.matrix[2][3] * self.w(),
            rhs.matrix[3][0] * self.x() +
                rhs.matrix[3][1] * self.y() +
                rhs.matrix[3][2] * self.z() +
                rhs.matrix[3][3] * self.w()
        )
    }
}


impl std::ops::Mul<Tuple> for Matrix4 {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Self::Output {
        Tuple::new(
            self.matrix[0][0] * rhs.x() +
                self.matrix[0][1] * rhs.y() +
                self.matrix[0][2] * rhs.z() +
                self.matrix[0][3] * rhs.w(),
            self.matrix[1][0] * rhs.x() +
                self.matrix[1][1] * rhs.y() +
                self.matrix[1][2] * rhs.z() +
                self.matrix[1][3] * rhs.w(),
            self.matrix[2][0] * rhs.x() +
                self.matrix[2][1] * rhs.y() +
                self.matrix[2][2] * rhs.z() +
                self.matrix[2][3] * rhs.w(),
            self.matrix[3][0] * rhs.x() +
                self.matrix[3][1] * rhs.y() +
                self.matrix[3][2] * rhs.z() +
                self.matrix[3][3] * rhs.w()
        )
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Matrix3 {
    matrix: [[f64; 3]; 3]
}

impl Matrix3 {
    fn new() -> Self {
        Matrix3 {
            matrix: [[0.0; 3]; 3]
        }
    }


    pub fn submatrix(&self, row_to_remove: usize, col_to_remove: usize) -> Matrix2 {
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


    pub fn minor(&self, row: usize, col: usize) -> f64 {
        self.submatrix(row, col).determinant()
    }


    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        let minor = self.minor(row, col);
        if (row + col) % 2 == 0 {
            minor
        } else {
            -minor
        }
    }


    pub fn determinant(&self) -> f64 {
        let mut result = 0.0;
        for i in 0..3 {
            result += self.matrix[0][i] * self.cofactor(0, i);
        }

        result
    }
}


#[derive(Copy, Clone, Debug)]
pub struct Matrix2 {
    matrix: [[f64; 2]; 2]
}

impl Matrix2 {
    pub fn new() -> Self {
        Matrix2 {
            matrix: [[0.0; 2]; 2]
        }
    }

    pub fn determinant(&self) -> f64 {
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

    #[test]
    fn minor_of_matrix3() {
        let matrix = Matrix3 { matrix: [
            [3.0, 5.0, 0.0],
            [2.0, -1.0, -7.0],
            [6.0, -1.0, 5.0]
        ]};

        let minor= matrix.minor(1, 0);

        let determinant = 25.0;

        assert_eq!(determinant, minor)
    }

    #[test]
    fn cofactor_no_negation() {
        let matrix = Matrix3 { matrix: [
            [3.0, 5.0, 0.0],
            [2.0, -1.0, -7.0],
            [6.0, -1.0, 5.0]
        ]};

        let actual_cofactor = matrix.cofactor(0, 0);

        let expected_cofactor = -12.0;

        assert_eq!(actual_cofactor, expected_cofactor)
    }

    #[test]
    fn cofactor_negation() {
        let matrix = Matrix3 { matrix: [
            [3.0, 5.0, 0.0],
            [2.0, -1.0, -7.0],
            [6.0, -1.0, 5.0]
        ]};

        let actual_cofactor = matrix.cofactor(1, 0);

        let expected_cofactor = -25.0;

        assert_eq!(actual_cofactor, expected_cofactor)
    }

    #[test]
    fn determinant_minor3() {
        let matrix = Matrix3 { matrix: [
            [1.0, 2.0, 6.0],
            [-5.0, 8.0, -4.0],
            [2.0, 6.0, 4.0]
        ]};

        let actual_determinant = matrix.determinant();
        let expected_determinant = -196.0;

        assert_eq!(actual_determinant, expected_determinant)
    }

    #[test]
    fn determinant_minor4() {
        let matrix = Matrix4 { matrix: [
            [-2.0, -8.0, 3.0, 5.0],
            [-3.0, 1.0, 7.0, 3.0],
            [1.0, 2.0, -9.0, 6.0],
            [-6.0, 7.0, 7.0, -9.0]
        ]};

        let actual_determinant = matrix.determinant();
        let expected_determinant = -4071.0;

        assert_eq!(actual_determinant, expected_determinant)
    }


    #[test]
    fn convertible_matrix4() {
        let matrix = Matrix4 { matrix: [
            [6.0, 4.0, 4.0, 4.0],
            [5.0, 5.0, 7.0, 6.0],
            [4.0, -9.0, 3.0, -7.0],
            [9.0, 1.0, 7.0, -6.0]
        ]};

        let actual_determinant = matrix.determinant();

        assert_ne!(actual_determinant, 0.0)
    }


    #[test]
    fn non_convertible_matrix4() {
        let matrix = Matrix4 { matrix: [
            [-4.0, 2.0, -2.0, -3.0],
            [9.0, 6.0, 2.0, 6.0],
            [0.0, -5.0, 1.0, -5.0],
            [0.0, 0.0, 0.0, 0.0]
        ]};

        let actual_determinant = matrix.determinant();

        assert_eq!(actual_determinant, 0.0)
    }


    #[test]
    fn inversion_matrix4_test1() {
        let matrix = Matrix4 { matrix: [
            [8.0, -5.0, 9.0, 2.0],
            [7.0, 5.0, 6.0, 1.0],
            [-6.0, 0.0, 9.0, 6.0],
            [-3.0, 0.0, -9.0, -4.0],
        ]};

        let expected_inverse = Matrix4 {
            matrix: [
                [-0.15385, -0.15385, -0.28205, -0.53846],
                [-0.07692, 0.12308, 0.02564, 0.03077],
                [0.35897, 0.35897, 0.43590, 0.92308],
                [-0.69231, -0.69231, -0.76923, -1.92308],
            ]};

        let actual_inverse = match matrix.inverse() {
            Some(val) => val,
            None => panic!("matrix is not invertible")
        };

        assert_eq!(actual_inverse, expected_inverse)
    }


    #[test]
    fn inversion_matrix4_test2() {
        let matrix = Matrix4 { matrix: [
                [-5.0, 2.0, 6.0, -8.0],
                [1.0, -5.0, 1.0, 8.0],
                [7.0, 7.0, -6.0, -7.0],
                [1.0, -3.0, 7.0, 4.0],
            ]};

        let expected_inverse = Matrix4 {
            matrix: [
                [0.21805, 0.45113, 0.24060, -0.04511],
                [-0.80827, -1.45677, -0.44361, 0.52068],
                [-0.07895, -0.22368, -0.05263, 0.19737],
                [-0.52256, -0.81391, -0.30075, 0.30639],
            ]};

        let actual_inverse = match matrix.inverse() {
            Some(val) => val,
            None => panic!("matrix is not invertible")
        };

        assert_eq!(actual_inverse, expected_inverse)
    }


    #[test]
    fn inversion_matrix4_test3() {
        let matrix = Matrix4 {
            matrix: [
                [9.0, 3.0, 0.0, 9.0],
                [-5.0, -2.0, -6.0, -3.0],
                [-4.0, 9.0, 6.0, 4.0],
                [-7.0, 6.0, 6.0, 2.0],
            ]};

        let expected_inverse = Matrix4 {
            matrix: [
                [-0.04074, -0.07778, 0.14444, -0.22222],
                [-0.07778, 0.03333, 0.36667, -0.33333],
                [-0.02901, -0.14630, -0.10926, 0.12963],
                [0.17778, 0.06667, -0.26667, 0.33333],
            ]};

        let actual_inverse = match matrix.inverse() {
            Some(val) => val,
            None => panic!("matrix is not invertible")
        };

        assert_eq!(actual_inverse, expected_inverse)
    }


    #[test]
    fn multiply_product_inverse() {
        let matrix_a = Matrix4 {
            matrix: [
                [3.0, -9.0, 7.0, 3.0],
                [3.0, -8.0, 2.0, -9.0],
                [-4.0, 4.0, 4.0, 1.0],
                [-6.0, 5.0, -1.0, 1.0],
            ],
        };

        let matrix_b = Matrix4 {
            matrix: [
                [8.0, 2.0, 2.0, 2.0],
                [3.0, -1.0, 7.0, 0.0],
                [7.0, 0.0, 5.0, 4.0],
                [6.0, -2.0, 0.0, 5.0],
            ],
        };

        let matrix_c = matrix_a * matrix_b;

        let reverse = matrix_c * matrix_b.inverse().unwrap();

        assert_eq!(matrix_a, reverse)
    }
}
