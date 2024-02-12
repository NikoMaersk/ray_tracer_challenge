use crate::Matrix4;

fn translation(x: f32, y: f32, z: f32) -> Matrix4 {
    let mut result = Matrix4::identity_matrix();
    result.matrix[0][3] = x;
    result.matrix[1][3] = y;
    result.matrix[2][3] = z;

    result
}


fn scaling(x: f32, y: f32, z: f32) -> Matrix4 {
    let mut result = Matrix4::identity_matrix();
    result.matrix[0][0] = x;
    result.matrix[1][1] = y;
    result.matrix[2][2] = z;

    result
}


fn rotation_x(radians: f32) -> Matrix4 {
    let mut result = Matrix4::identity_matrix();

    result.matrix[1][1] = f32::cos(radians);
    result.matrix[1][2] = -f32::sin(radians);
    result.matrix[2][1] = f32::sin(radians);
    result.matrix[2][2] = f32::cos(radians);

    result
}


fn rotation_y(radian: f32) -> Matrix4 {
    let mut result = Matrix4::identity_matrix();

    result.matrix[0][0] = f32::cos(radian);
    result.matrix[2][0] = -f32::sin(radian);
    result.matrix[0][2] = f32::sin(radian);
    result.matrix[2][2] = f32::cos(radian);

    result
}


fn rotation_z(radian: f32) -> Matrix4 {
    let mut result = Matrix4::identity_matrix();

    result.matrix[0][0] = f32::cos(radian);
    result.matrix[1][0] = f32::sin(radian);
    result.matrix[0][1] = -f32::sin(radian);
    result.matrix[1][1] = f32::cos(radian);

    result
}


#[cfg(test)]
mod tests {
    use crate::transformation::*;
    use crate::Tuple;

    use std::f32::consts;

    #[test]
    fn mul_by_translation() {
        let transform = translation(5.0, -3.0, 2.0);

        let point = Tuple::point(-3.0, 4.0, 5.0);

        let expected = Tuple::point(2.0, 1.0, 7.0);

        assert_eq!(point * transform, expected)
    }

    #[test]
    fn mul_by_inverse() {
        let transform = translation(5.0, -3.0, 2.0);
        let inverse = transform.inverse().unwrap();

        let point = Tuple::point(-3.0, 4.0, 5.0);

        let expected = Tuple::point(-8.0, 7.0, 3.0);

        assert_eq!(inverse * point, expected)
    }

    #[test]
    fn translation_with_vector() {
        let transform = translation(5.0, -3.0, 2.0);

        let vector = Tuple::vector(-3.0, 4.0, 5.0);

        let expected = Tuple::vector(-3.0, 4.0, 5.0);

        assert_eq!(vector * transform, expected)
    }

    #[test]
    fn scaling_with_point() {
        let scaling = scaling(2.0, 3.0, 4.0);

        let point = Tuple::point(-4.0, 6.0, 8.0);

        let expected = Tuple::point(-8.0, 18.0, 32.0);

        assert_eq!(point * scaling, expected)
    }

    #[test]
    fn scaling_with_vector() {
        let scaling = scaling(2.0, 3.0, 4.0);

        let vector = Tuple::vector(-4.0, 6.0, 8.0);

        let expected = Tuple::vector(-8.0, 18.0, 32.0);

        assert_eq!(vector * scaling, expected)
    }

    #[test]
    fn scaling_with_inverse() {
        let scaling = scaling(2.0, 3.0, 4.0);
        let inverse = scaling.inverse().unwrap();

        let vector = Tuple::vector(-4.0, 6.0, 8.0);

        let expected = Tuple::vector(-2.0, 2.0, 2.0);

        assert_eq!(vector * inverse, expected)
    }

    #[test]
    fn reflection() {
        let scaling = scaling(-1.0, 1.0, 1.0);

        let point = Tuple::point(2.0, 3.0, 4.0);

        let expected = Tuple::vector(-2.0, 3.0, 4.0);

        assert_eq!(point * scaling, expected)
    }

    #[test]
    fn rotate_point_x() {
        let point = Tuple::point(0.0, 1.0, 0.0);

        let half_quarter = rotation_x(consts::PI / 4.0);
        let full_quarter = rotation_x(consts::PI / 2.0);


        let expected_half = Tuple::point(0.0, f32::sqrt(2.0)/ 2.0, f32::sqrt(2.0)/2.0);
        assert_eq!(half_quarter * point, expected_half);

        let expected_full = Tuple::point(0.0, 0.0, 1.0);
        assert_eq!(full_quarter * point, expected_full)
    }

    #[test]
    fn inverse_x_rotation() {
        let point = Tuple::point(0.0, 1.0, 0.0);

        let half_quarter = rotation_x(consts::PI / 4.0);
        let inverse = half_quarter.inverse().unwrap();

        let expected = Tuple::point(0.0, f32::sqrt(2.0) / 2.0, -f32::sqrt(2.0) / 2.0);

        assert_eq!(inverse * point, expected)
    }


    #[test]
    fn rotate_point_y() {
        let point = Tuple::point(0.0, 0.0, 1.0);

        let half_quarter = rotation_y(consts::PI / 4.0);
        let full_quarter = rotation_y(consts::PI / 2.0);

        let expected_half = Tuple::point(f32::sqrt(2.0) / 2.0, 0.0, f32::sqrt(2.0) / 2.0);
        assert_eq!(half_quarter * point, expected_half);

        let expected_full = Tuple::point(1.0, 0.0, 0.0);
        assert_eq!(full_quarter * point, expected_full)
    }


    #[test]
    fn rotate_point_z() {
        let point = Tuple::point(0.0, 1.0, 0.0);

        let half_quarter = rotation_z(consts::PI / 4.0);
        let full_quarter = rotation_z(consts::PI / 2.0);

        let expected_half = Tuple::point(-f32::sqrt(2.0) / 2.0, f32::sqrt(2.0) / 2.0, 0.0);
        assert_eq!(half_quarter * point, expected_half);

        let expected_full = Tuple::point(-1.0, 0.0, 0.0);
        assert_eq!(full_quarter * point, expected_full)
    }
}