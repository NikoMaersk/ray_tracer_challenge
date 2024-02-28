use crate::shapes::sphere::Sphere;
use crate::Tuple;

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    origin: Tuple,
    direction: Tuple
}

impl Ray {

    fn new(o: Tuple, d: Tuple) -> Self {
        Ray { origin: o, direction: d }
    }

    fn position(&self, t: f32) -> Tuple {
        self.origin + self.direction * t
    }

    fn intersection(&self, s: f32) -> (f32, f32) {
        let sphere_to_ray = self.origin - Tuple::point(0.0, 0.0, 0.0);

        let a = self.direction.dot(self.direction);
        let b = 2.0 * self.direction.dot(sphere_to_ray);
        let c = (sphere_to_ray.dot(sphere_to_ray)) - 1.0;

        let discriminant: f32 = (b*b) - 4.0 * a * c;

        if discriminant < 0.0 {
            return (0.0, 0.0);
        }

        let t1: f32 = -b - f32::sqrt(discriminant) / (2.0 * a);
        let t2: f32 = -b + f32::sqrt(discriminant) / (2.0 * a);


        (t1, t2)
    }
}

#[cfg(test)]
mod tests {
    use crate::ray::Ray;
    use crate::shapes::sphere::Sphere;
    use crate::Tuple;

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
    fn two_point_intersection() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));

        let s = Sphere::new();


        let (a, b) = r.intersection(0.0);

    }

    #[test]
    fn sphere_to_ray() {
        let origin = Tuple::point(1.0, 2.0, 3.0);
        let direction = Tuple::vector(4.0, 5.0, 6.0);

        let r = Ray { origin, direction };


    }
}