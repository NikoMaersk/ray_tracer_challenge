use crate::comparison::ApproxEq;
use crate::Matrix4;
use crate::transformation::Transform;

#[derive(Copy, Clone, Debug)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64
}

impl Tuple {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Tuple { x, y, z, w }
    }

    pub fn point(x: f64, y: f64, z: f64) -> Self {
        Tuple {
            x,
            y,
            z,
            w: 1.0
        }
    }

    pub fn vector(x: f64, y: f64, z: f64) -> Self {
        Tuple {
            x,
            y,
            z,
            w: 0.0
        }
    }

    pub fn magnitude(&self) -> f64 {
        f64::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
    }

    pub fn normalize(&self) -> Tuple {
        let mag = self.magnitude();

        if mag != 0.0 {
            *self / mag
        } else {
            *self
        }
    }

    pub fn dot(&self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn w(&self) -> f64 {
        self.w
    }
}

impl Transform for Tuple {
    fn transform(self, transformation: &Matrix4) -> Self {
        *transformation * self
    }
}

impl PartialEq for Tuple {
    fn eq(&self, compare_to: &Self) -> bool {
        self.x.approx_eq(compare_to.x)
            && self.y.approx_eq(compare_to.y)
            && self.z.approx_eq(compare_to.z)
    }
}

impl std::ops::Add for Tuple {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Tuple {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w
        }
    }
}

impl std::ops::Sub for Tuple {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Tuple {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

impl std::ops::Neg for Tuple {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Tuple {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl std::ops::Mul<f64> for Tuple {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Tuple {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w
        }
    }
}

impl std::ops::Mul<Tuple> for f64 {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Self::Output {
        rhs * self
    }
}


impl std::ops::Mul for Tuple {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
            w: self.w
        }
    }
}


impl std::ops::Div<f64> for Tuple {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Tuple {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_point() {
        let point = Tuple::point(1.0, 2.0, 3.0);

        assert_eq!(point.x, 1.0);
        assert_eq!(point.y(), 2.0);
        assert_eq!(point.z(), 3.0);
        assert_eq!(point.w(), 1.0);
    }

    #[test]
    fn test_create_vector() {
        let vector = Tuple::vector(1.0, 2.0, 3.0);

        assert_eq!(vector.x(), 1.0);
        assert_eq!(vector.y(), 2.0);
        assert_eq!(vector.z(), 3.0);
        assert_eq!(vector.w(), 0.0);
    }

    #[test]
    fn test_add_vector_point() {
        let point = Tuple::point(5.5, 2.5, 0.0);
        let vector = Tuple::vector(1.0, 2.0, 3.0);

        let expected = Tuple{
            x: 6.5,
            y: 4.5,
            z: 3.0,
            w: 1.0
        };

        let actual = point + vector;

        assert_eq!(actual, expected);
    }

    #[test]
    fn subtract_point_from_point() {
        let point = Tuple::point(3.0, 2.0, 1.0);
        let vector = Tuple::point(5.0, 6.0, 7.0);

        let expected = Tuple{
            x: -2.0,
            y: -4.0,
            z: -6.0,
            w: 0.0
        };

        let actual = point - vector;
        assert_eq!(actual, expected)
    }

    #[test]
    fn subtract_vector_from_point() {
        let point = Tuple::point(3.0, 2.0, 1.0);
        let vector = Tuple::vector(5.0, 6.0, 7.0);

        let expected = Tuple{
            x: -2.0,
            y: -4.0,
            z: -6.0,
            w: 1.0
        };

        let actual = point - vector;
        assert_eq!(actual, expected)
    }

    #[test]
    fn subtract_vector_from_zero_vector() {
        let zero = Tuple::vector(0.0, 0.0, 0.0);
        let vector = Tuple::vector(1.0, -2.0, 3.0);

        let expected = Tuple {
            x: -1.0,
            y: 2.0,
            z: -3.0,
            w: 0.0
        };

        let actual = zero - vector;

        assert_eq!(actual, expected)
    }

