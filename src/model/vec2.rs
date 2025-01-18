use std::f64::consts::PI;
use std::ops::{Add, Div, DivAssign, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    x: f64,
    y: f64,
}

impl Vec2 {
    pub const ZERO: Vec2 = Vec2 { x: 0.0, y: 0.0 };

    pub fn new(x: f64, y: f64) -> Vec2 {
        Vec2 { x, y }
    }

    pub fn is_zero(&self) -> bool {
        self == &Self::ZERO
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn normalize(&mut self) {
        let length = self.length();
        if length != 0.0 {
            *self /= length
        }
    }

    pub fn normalized(&self) -> Vec2 {
        let mut clone = self.clone();
        clone.normalize();
        clone
    }

    pub fn angle_between(&self, other: &Vec2) -> f64 {
        if self.is_zero() || other.is_zero() {
            0.0
        } else {
            let angle = other.y.atan2(other.x) - self.y.atan2(self.x);
            if angle > PI {
                angle - 2.0 * PI
            } else if angle <= -PI {
                angle + 2.0 * PI
            } else {
                angle
            }
        }
    }
}

impl Add for Vec2 {
    type Output = Vec2;
    fn add(self, other: Vec2) -> Vec2 {
        Vec2::new(self.x + other.x, self.y + other.y)
    }
}

impl Sub for Vec2 {
    type Output = Vec2;
    fn sub(self, other: Vec2) -> Vec2 {
        Vec2::new(self.x - other.x, self.y - other.y)
    }
}

impl Mul<f64> for Vec2 {
    type Output = Vec2;
    fn mul(self, other: f64) -> Vec2 {
        Vec2::new(self.x * other, self.y * other)
    }
}

impl Div<f64> for Vec2 {
    type Output = Vec2;
    fn div(self, other: f64) -> Vec2 {
        Vec2::new(self.x / other, self.y / other)
    }
}

impl DivAssign<f64> for Vec2 {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl Neg for Vec2 {
    type Output = Vec2;
    fn neg(self) -> Vec2 {
        Vec2::new(-self.x, -self.y)
    }
}

impl From<(f64, f64)> for Vec2 {
    fn from((x, y): (f64, f64)) -> Self {
        Vec2::new(x, y)
    }
}

#[cfg(test)]
mod tests {
    use crate::model::vec2::Vec2;
    use proptest::arbitrary::Arbitrary;
    use proptest::prelude::{BoxedStrategy, Strategy};
    use std::ops::Range;

    #[derive(Debug, Clone, PartialEq)]
    pub struct Vec2Parameters {
        pub x_range: Range<f64>,
        pub y_range: Range<f64>,
        pub non_zero: bool,
    }

    impl Default for Vec2Parameters {
        fn default() -> Self {
            Vec2Parameters {
                x_range: -100.0..100.0,
                y_range: -100.0..100.0,
                non_zero: false,
            }
        }
    }

    impl Arbitrary for Vec2 {
        type Parameters = Vec2Parameters;

        fn arbitrary_with(params: Self::Parameters) -> Self::Strategy {
            (params.x_range, params.y_range)
                .prop_map(Vec2::from)
                .prop_filter("Must not be zero", move |v| {
                    !params.non_zero || !v.is_zero()
                })
                .boxed()
        }

        type Strategy = BoxedStrategy<Self>;
    }

    impl Vec2 {
        pub fn non_zero() -> impl Strategy<Value = Vec2> {
            let mut params = Vec2Parameters::default();
            params.non_zero = true;
            Self::arbitrary_with(params)
        }
    }

    mod is_zero {
        use crate::model::vec2::tests::Vec2Parameters;
        use crate::model::vec2::Vec2;
        use proptest::prelude::Arbitrary;
        use proptest::proptest;

        #[test]
        fn should_be_true_when_zero() {
            assert!(Vec2::ZERO.is_zero());
        }

        #[test]
        fn should_be_false_when_not_zero() {
            proptest!(|(v in Vec2::non_zero())| {
                assert!(!v.is_zero());
            })
        }
    }

    mod length {
        use crate::model::vec2::Vec2;
        use proptest::proptest;

        #[test]
        fn should_have_correct_length_when_one_component_is_zero() {
            proptest!(|(len in -100.0..100.0)| {
                    for v in &[
                        Vec2::new(0.0, len),
                        Vec2::new(len, 0.0),
                        Vec2::new(0.0, -len),
                        Vec2::new(-len, 0.0),
                    ] {
                        assert_eq!(v.length(), len.abs());
                    }
            })
        }
    }
}
