use crate::Matrix4;

fn translation(x: f32, y: f32, z: f32) -> Matrix4 {
    let mut result = Matrix4::identity_matrix();
    result.matrix[0][3] = x;
    result.matrix[1][3] = y;
    result.matrix[2][3] = z;

    result
}

#[cfg(test)]
mod tests {
    use crate::transformation::translation;
    use crate::Tuple;

    fn mul_by_translation() {
        let transform = translation(5.0, -3.0, 2.0);

        let point = Tuple::point(-3.0, 4.0, 5.0);

        let expected = Tuple::point(2.0, 1.0, 7.0);

        assert_eq!(point * transform, expected)
    }
}