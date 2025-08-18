use std::ops::{Add, Mul, Sub};

#[cfg(test)]
use assert_float_eq::assert_f64_near;

#[cfg(test)]
use assert_float_eq::assert_float_relative_eq;

pub const BLACK: Color = Color {
    components: [0.0, 0.0, 0.0],
};

pub const WHITE: Color = Color {
    components: [1.0, 1.0, 1.0],
};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Color {
    components: [f64; 3],
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Color {
            components: [r, g, b],
        }
    }

    pub fn components(&self) -> &[f64] {
        &self.components
    }

    #[cfg(test)]
    pub fn assert_approx_eq(&self, other: &Color) {
        assert_f64_near!(self.components[0], other.components[0]);
        assert_f64_near!(self.components[1], other.components[1]);
        assert_f64_near!(self.components[2], other.components[2]);
    }

    #[cfg(test)]
    pub fn assert_approx_eq_epsilon(&self, other: &Color, epsilon: f64) {
        assert_float_relative_eq!(self.components[0], other.components[0], epsilon);
        assert_float_relative_eq!(self.components[1], other.components[1], epsilon);
        assert_float_relative_eq!(self.components[2], other.components[2], epsilon);
    }
}

impl Add<Color> for Color {
    type Output = Self;

    fn add(self, rhs: Color) -> Self::Output {
        Color::new(
            self.components[0] + rhs.components[0],
            self.components[1] + rhs.components[1],
            self.components[2] + rhs.components[2],
        )
    }
}

impl Sub<Color> for Color {
    type Output = Self;

    fn sub(self, rhs: Color) -> Self::Output {
        Color::new(
            self.components[0] - rhs.components[0],
            self.components[1] - rhs.components[1],
            self.components[2] - rhs.components[2],
        )
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Color::new(
            self.components[0] * rhs,
            self.components[1] * rhs,
            self.components[2] * rhs,
        )
    }
}

impl Mul<Color> for Color {
    type Output = Self;

    fn mul(self, rhs: Color) -> Self::Output {
        Color::new(
            self.components[0] * rhs.components[0],
            self.components[1] * rhs.components[1],
            self.components[2] * rhs.components[2],
        )
    }
}

#[cfg(test)]
mod test {
    use crate::color::Color;

    #[test]
    fn test_color_add() {
        Color::new(1.6, 0.7, 1.0)
            .assert_approx_eq(&(Color::new(0.9, 0.6, 0.75) + Color::new(0.7, 0.1, 0.25)));
    }

    #[test]
    fn test_color_sub() {
        Color::new(0.2, 0.5, 0.5)
            .assert_approx_eq(&(Color::new(0.9, 0.6, 0.75) - Color::new(0.7, 0.1, 0.25)))
    }

    #[test]
    fn test_color_mul_scalar() {
        Color::new(0.4, 0.6, 0.8).assert_approx_eq(&(Color::new(0.2, 0.3, 0.4) * 2.0));
    }

    #[test]
    fn test_color_mul_color() {
        Color::new(0.9, 0.2, 0.04)
            .assert_approx_eq(&(Color::new(1.0, 0.2, 0.4) * Color::new(0.9, 1.0, 0.1)));
    }
}
