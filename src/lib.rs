pub mod tuple;
pub mod color;
pub mod canvas;

pub use tuple::Tuple;

pub mod comparison {
    pub use epsilon::EPSILON;
    pub use epsilon::LOW_EPSILON;
    pub use approx_eq::ApproxEq;

    pub mod epsilon;
    pub mod approx_eq;
}

