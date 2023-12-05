use float_cmp::approx_eq;
use super::epsilon::{EPSILON, LOW_EPSILON};
pub trait ApproxEq<Rhs = Self> {
    fn approx_eq(self, compare_to: Rhs) -> bool;
    fn approx_eq_low_precision(self, compare_to: Rhs) -> bool;
    fn approx_eq_epsilon(self, compare_to: Rhs, epsilon: f32) -> bool;
}

impl ApproxEq for f32 {
    fn approx_eq(self, compare_to: Self) -> bool {
        self.approx_eq_epsilon(compare_to, EPSILON)
    }

    fn approx_eq_low_precision(self, compare_to: Self) -> bool {
        self.approx_eq_epsilon(compare_to, LOW_EPSILON)
    }

    fn approx_eq_epsilon(self, compare_to: Self, epsilon: f32) -> bool {
        approx_eq!(f32, self, compare_to, epsilon = epsilon)
    }
}