use float_cmp::approx_eq;
use super::epsilon::{LOW_EPSILON};
pub trait ApproxEq<Rhs = Self> {
    fn approx_eq(self, compare_to: Rhs) -> bool;
    fn approx_eq_low_precision(self, compare_to: Rhs) -> bool;
    fn approx_eq_epsilon(self, compare_to: Rhs, epsilon: f64) -> bool;
}

impl ApproxEq for f64 {
    fn approx_eq(self, compare_to: Self) -> bool {
        self.approx_eq_epsilon(compare_to, f64::EPSILON)
    }

    fn approx_eq_low_precision(self, compare_to: Self) -> bool {
        self.approx_eq_epsilon(compare_to, LOW_EPSILON)
    }

    fn approx_eq_epsilon(self, compare_to: Self, epsilon: f64) -> bool {
        approx_eq!(f64, self, compare_to, epsilon = epsilon)
    }
}
