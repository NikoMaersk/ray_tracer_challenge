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
}

#[cfg(test)]
mod tests {
    use crate::ray::Ray;
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
}