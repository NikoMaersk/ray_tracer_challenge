use crate::{Color, Tuple};
use crate::lights::Light;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64
}

impl Material {
    pub fn new() -> Self {
        Material::default()
    }

    pub fn lighting(&self, light: Light, point: Tuple, eye_v: Tuple, normal_v: Tuple) -> Color {
        let diffuse;
        let specular;
        let ambient;

        // Combine the surface color with the light's color/intensity
        let effective_color = self.color * light.intensity;

        // Find the direction to the light source
        let light_v = (light.position - point).normalize();

        // Compute the ambient contribution
        ambient = effective_color * self.ambient;

        // light_dot_normal represent the cosine of the angle between the
        // light vector and the normal vector. A negative number means the
        // light is on the other side of the surface.
        let light_dot_normal = light_v.dot(normal_v);

        if light_dot_normal < 0.0 {
            diffuse = Color::black();
            specular = Color::black();
        } else {
            // Compute the diffuse contribution
            diffuse = effective_color * self.diffuse * light_dot_normal;

            // reflect_dot_eye represent the cosine of the angle between the
            // reflection vector and the eye vector. A negative number means the
            // light reflects away from the eye.
            let reflect_v = -light_v.reflect(normal_v);
            let reflect_dot_eye = reflect_v.dot(eye_v);

            if reflect_dot_eye <= 0.0 {
                specular = Color::black();
            } else {
                // Compute the specular contribution
                let factor = reflect_dot_eye.powf(self.shininess);
                specular = light.intensity * self.specular * factor;
            }
        }

        ambient + diffuse + specular
    }
}

impl Default for Material {
    fn default() -> Self {
        Material {
            color: Color::white(),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Color, Tuple};
    use crate::lights::Light;
    use crate::Material;

    #[test]
    fn default_material() {
        let m = Material::default();

        assert_eq!(m.color, Color::new(1.0, 1.0, 1.0));
        assert_eq!(m.ambient, 0.1);
        assert_eq!(m.diffuse, 0.9);
        assert_eq!(m.specular, 0.9);
        assert_eq!(m.shininess, 200.0);
    }

    #[test]
    fn lighting_with_eye_between_light_and_the_surface() {
        let m = Material::default();
        let position = Tuple::point(0.0, 0.0, 0.0);

        let eye_v = Tuple::vector(0.0, 0.0, -1.0);
        let normal_v = Tuple::vector(0.0, 0.0, -1.0);
        let light = Light::new(Tuple::point(0.0, 0.0, -10.0), Color::white());

        let result = m.lighting(light, position, eye_v, normal_v);

        assert_eq!(result, Color::new(1.9, 1.9, 1.9))
    }

    #[test]
    fn lighting_with_eye_between_light_and_the_surface_offset_45_degrees() {
        let m = Material::default();
        let position = Tuple::point(0.0, 0.0, 0.0);
        let sqrt_two = f64::sqrt(2.0) / 2.0;

        let eye_v = Tuple::vector(0.0, sqrt_two, sqrt_two);
        let normal_v = Tuple::vector(0.0, 0.0, -1.0);
        let light = Light::new(Tuple::point(0.0, 0.0, -10.0), Color::white());

        let result = m.lighting(light, position, eye_v, normal_v);

        assert_eq!(result, Color::new(1.0, 1.0, 1.0))
    }

    #[test]
    fn lighting_with_eye_opposite_surface_light_offset_45_degrees() {
        let m = Material::default();
        let position = Tuple::point(0.0, 0.0, 0.0);

        let eye_v = Tuple::vector(0.0, 0.0, -1.0);
        let normal_v = Tuple::vector(0.0, 0.0, -1.0);
        let light = Light::new(Tuple::point(0.0, 10.0, -10.0), Color::white());

        let result = m.lighting(light, position, eye_v, normal_v);

        assert_eq!(result, Color::new(0.7364, 0.7364, 0.7364))
    }

    #[test]
    fn lighting_with_eye_in_the_path_of_reflection_vector() {
        let m = Material::default();
        let position = Tuple::point(0.0, 0.0, 0.0);
        let sqrt_two = f64::sqrt(2.0) / 2.0;

        let eye_v = Tuple::vector(0.0, -sqrt_two, -sqrt_two);
        let normal_v = Tuple::vector(0.0, 0.0, -1.0);
        let light = Light::new(Tuple::point(0.0, 10.0, -10.0), Color::white());

        let result = m.lighting(light, position, eye_v, normal_v);

        assert_eq!(result, Color::new(1.6364, 1.6364, 1.6364))
    }

    #[test]
    fn lightning_with_the_light_behind_the_surface() {
        let m = Material::default();
        let position = Tuple::point(0.0, 0.0, 0.0);

        let eye_v = Tuple::vector(0.0, 0.0, -1.0);
        let normal_v = Tuple::vector(0.0, 0.0, -1.0);
        let light = Light::new(Tuple::point(0.0, 0.0, 10.0), Color::white());

        let result = m.lighting(light, position, eye_v, normal_v);

        assert_eq!(result, Color::new(0.1, 0.1, 0.1))
    }
}