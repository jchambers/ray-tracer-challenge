use std::ops::{Add, Mul, Sub};

pub struct Color {
    components: [f64; 3]
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Color {
            components: [r, g, b]
        }
    }
}

impl Add<&Color> for Color {
    type Output = Self;

    fn add(self, rhs: &Color) -> Self::Output {
        Color::new(
            self.components[0] + rhs.components[0],
            self.components[1] + rhs.components[1],
            self.components[2] + rhs.components[2],
        )
    }
}

impl Sub<&Color> for Color {
    type Output = Self;

    fn sub(self, rhs: &Color) -> Self::Output {
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

impl Mul<&Color> for Color {
    type Output = Self;

    fn mul(self, rhs: &Color) -> Self::Output {
        Color::new(
            self.components[0] * rhs.components[0],
            self.components[1] * rhs.components[1],
            self.components[2] * rhs.components[2],
        )
    }
}

#[cfg(test)]
mod test {
    use assert_float_eq::assert_f64_near;
    use crate::color::Color;

    fn assert_colors_eq(a: &Color, b: &Color) {
        assert_f64_near!(a.components[0], b.components[0]);
        assert_f64_near!(a.components[1], b.components[1]);
        assert_f64_near!(a.components[2], b.components[2]);
    }

    #[test]
    fn test_color_add() {
        assert_colors_eq(
            &Color::new(1.6, 0.7, 1.0),
            &(Color::new(0.9, 0.6, 0.75) + &Color::new(0.7, 0.1, 0.25)),
        )
    }

    #[test]
    fn test_color_sub() {
        assert_colors_eq(
            &Color::new(0.2, 0.5, 0.5),
            &(Color::new(0.9, 0.6, 0.75) - &Color::new(0.7, 0.1, 0.25)),
        )
    }

    #[test]
    fn test_color_mul_scalar() {
        assert_colors_eq(
            &Color::new(0.4, 0.6, 0.8),
            &(Color::new(0.2, 0.3, 0.4) * 2.0),
        )
    }

    #[test]
    fn test_color_mul_color() {
        assert_colors_eq(
            &Color::new(0.9, 0.2, 0.04),
            &(Color::new(1.0, 0.2, 0.4) * &Color::new(0.9, 1.0, 0.1)),
        )
    }
}