    #[test]
    fn negate_tuple() {
        let point = Tuple::new(1.0, -2.0, 3.0, -4.0);

        let expected = Tuple::new(-1.0, 2.0, -3.0, 4.0);
        let actual = -point;

        assert_eq!(actual, expected)
    }

    #[test]
    fn multiply_with_scalar() {
        let vector = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let scalar: f64 = 3.5;

        let expected = Tuple::new(3.5, -7.0, 10.5, -14.0);
        let actual = vector * scalar;

        assert_eq!(actual, actual)
    }

    #[test]
    fn multiply_with_fraction() {
        let vector = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let scalar: f64 = 0.5;

        let expected = Tuple::new(0.5, -1.0, 1.5, -2.0);
        let actual = vector * scalar;

        assert_eq!(actual, actual)
    }

    #[test]
    fn divide_with_scalar() {
        let vector = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let scalar: f64 = 2.0;

        let expected = Tuple::new(0.5, -1.0, 1.5, -2.0);
        let actual = vector / scalar;

        assert_eq!(actual, expected)
    }

    #[test]
    fn compute_magnitude_y() {
        let vector = Tuple::vector(0.0, 1.0, 0.0);

        let expected: f64 = 1.0;
        let actual = vector.magnitude();

        assert_eq!(actual, expected)
    }

    #[test]
    fn compute_magnitude_x() {
        let vector = Tuple::vector(1.0, 0.0, 0.0);

        let expected: f64 = 1.0;
        let actual = vector.magnitude();

        assert_eq!(actual, expected)
    }

    #[test]
    fn compute_magnitude_z() {
        let vector = Tuple::vector(0.0, 0.0, 1.0);

        let expected: f64 = 1.0;
        let actual = vector.magnitude();

        assert_eq!(actual, expected)
    }

    #[test]
    fn compute_magnitude_positive() {
        let vector = Tuple::vector(1.0, 2.0, 3.0);

        let expected: f64 = f64::sqrt(14.0);
        let actual = vector.magnitude();

        assert_eq!(actual, expected)
    }

    #[test]
    fn compute_magnitude_negative() {
        let vector = Tuple::vector(-1.0, -2.0, -3.0);

        let expected: f64 = f64::sqrt(14.0);
        let actual = vector.magnitude();

        assert_eq!(actual, expected)
    }

    #[test]
    fn normalize_simple_vector() {
        let vector = Tuple::vector(4.0, 0.0, 0.0);

        let expected = Tuple::vector(1.0, 0.0, 0.0);
        let actual = vector.normalize();

        assert_eq!(actual, expected)
    }

    #[test]
    fn normalize() {
        let vector = Tuple::vector(1.0, 2.0, 3.0);

        let expected = Tuple::vector(1.0 / f64::sqrt(14.0), 2.0 / f64::sqrt(14.0), 3.0 / f64::sqrt(14.0));
        let actual = vector.normalize();

        assert_eq!(actual, expected)
    }

    /*
    #[test]
    fn magnitude_of_normalized_vector() {
        let vector = Tuple::vector(1.0, 2.0, 3.0);

        assert_eq!(vector.normalize().magnitude(), 1.0)
    }
    */

    #[test]
    fn dot() {
        let v1 = Tuple::vector(1.0, 2.0, 3.0);
        let v2 = Tuple::vector(2.0, 3.0, 4.0);

        let expected: f64 = 20.0;

        assert_eq!(v1.dot(v2), expected)
    }

    #[test]
    fn cross() {
        let v1 = Tuple::vector(1.0, 2.0, 3.0);
        let v2 = Tuple::vector(2.0, 3.0, 4.0);

        let expected = Tuple::vector(-1.0, 2.0, -1.0);

        assert_eq!(v1 * v2, expected);

        let expected = Tuple::vector(1.0, -2.0, 1.0);

        assert_eq!(v2 * v1, expected)
    }
}
