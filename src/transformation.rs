use crate::Matrix4;

pub fn translation(x: f64, y: f64, z: f64) -> Matrix4 {
    let mut result = Matrix4::identity_matrix();
    result.matrix[0][3] = x;
    result.matrix[1][3] = y;
    result.matrix[2][3] = z;

    result
}


pub fn scaling(x: f64, y: f64, z: f64) -> Matrix4 {
    let mut result = Matrix4::identity_matrix();
    result.matrix[0][0] = x;
    result.matrix[1][1] = y;
    result.matrix[2][2] = z;

    result
}


pub fn rotation_x(radians: f64) -> Matrix4 {
    let mut result = Matrix4::identity_matrix();

    result.matrix[1][1] = f64::cos(radians);
    result.matrix[1][2] = -f64::sin(radians);
    result.matrix[2][1] = f64::sin(radians);
    result.matrix[2][2] = f64::cos(radians);

    result
}


pub fn rotation_y(radian: f64) -> Matrix4 {
    let mut result = Matrix4::identity_matrix();

    result.matrix[0][0] = f64::cos(radian);
    result.matrix[2][0] = -f64::sin(radian);
    result.matrix[0][2] = f64::sin(radian);
    result.matrix[2][2] = f64::cos(radian);

    result
}


pub fn rotation_z(radian: f64) -> Matrix4 {
    let mut result = Matrix4::identity_matrix();

    result.matrix[0][0] = f64::cos(radian);
    result.matrix[1][0] = f64::sin(radian);
    result.matrix[0][1] = -f64::sin(radian);
    result.matrix[1][1] = f64::cos(radian);

    result
}


