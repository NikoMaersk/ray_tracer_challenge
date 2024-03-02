use crate::intersection::Intersection;
use crate::ray::Ray;
use crate::{Matrix4, Transform, Tuple};
use crate::shapes::shape_enum::Shape;


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Sphere {
    pub transform: Matrix4,
}

impl Sphere {
    pub fn new() -> Self {
        Sphere {
            transform: Matrix4::identity_matrix()
        }
    }

    pub fn with_transform(mut self, transform: Matrix4) -> Self {
        self.transform = transform;
        self
    }

    pub fn intersect(&self, ray: Ray) -> Vec<Intersection> {
        let transformed_ray = match self.transform.inverse() {
            Some(inverse) => { ray.transform(&inverse) }
            None => { return vec![] }
        };
        let sphere_to_ray = transformed_ray.origin - Tuple::point(0.0, 0.0, 0.0);

        let a = transformed_ray.direction.dot(transformed_ray.direction);
        let b = 2.0 * transformed_ray.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;

        let discriminant = (b * b) - 4.0 * a * c;

        if discriminant < 0.0 {
            return vec![];
        } else {
            let t1 = Intersection::new((-b - discriminant.sqrt()) / (2.0 * a), Shape::Sphere(self));
            let t2 = Intersection::new((-b + discriminant.sqrt()) / (2.0 * a), Shape::Sphere(self));

            vec![t1, t2]
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ray::Ray;
    use crate::shapes::sphere::Sphere;
    use crate::{Matrix4, Tuple, scaling, translation};

    #[test]
    fn ray_intersects_sphere_at_two_points() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();

        let xs = s.intersect(r);

        assert_eq!(2, xs.len());
        assert_eq!(4.0, xs[0].t);
        assert_eq!(6.0, xs[1].t)
    }

    #[test]
    fn ray_intersects_a_sphere_at_tangent() {
        let r = Ray::new(Tuple::point(0.0, 1.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();

        let xs = s.intersect(r);

        assert_eq!(2, xs.len());
        assert_eq!(5.0, xs[0].t);
        assert_eq!(5.0, xs[1].t);
    }

    #[test]
    fn ray_misses_sphere() {
        let r = Ray::new(Tuple::point(0.0, 2.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();

        let xs = s.intersect(r);

        assert_eq!(0, xs.len())
    }

    #[test]
    fn ray_originates_inside_a_sphere() {
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();

        let xs = s.intersect(r);

        assert_eq!(2, xs.len());
        assert_eq!(-1.0, xs[0].t);
        assert_eq!(1.0, xs[1].t);
    }

    #[test]
    fn sphere_is_behind_a_ray() {
        let r = Ray::new(Tuple::point(0.0, 0.0, 5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();

        let xs = s.intersect(r);

        assert_eq!(2, xs.len());
        assert_eq!(-6.0, xs[0].t);
        assert_eq!(-4.0, xs[1].t);
    }

    #[test]
    fn sphere_default_transformation() {
        let s = Sphere::new();

        assert_eq!(s.transform, Matrix4::identity_matrix())
    }

    #[test]
    fn change_sphere_transformation() {
        let t = translation(2.0, 3.0, 4.0);
        let mut s = Sphere::new().with_transform(t);

        assert_eq!(s.transform, t)
    }

    #[test]
    fn intersecting_a_scaled_sphere_with_a_ray() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let mut s = Sphere::new().with_transform(scaling(2.0, 2.0, 2.0));

        let xs = s.intersect(r);

        assert_eq!(2, xs.len());
        assert_eq!(3.0, xs[0].t);
        assert_eq!(7.0, xs[1].t);
    }

    #[test]
    fn intersecting_a_translated_sphere_with_a_ray() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let mut s = Sphere::new().with_transform(translation(5.0, 0.0, 0.0));

        let xs = s.intersect(r);

        assert_eq!(0, xs.len());
    }
}
