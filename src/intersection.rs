use std::ops::Index;
use crate::shapes::shape_enum::Shape;

#[derive(Debug)]
pub struct Intersection<'a> {
    pub t: f32,
    pub object: Shape<'a>,
}

impl<'a> Intersection<'a> {
    pub fn new(t: f32, obj: Shape<'a>) -> Self {
        Intersection { t, object: obj }
    }
}

#[derive(Debug)]
pub struct Intersections<'a> {
    intersections: Vec<Intersection<'a>>
}

impl<'a> Intersections<'a> {
    pub fn new() -> Self {
        Self {
            intersections: Vec::<Intersection<'a>>::new(),
        }
    }

    pub fn new_from_vec(vec: Vec<Intersection<'a>>) -> Self {
        Intersections { intersections: vec }
    }

    pub fn len(&self) -> usize {
        self.intersections.len()
    }

    pub fn is_empty(&self) -> bool {
        self.intersections.is_empty()
    }

    pub fn push(&mut self, i: Intersection<'a>) {
        self.intersections.push(i)
    }
}

impl<'a> Index<usize> for Intersections<'a> {
    type Output = Intersection<'a>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.intersections[index]
    }
}

impl<'a> FromIterator<Intersection<'a>> for Intersections<'a> {
    fn from_iter<I: IntoIterator<Item = Intersection<'a>>>(iter: I) -> Self {
        let intersections = iter.into_iter().collect();
        Intersections { intersections }
    }
}


#[cfg(test)]
mod tests {
    use crate::shapes::sphere::Sphere;
    use crate::intersection::{Intersection, Intersections};
    use crate::shapes::shape_enum::Shape;

    #[test]
    fn intersection_encapsulates_t_and_object() {
        let s = Sphere::new();

        let i = Intersection::new(3.5, Shape::Sphere(&s));

        let expected_s = match i.object {
            Shape::Sphere(sphere) => sphere
        };

        assert_eq!(3.5, i.t);
        assert_eq!(s, *expected_s);
    }

    #[test]
    fn aggregating_intersections() {
        let s = Sphere::new();
        let shape = Shape::Sphere(&s);

        let i1 = Intersection::new(1.0, shape);
        let i2 = Intersection::new(2.0, shape);

        let mut xs = Intersections::new();
        xs.push(i1);
        xs.push(i2);

        assert_eq!(2, xs.len());
        assert_eq!(1.0, xs[0].t);
        assert_eq!(2.0, xs[1].t);
    }

    #[test]
    fn hit_when_all_intersections_have_positive_t() {
        let s = Sphere::new();
        let shape = Shape::Sphere(&s);

        let i1 = Intersection::new(1.0, shape);
        let i2 = Intersection::new(2.0, shape);

        let xs = Intersections::new_from_vec(vec![i1, i2]);

        assert_eq!(i1, xs.hit());
    }
}