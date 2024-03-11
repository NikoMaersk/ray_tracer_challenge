use std::ops::Index;
use crate::{Ray, Tuple};
use crate::shapes::shape_enum::{RayInteractable, Shape};

#[derive(Copy, Clone, Debug)]
pub struct Intersection {
    pub t: f64,
    pub object: Shape,
}

impl Intersection {
    pub fn new(t: f64, obj: Shape) -> Self {
        Intersection { t, object: obj }
    }

    pub fn prepare_computations(&self, ray: Ray) -> Computations {

        let point = ray.position(self.t);
        let eye_v = -ray.direction;

        let shape = match self.object {
            Shape::Sphere(sphere) => { sphere }
        };

        let normal_v = shape.normal_at(point);


        let comps = Computations::new(self.t, self.object, point, eye_v, normal_v);


        comps
    }
}


impl PartialEq for Intersection {
    fn eq(&self, other: &Self) -> bool {
        self.t == other.t
    }
}

#[derive(Debug)]
pub struct Intersections {
    intersections: Vec<Intersection>
}

impl Intersections {
    pub fn new() -> Self {
        Self {
            intersections: Vec::<Intersection>::new(),
        }
    }

    pub fn new_from_vec(mut vec: Vec<Intersection>) -> Self {
        vec.sort_unstable_by(|a, b| a.t.partial_cmp(&b.t).unwrap());

        Intersections { intersections: vec }
    }

    pub fn len(&self) -> usize {
        self.intersections.len()
    }

    pub fn is_empty(&self) -> bool {
        self.intersections.is_empty()
    }

    pub fn push(&mut self, i: Intersection) {
        self.intersections.push(i);
        self.sort()
    }

    pub fn push_vec(&mut self, vec: Vec<Intersection>) {
        for i in vec {
            self.intersections.push(i);
        }

        self.sort();
    }

    pub fn sort(&mut self) {
        self.intersections.sort_unstable_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
    }

    pub fn hit(&self) -> Option<&Intersection> {
        self.intersections.iter().find(|i| i.t >= 0.0)
    }
}

impl<'a> Index<usize> for Intersections {
    type Output = Intersection;

    fn index(&self, index: usize) -> &Self::Output {
        &self.intersections[index]
    }
}


pub struct Computations {
    pub t: f64,
    pub object: Shape,
    pub point: Tuple,
    pub eye_v: Tuple,
    pub normal_v: Tuple
}

impl Computations {
    pub fn new(t: f64, object: Shape, point: Tuple, eye_v: Tuple, normal_v: Tuple) -> Self {
        Computations {
            t,
            object,
            point,
            eye_v,
            normal_v,
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::shapes::sphere::Sphere;
    use crate::intersection::{Intersection, Intersections};
    use crate::{Ray, Tuple};
    use crate::shapes::shape_enum::Shape;

    #[test]
    fn intersection_encapsulates_t_and_object() {
        let s = Sphere::new();

        let i = Intersection::new(3.5, Shape::Sphere(s));

        let expected_s = match i.object {
            Shape::Sphere(sphere) => sphere
        };

        assert_eq!(3.5, i.t);
        assert_eq!(s, expected_s);
    }

    #[test]
    fn aggregating_intersections() {
        let s = Sphere::new();
        let shape = Shape::Sphere(s);

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
        let shape = Shape::Sphere(s);

        let i1 = Intersection::new(1.0, shape);
        let i2 = Intersection::new(2.0, shape);

        let xs = Intersections::new_from_vec(vec![i1, i2]);

        assert_eq!(i1, *xs.hit().unwrap());
    }

    #[test]
    fn hit_when_some_intersections_have_negative_t() {
        let s = Sphere::new();
        let shape = Shape::Sphere(s);

        let i1 = Intersection::new(-1.0, shape);
        let i2 = Intersection::new(1.0, shape);

        let xs = Intersections::new_from_vec(vec![i1, i2]);

        assert_eq!(i2, *xs.hit().unwrap());
    }


    #[test]
    fn hit_when_all_intersections_have_negative_t() {
        let s = Sphere::new();
        let shape = Shape::Sphere(s);

        let i1 = Intersection::new(-2.0, shape);
        let i2 = Intersection::new(-1.0, shape);

        let xs = Intersections::new_from_vec(vec![i1, i2]);

        assert_eq!(None, xs.hit());
    }


    #[test]
    fn hit_is_always_lowest() {
        let s = Sphere::new();
        let shape = Shape::Sphere(s);

        let i1 = Intersection::new(5.0, shape);
        let i2 = Intersection::new(7.0, shape);
        let i3 = Intersection::new(-3.0, shape);
        let i4 = Intersection::new(2.0, shape);

        let xs = Intersections::new_from_vec(vec![i1, i2, i3, i4]);

        assert_eq!(i4, *xs.hit().unwrap());
    }


    #[test]
    fn precomputing_state_of_an_intersection() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let shape = Sphere::new();
        let i = Intersection::new(4.0, Shape::Sphere(shape));

        let comps = i.prepare_computations(r);

        assert_eq!(i.t, comps.t);
        assert_eq!(Tuple::point(0.0, 0.0, -1.0), comps.point);
        assert_eq!(Tuple::vector(0.0, 0.0, -1.0), comps.eye_v);
        assert_eq!(Tuple::vector(0.0, 0.0, -1.0), comps.normal_v);
    }
}
