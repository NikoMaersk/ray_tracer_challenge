use crate::comparison::ApproxEq;

#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Color { r, g, b }
    }

    pub fn black() -> Self {
        Color { r: 0.0, g: 0.0, b: 0.0 }
    }

    pub fn white() -> Self {
        Color { r: 1.0, g: 1.0, b: 1.0 }
    }

    pub fn red() -> Self {
        Color { r: 1.0, g: 0.0, b: 0.0 }
    }

    pub fn green() -> Self {
        Color { r: 0.0, g: 1.0, b: 0.0 }
    }

    pub fn blue() -> Self {
        Color { r: 0.0, g: 0.0, b: 1.0 }
    }
}


impl std::ops::Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b
        }
    }
}

impl std::ops::Sub for Color {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b
        }
    }
}


impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        self.r.approx_eq_low_precision(other.r)
            && self.g.approx_eq_low_precision(other.g)
            && self.b.approx_eq_low_precision(other.b)
    }
}

impl std::ops::Mul<f32> for Color {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs
        }
    }
}

impl std::ops::Mul<Color> for f32 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        rhs * self
    }
}

impl  std::ops::Mul<Color> for Color {
    type Output = Self;

    fn mul(self, rhs: Color) -> Self::Output {
        Self {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::color::Color;

    #[test]
    fn add_color() {
        let c1 = Color { r: 0.9, g: 0.6, b: 0.75 };
        let c2 = Color { r: 0.7, g: 0.1, b: 0.25 };

        let expected = Color { r: 1.6, g: 0.7, b: 1.0 };
        assert_eq!(c1 + c2, expected)
    }

    #[test]
    fn subtract_color() {
        let c1 = Color { r: 0.9, g: 0.6, b: 0.75 };
        let c2 = Color { r: 0.7, g: 0.1, b: 0.25 };

        let expected = Color { r: 0.2, g: 0.5, b: 0.5 };
        assert_eq!(c1 - c2, expected)
    }

    #[test]
    fn multiply_scalar() {
        let c1 = Color { r: 0.2, g: 0.3, b: 0.4 };
        let scalar: f32 = 2.0;
        let expected = Color { r: 0.4, g: 0.6, b: 0.8 };
        assert_eq!(c1 * scalar, expected)
    }

    #[test]
    fn hadamard_product() {
        let c1 = Color { r: 1.0, g: 0.2, b: 0.4 };
        let c2 = Color { r: 0.9, g: 1.0, b: 0.1 };

        let expected: Color = Color::new(0.9, 0.2, 0.04);

        assert_eq!(c1 * c2, expected)
    }
}