use crate::shapes::sphere::Sphere;

#[derive(Copy, Clone, Debug)]
pub enum Shape<'a> {
    Sphere(&'a Sphere),
}