pub fn shearing(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Matrix4 {
    let mut result = Matrix4::identity_matrix();

    result.matrix[0][1] = xy;
    result.matrix[0][2] = xz;
    result.matrix[1][0] = yx;
    result.matrix[1][2] = yz;
    result.matrix[2][0] = zx;
    result.matrix[2][1] = zy;

    result
}


pub struct TransformationBuilder<T> {
    transformation: Matrix4,
    x: T,
}


impl<T> TransformationBuilder<T> where T: Transform {
    pub fn transform(self) -> T {
        self.x.transform(&self.transformation)
    }

    pub fn translate(mut self, x: f64, y: f64, z: f64) -> Self {
        self.transformation = translation(x, y, z) * self.transformation;
        self
    }

    pub fn scale(mut self, x:f64, y: f64, z: f64) -> Self {
        self.transformation = scaling(x, y, z) * self.transformation;
        self
    }

    pub fn rotate_x(mut self, radian: f64) -> Self {
        self.transformation = rotation_x(radian) * self.transformation;
        self
    }

    pub fn rotate_y(mut self, radian: f64) -> Self {
        self.transformation = rotation_y(radian) * self.transformation;
        self
    }

    pub fn rotate_z(mut self, radian: f64) -> Self {
        self.transformation = rotation_z(radian) * self.transformation;
        self
    }

    pub fn shear(mut self, xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Self {
        self.transformation = shearing(xy, xz, yx, yz, zx, zy) * self.transformation;
        self
    }
}


pub trait Transform {
    fn transform(self, transformation: &Matrix4) -> Self;

    fn translate(self, x: f64, y: f64, z: f64) -> TransformationBuilder<Self> where Self: Sized {
        TransformationBuilder {
            transformation: translation(x, y, z),
            x: self,
        }
    }

    fn scale(self, x: f64, y: f64, z: f64) -> TransformationBuilder<Self> where Self: Sized {
        TransformationBuilder {
            transformation: scaling(x, y, z),
            x: self,
        }
    }

    fn rotate_x(self, radian: f64) -> TransformationBuilder<Self> where Self: Sized {
        TransformationBuilder {
            transformation: rotation_x(radian),
            x: self,
        }
    }

    fn rotate_y(self, radian: f64) -> TransformationBuilder<Self> where Self: Sized {
        TransformationBuilder {
            transformation: rotation_y(radian),
            x: self,
        }
    }

    fn rotate_z(self, radian: f64) -> TransformationBuilder<Self> where Self: Sized {
        TransformationBuilder {
            transformation: rotation_z(radian),
            x: self,
        }
    }

    fn shear(self, xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> TransformationBuilder<Self>
        where Self: Sized,
    {
        TransformationBuilder {
            transformation: shearing(xy, xz, yx, yz, zx, zy),
            x: self,
        }
    }
}



#[cfg(test)]
mod tests {
    use crate::transformation::*;
    use crate::Tuple;

    use std::f64::consts;

    #[test]
    fn mul_by_translation() {
        let transform = translation(5.0, -3.0, 2.0);

        let point = Tuple::point(-3.0, 4.0, 5.0);

        let expected = Tuple::point(2.0, 1.0, 7.0);

        assert_eq!(transform * point, expected)
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

        assert_eq!(transform * vector, expected)
    }

    #[test]
    fn scaling_with_point() {
        let scaling = scaling(2.0, 3.0, 4.0);

        let point = Tuple::point(-4.0, 6.0, 8.0);

        let expected = Tuple::point(-8.0, 18.0, 32.0);

        assert_eq!(scaling * point, expected)
    }

    #[test]
    fn scaling_with_vector() {
        let scaling = scaling(2.0, 3.0, 4.0);

        let vector = Tuple::vector(-4.0, 6.0, 8.0);

        let expected = Tuple::vector(-8.0, 18.0, 32.0);

        assert_eq!(scaling * vector, expected)
    }

    #[test]
    fn scaling_with_inverse() {
        let scaling = scaling(2.0, 3.0, 4.0);
        let inverse = scaling.inverse().unwrap();

        let vector = Tuple::vector(-4.0, 6.0, 8.0);

        let expected = Tuple::vector(-2.0, 2.0, 2.0);

        assert_eq!(inverse * vector, expected)
    }

    #[test]
    fn reflection() {
        let scaling = scaling(-1.0, 1.0, 1.0);

        let point = Tuple::point(2.0, 3.0, 4.0);

        let expected = Tuple::vector(-2.0, 3.0, 4.0);

        assert_eq!(scaling * point, expected)
    }

    #[test]
    fn rotate_point_x() {
        let point = Tuple::point(0.0, 1.0, 0.0);

        let half_quarter = rotation_x(consts::PI / 4.0);
        let full_quarter = rotation_x(consts::PI / 2.0);


        let expected_half = Tuple::point(0.0, f64::sqrt(2.0)/ 2.0, f64::sqrt(2.0)/2.0);
        assert_eq!(half_quarter * point, expected_half);

        let expected_full = Tuple::point(0.0, 0.0, 1.0);
        assert_eq!(full_quarter * point, expected_full)
    }

    #[test]
    fn inverse_x_rotation() {
        let point = Tuple::point(0.0, 1.0, 0.0);

        let half_quarter = rotation_x(consts::PI / 4.0);
        let inverse = half_quarter.inverse().unwrap();

        let expected = Tuple::point(0.0, f64::sqrt(2.0) / 2.0, -f64::sqrt(2.0) / 2.0);

        assert_eq!(inverse * point, expected)
    }


    #[test]
    fn rotate_point_y() {
        let point = Tuple::point(0.0, 0.0, 1.0);

        let half_quarter = rotation_y(consts::PI / 4.0);
        let full_quarter = rotation_y(consts::PI / 2.0);

        let expected_half = Tuple::point(f64::sqrt(2.0) / 2.0, 0.0, f64::sqrt(2.0) / 2.0);
        assert_eq!(half_quarter * point, expected_half);

        let expected_full = Tuple::point(1.0, 0.0, 0.0);
        assert_eq!(full_quarter * point, expected_full)
    }


    #[test]
    fn rotate_point_z() {
        let point = Tuple::point(0.0, 1.0, 0.0);

        let half_quarter = rotation_z(consts::PI / 4.0);
        let full_quarter = rotation_z(consts::PI / 2.0);

        let expected_half = Tuple::point(-f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0, 0.0);
        assert_eq!(half_quarter * point, expected_half);

        let expected_full = Tuple::point(-1.0, 0.0, 0.0);
        assert_eq!(full_quarter * point, expected_full)
    }


    #[test]
    fn shearing_xy() {
        let transform = shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);

        let point = Tuple::point(2.0, 3.0, 4.0);

        let expected = Tuple::point(5.0, 3.0, 4.0);

        assert_eq!(transform * point, expected)
    }


    #[test]
    fn shearing_xz() {
        let transform = shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);

        let point = Tuple::point(2.0, 3.0, 4.0);

        let expected = Tuple::point(6.0, 3.0, 4.0);

        assert_eq!(transform * point, expected)
    }


    #[test]
    fn shearing_yx() {
        let transform = shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);

        let point = Tuple::point(2.0, 3.0, 4.0);

        let expected = Tuple::point(2.0, 5.0, 4.0);

        assert_eq!(transform * point, expected)
    }


    #[test]
    fn shearing_yz() {
        let transform = shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);

        let point = Tuple::point(2.0, 3.0, 4.0);

        let expected = Tuple::point(2.0, 7.0, 4.0);

        assert_eq!(transform * point, expected)
    }


    #[test]
    fn shearing_zx() {
        let transform = shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);

        let point = Tuple::point(2.0, 3.0, 4.0);

        let expected = Tuple::point(2.0, 3.0, 6.0);

        assert_eq!(transform * point, expected)
    }


    #[test]
    fn shearing_zy() {
        let transform = shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);

        let point = Tuple::point(2.0, 3.0, 4.0);

        let expected = Tuple::point(2.0, 3.0, 7.0);

        assert_eq!(transform * point, expected)
    }


    #[test]
    fn chaining_transformations() {
        let point = Tuple::point(1.0, 0.0, 1.0);

        let rotate_x = rotation_x(consts::PI / 2.0);
        let scaling = scaling(5.0, 5.0, 5.0);
        let translation = translation(10.0, 5.0, 7.0);

        let p2 = rotate_x * point;
        let p3 = scaling * p2;
        let p4 = translation * p3;

        assert_eq!(p4, Tuple::point(15.0, 0.0, 7.0))
    }


    #[test]
    fn chained_reverse() {
        let point = Tuple::point(1.0, 0.0, 1.0);

        let rotate_x = rotation_x(consts::PI / 2.0);
        let scaling = scaling(5.0, 5.0, 5.0);
        let translation = translation(10.0, 5.0, 7.0);

        let t = translation * scaling * rotate_x;

        assert_eq!(t * point, Tuple::point(15.0, 0.0, 7.0))
    }

    #[test]
    fn transform_builder() {
        let point = Tuple::point(1.0, 0.0, 1.0);

        let actual = point.rotate_x(consts::PI / 2.0).scale(5.0, 5.0, 5.0).translate(10.0, 5.0, 7.0).transform();

        assert_eq!(actual, Tuple::point(15.0, 0.0, 7.0))
    }
}
