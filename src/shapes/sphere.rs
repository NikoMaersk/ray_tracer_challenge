use crate::intersection::Intersection;
use crate::ray::Ray;
use crate::{Matrix4, Transform, Tuple};
use crate::materials::Material;
use crate::shapes::shape_enum::{RayInteractable, Shape};


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Sphere {
    pub transform: Matrix4,
    pub material: Material,
}

impl Sphere {
    pub fn new() -> Self {
        Sphere {
            transform: Matrix4::identity_matrix(),
            material: Material::new()
        }
    }

    pub fn with_transform(mut self, transform: Matrix4) -> Self {
        self.transform = transform;
        self
    }

    pub fn with_material(mut self, material: Material) -> Self {
        self.material = material;
        self
    }
}

impl RayInteractable for Sphere {

    // @FIXME Possible refactor to return Intersections instead of a Vec<Intersection>
    fn intersect(&self, ray: Ray) -> Vec<Intersection> {
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
            let t1 = Intersection::new((-b - discriminant.sqrt()) / (2.0 * a), Shape::Sphere(*self));
            let t2 = Intersection::new((-b + discriminant.sqrt()) / (2.0 * a), Shape::Sphere(*self));

            vec![t1, t2]
        }
    }

    fn normal_at(&self, point: Tuple) -> Tuple {
        let inverse_transform = match self.transform.inverse() {
            Some(matrix) => matrix,
            None => return point
        };

        let object_point = inverse_transform * point;
        let object_normal = (object_point - Tuple::point(0.0, 0.0, 0.0)).normalize();
        let mut world_normal = inverse_transform.transpose() * object_normal;
        world_normal.w = 0.0;

        world_normal.normalize()
    }

    fn material(&self) -> Material {
        self.material
    }
}

impl Transform for Sphere {
    fn transform(self, transformation: &Matrix4) -> Self {
        let new_transform = *transformation * self.transform;
        Sphere {
            transform: new_transform,
            material: self.material
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ray::Ray;
    use crate::shapes::sphere::Sphere;
    use crate::{Matrix4, Tuple, scaling, translation, rotation_z, Transform, Material};
    use crate::shapes::shape_enum::RayInteractable;

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
        let s = Sphere::new().with_transform(t);

        assert_eq!(s.transform, t)
    }

    #[test]
    fn intersecting_a_scaled_sphere_with_a_ray() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new().with_transform(scaling(2.0, 2.0, 2.0));

        let xs = s.intersect(r);

        assert_eq!(2, xs.len());
        assert_eq!(3.0, xs[0].t);
        assert_eq!(7.0, xs[1].t);
    }

    #[test]
    fn intersecting_a_translated_sphere_with_a_ray() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new().with_transform(translation(5.0, 0.0, 0.0));

        let xs = s.intersect(r);

        assert_eq!(0, xs.len());
    }

    #[test]
    fn normal_on_a_sphere_on_x_axis() {
        let s = Sphere::new();

        let n = s.normal_at(Tuple::point(1.0, 0.0, 0.0));

        assert_eq!(Tuple::vector(1.0, 0.0, 0.0), n);
    }

    #[test]
    fn normal_on_a_sphere_on_y_axis() {
        let s = Sphere::new();

        let n = s.normal_at(Tuple::point(0.0, 1.0, 0.0));

        assert_eq!(Tuple::vector(0.0, 1.0, 0.0), n);
    }

    #[test]
    fn normal_on_a_sphere_on_z_axis() {
        let s = Sphere::new();

        let n = s.normal_at(Tuple::point(0.0, 0.0, 1.0));

        assert_eq!(Tuple::vector(0.0, 0.0, 1.0), n);
    }

    #[test]
    fn normal_on_a_sphere_at_nonaxial_point() {
        let s = Sphere::new();

        let point = f64::sqrt(3.0)/3.0;
        let n = s.normal_at(Tuple::point(point, point, point));

        assert_eq!(Tuple::vector(point, point, point), n);
    }

    #[test]
    fn the_normal_is_a_normalized_vector() {
        let s = Sphere::new();

        let point = f64::sqrt(3.0)/3.0;
        let n = s.normal_at(Tuple::point(point, point, point));

        assert_eq!(Tuple::vector(point, point, point), n);
    }

    #[test]
    fn computing_the_normal_on_translated_sphere() {
        let s = Sphere::new().with_transform(translation(0.0, 1.0, 0.0));

        let n = s.normal_at(Tuple::point(0.0, 1.70711, -0.70711));

        assert_eq!(Tuple::vector(0.0, 0.70711, -0.70711), n);
    }

    #[test]
    fn computing_the_normal_on_transformed_sphere() {
        let s = Sphere::new().with_transform(scaling(1.0, 0.5, 1.0) * rotation_z(std::f64::consts::PI/5.0));

        let sqrt_two = f64::sqrt(2.0) / 2.0;

        let n = s.normal_at(Tuple::point(0.0, sqrt_two, -sqrt_two));

        assert_eq!(Tuple::vector(0.0, 0.97014, -0.24254), n);
    }

    #[test]
    fn computing_the_normal_on_transformed_sphere_with_builder() {
        let s = Sphere::new()
            .rotate_z(std::f64::consts::PI/5.0)
            .scale(1.0, 0.5, 1.0)
            .transform();

        let sqrt_two = f64::sqrt(2.0) / 2.0;

        let n = s.normal_at(Tuple::point(0.0, sqrt_two, -sqrt_two));

        assert_eq!(Tuple::vector(0.0, 0.97014, -0.24254), n);
    }

    #[test]
    fn sphere_has_default_material() {
        let s = Sphere::new();

        assert_eq!(Material::new(), s.material)
    }

    #[test]
    fn sphere_may_be_assigned_material() {
        let mut s = Sphere::new();

        let mut m = Material::new();
        m.ambient = 1.0;

        s.material = m;

        assert_eq!(m, s.material)
    }
}
