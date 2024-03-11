use crate::{Color, Light, Material, Ray, transformation, Tuple};
use crate::intersection::Intersections;
use crate::shapes::{Shape, Sphere};
use crate::shapes::shape_enum::RayInteractable;

pub struct World {
    pub objects: Vec<Shape>,
    pub lights: Vec<Light>,
}

impl World {
    pub fn new(objects: Vec<Shape>, lights: Vec<Light>) -> Self {
        World {
            objects,
            lights,
        }
    }

    pub fn create_default_world() -> Self {
        let light = Light::new(Tuple::point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let material = Material {
            color: Color::new(0.8, 1.0, 0.6),
            ambient: 0.9,
            diffuse: 0.7,
            specular: 0.2,
            shininess: 200.0
        };
        let s1 = Sphere::new().with_material(material);
        let s2 = Sphere::new().with_transform(transformation::scaling(0.5, 0.5, 0.5));

        World::new(vec![Shape::Sphere(s1), Shape::Sphere(s2)], vec![light])
    }

    pub fn intersect(&self, ray: Ray) -> Intersections {
        let mut res = Intersections::new();

        for shape in &self.objects {
            let intersections = shape.intersect(ray);
            res.push_vec(intersections);
        }

        res
    }
}

impl Default for World {
    fn default() -> Self {
        World {
            objects: vec![],
            lights: vec![],
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::{Color, Light, Ray, scaling, Tuple};
    use crate::shapes::{Sphere};
    use crate::world::World;

    #[test]
    fn creating_a_world() {
        let w = World::default();

        assert_eq!(0, w.objects.len());
        assert_eq!(0, w.lights.len())
    }

    #[test]
    fn creating_default_world() {
        let light = Light::new(
            Tuple::point(-10.0, 10.0, -10.0),
            Color::white());

        let mut s1 = Sphere::new();
        s1.material.color = Color::new(0.8, 1.0, 0.6);
        s1.material.diffuse = 0.7;
        s1.material.specular = 0.2;

        let s2 = Sphere::new().with_transform(scaling(0.5, 0.5, 0.5));

        let w = World::create_default_world();

        assert!(w.lights.contains(&light));
    }

    #[test]
    fn intersect_world_with_ray() {
        let w = World::create_default_world();
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0),
                         Tuple::vector(0.0, 0.0, 1.0));

        let xs = w.intersect(r);

        assert_eq!(4, xs.len());
        assert_eq!(4.0, xs[0].t);
        assert_eq!(4.5, xs[1].t);
        assert_eq!(5.5, xs[2].t);
        assert_eq!(6.0, xs[3].t);
    }
}