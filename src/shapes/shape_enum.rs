use crate::intersection::Intersection;
use crate::{Material, Ray, Tuple};
use crate::shapes::sphere::Sphere;


pub trait RayInteractable {
    fn intersect(&self, ray: Ray) -> Vec<Intersection>;
    fn normal_at(&self, point: Tuple) -> Tuple;
    fn material(&self) -> Material;
}


#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Shape {
    Sphere(Sphere),
}

impl RayInteractable for Shape {
    fn intersect(&self, ray: Ray) -> Vec<Intersection> {
        match self {
            Shape::Sphere(sphere) => sphere.intersect(ray),
        }
    }

    fn normal_at(&self, point: Tuple) -> Tuple {
        match self {
            Shape::Sphere(sphere) => sphere.normal_at(point),
        }
    }

    fn material(&self) -> Material {
        match self {
            Shape::Sphere(sphere) => sphere.material(),
        }
    }
}


