
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
}

impl PartialEq for Matrix4 {
    fn eq(&self, other: &Self) -> bool {
        for (row, row_other) in self.matrix.iter().zip(other.matrix.iter()) {
            for (element_self, element_other) in row.iter().zip(row_other.iter()) {
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
            for j in 0..result.matrix.len() {
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
}

#[cfg(test)]
mod tests {
    use crate::matrices::Matrix4;

    #[test]
    fn create_4x4() {
        let matrix4 = Matrix4::new();
        let mut matrix = matrix4.matrix;

        matrix[0][0] = 1.5;

        let expected = 1.5;

        assert_eq!(matrix[0][0], expected)
    }

    #[test]
    fn matrix_equality() {
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
}