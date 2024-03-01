use crate::{Matrix4, Transform, Tuple};

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple
}

impl Ray {

    pub fn new(o: Tuple, d: Tuple) -> Self {
        Ray { origin: o, direction: d }
    }

    pub fn position(&self, t: f32) -> Tuple {
        self.origin + self.direction * t
    }

    pub fn transformation(&self, transform: &Matrix4) -> Self {
        Ray {
            origin: self.origin * *transform,
            direction: self.direction * *transform
        }
    }
}

impl Transform for Ray {
    fn transform(self, transformation: &Matrix4) -> Self {
        self.transformation(transformation)
    }
}

#[cfg(test)]
mod tests {
    use crate::ray::Ray;
    use crate::{Transform, transformation, Tuple};

    #[test]
    fn create_queue_ray() {
        let origin = Tuple::point(1.0, 2.0, 3.0);
        let direction = Tuple::vector(4.0, 5.0, 6.0);

        let r = Ray { origin, direction };

        assert_eq!(r.origin, origin);
        assert_eq!(r.direction, direction)
    }

    #[test]
    fn point_from_distance() {
        let r = Ray::new(Tuple::point(2.0, 3.0, 4.0), Tuple::vector(1.0, 0.0, 0.0));

        assert_eq!(r.position(0.0), Tuple::point(2.0, 3.0, 4.0));
        assert_eq!(r.position(1.0), Tuple::point(3.0, 3.0, 4.0));
        assert_eq!(r.position(-1.0), Tuple::point(1.0, 3.0, 4.0));
        assert_eq!(r.position(2.5), Tuple::point(4.5, 3.0, 4.0))
    }

    #[test]
    fn translating_a_ray() {
        let r = Ray::new(Tuple::point(1.0, 2.0, 3.0), Tuple::vector(0.0, 1.0, 0.0));

        let m = transformation::translation(3.0, 4.0, 5.0);

        let r2 = r.transformation(&m);

        let expected_origin = Tuple::point(4.0, 6.0, 8.0);
        let expected_direction = Tuple::vector(0.0, 1.0, 0.0);

        assert_eq!(expected_origin, r2.origin);
        assert_eq!(expected_direction, r2.direction)
    }

    #[test]
    fn scaling_a_ray() {
        let r = Ray::new(Tuple::point(1.0, 2.0, 3.0), Tuple::vector(0.0, 1.0, 0.0));

        let m = transformation::scaling(2.0, 3.0, 4.0);

        let r2 = r.transformation(&m);

        let expected_origin = Tuple::point(2.0, 6.0, 12.0);
        let expected_direction = Tuple::vector(0.0, 3.0, 0.0);

        assert_eq!(expected_origin, r2.origin);
        assert_eq!(expected_direction, r2.direction)
    }

    #[test]
    fn fluent_translating_a_ray() {
        let r = Ray::new(Tuple::point(1.0, 2.0, 3.0), Tuple::vector(0.0, 1.0, 0.0));

        let r2 = r.translate(3.0, 4.0, 5.0).transform();

        let expected_origin = Tuple::point(4.0, 6.0, 8.0);
        let expected_direction = Tuple::vector(0.0, 1.0, 0.0);

        assert_eq!(expected_origin, r2.origin);
        assert_eq!(expected_direction, r2.direction)
    }

    #[test]
    fn fluent_scaling_a_ray() {
        let r = Ray::new(Tuple::point(1.0, 2.0, 3.0), Tuple::vector(0.0, 1.0, 0.0));

        let r2 = r.scale(2.0, 3.0, 4.0).transform();

        let expected_origin = Tuple::point(2.0, 6.0, 12.0);
        let expected_direction = Tuple::vector(0.0, 3.0, 0.0);

        assert_eq!(expected_origin, r2.origin);
        assert_eq!(expected_direction, r2.direction)
    }
}