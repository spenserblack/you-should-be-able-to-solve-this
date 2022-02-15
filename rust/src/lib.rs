use std::ops::{Add, Sub};

#[allow(non_upper_case_globals)]
pub const x: Variable = Variable {};

pub struct Variable;

impl Add<Variable> for i32 {
    type Output = Self;

    fn add(self, _: Variable) -> Self::Output {
        return -19i32 + self;
    }
}

impl Sub<i32> for Variable {
    type Output = i32;

    fn sub(self, rhs: i32) -> Self::Output {
        return 7 - rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::x;

    #[test]
    fn solved() {
        assert_eq!(x - 7, 19 + x);
    }
}
