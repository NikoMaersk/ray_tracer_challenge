use crate::shapes::sphere::Sphere;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Shape {
    Sphere(Sphere),
}