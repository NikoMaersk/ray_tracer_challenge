use crate::{Color, Tuple};

#[derive(Copy, Clone, Debug)]
pub struct Light {
    pub position: Tuple,
    pub intensity: Color,
}

impl Light {
    pub fn new(position: Tuple, intensity: Color) -> Self {
        Light {
            position,
            intensity,
        }
    }
}

impl Default for Light {
    fn default() -> Self {
        Light {
            intensity: Color::white(),
            position: Tuple::point(0.0, 0.0, 0.0)
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::{Color, Tuple};
    use crate::lights::Light;

    #[test]
    fn light_has_position_and_intensity() {
        let intensity = Color::new(1.0, 1.0, 1.0);
        let position = Tuple::point(0.0, 0.0, 0.0);

        let light = Light::new(position, intensity);

        assert_eq!(light.position, position);
        assert_eq!(light.intensity, intensity)
    }


}