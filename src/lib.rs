pub mod tuple;
pub mod color;
pub mod canvas;
pub mod matrices;
mod transformation;

pub use tuple::Tuple;
pub use canvas::Canvas;
pub use color::Color;
pub use matrices::*;

pub mod comparison {
    pub mod epsilon;
    pub mod approx_eq;

    pub use epsilon:: {EPSILON, LOW_EPSILON};
    pub use approx_eq::ApproxEq;
}

