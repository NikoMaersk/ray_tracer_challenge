use crate::shapes::sphere::Sphere;

#[derive(Copy, Clone, Debug)]
pub enum Shape {
    Sphere(Sphere)
}