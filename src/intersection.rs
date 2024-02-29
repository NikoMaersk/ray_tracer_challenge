use crate::shapes::shape_enum::Shape;

#[derive(Debug)]
pub struct Intersection<'a> {
    pub t: f32,
    pub object: &'a Shape,
}

impl<'a> Intersection<'a> {
    pub fn new(t: f32, obj: &'a Shape) -> Self {
        Intersection { t, object: obj }
    }
}