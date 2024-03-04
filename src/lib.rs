pub mod tuple;
pub mod color;
pub mod canvas;
pub mod matrices;
pub mod transformation;
pub mod ray;
pub mod intersection;
mod lights;
mod materials;

pub use tuple::Tuple;
pub use canvas::Canvas;
pub use color::Color;
pub use matrices::*;
pub use transformation::*;
pub use ray::*;
pub use materials::Material;
pub use lights::Light;

pub mod shapes {
    pub mod sphere;
    pub mod shape_enum;

    pub use sphere::Sphere;
    pub use shape_enum::Shape;
}

pub mod comparison {
    pub mod epsilon;
    pub mod approx_eq;

    pub use epsilon:: {EPSILON, LOW_EPSILON};
    pub use approx_eq::ApproxEq;
}

