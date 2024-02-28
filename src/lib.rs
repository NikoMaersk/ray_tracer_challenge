pub mod tuple;
pub mod color;
pub mod canvas;
pub mod matrices;
mod transformation;
mod ray;
mod intersection;

pub use tuple::Tuple;
pub use canvas::Canvas;
pub use color::Color;
pub use matrices::*;
pub use transformation::*;

pub mod shapes {
    pub mod sphere;
}

pub mod comparison {
    pub mod epsilon;
    pub mod approx_eq;

    pub use epsilon:: {EPSILON, LOW_EPSILON};
    pub use approx_eq::ApproxEq;
}

