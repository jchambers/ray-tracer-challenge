use std::ops::{Add, Div, Mul, Neg, Sub};

#[cfg(test)]
use assert_float_eq::assert_f64_near;

pub struct Point {
    components: [f64; 4],
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            components: [x, y, z, 1.0],
        }
    }

    pub fn components(&self) -> [f64; 4] {
        self.components
    }

    #[cfg(test)]
    pub fn assert_approx_eq(&self, other: &Point) {
        assert_f64_near!(self.components[0], other.components[0]);
        assert_f64_near!(self.components[1], other.components[1]);
        assert_f64_near!(self.components[2], other.components[2]);
    }
}

impl From<[f64; 4]> for Point {
    fn from(components: [f64; 4]) -> Self {
        // TODO What if the last component isn't 1.0?
        Point { components }
    }
}

impl Add<&Vector> for Point {
    type Output = Self;

    fn add(self, rhs: &Vector) -> Self::Output {
        Self::new(
            self.components[0] + rhs.components[0],
            self.components[1] + rhs.components[1],
            self.components[2] + rhs.components[2],
        )
    }
}

impl Sub<&Vector> for Point {
    type Output = Self;

    fn sub(self, rhs: &Vector) -> Self::Output {
        Self::new(
            self.components[0] - rhs.components[0],
            self.components[1] - rhs.components[1],
            self.components[2] - rhs.components[2],
        )
    }
}

impl Sub<&Point> for Point {
    type Output = Vector;

    fn sub(self, rhs: &Point) -> Self::Output {
        Vector::new(
            self.components[0] - rhs.components[0],
            self.components[1] - rhs.components[1],
            self.components[2] - rhs.components[2],
        )
    }
}

pub struct Vector {
    components: [f64; 4],
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            components: [x, y, z, 0.0],
        }
    }

    pub fn components(&self) -> [f64; 4] {
        self.components
    }

    pub fn magnitude(&self) -> f64 {
        ((self.components[0] * self.components[0])
            + (self.components[1] * self.components[1])
            + (self.components[2] * self.components[2]))
            .sqrt()
    }

    pub fn normalize(&self) -> Self {
        // TODO Do we need to do something more than panic for zero-magnitude vectors?
        self / self.magnitude()
    }

    pub fn dot(&self, rhs: &Self) -> f64 {
        (self.components[0] * rhs.components[0])
            + (self.components[1] * rhs.components[1])
            + (self.components[2] * rhs.components[2])
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        Vector::new(
            (self.components[1] * rhs.components[2]) - (self.components[2] * rhs.components[1]),
            (self.components[2] * rhs.components[0]) - (self.components[0] * rhs.components[2]),
            (self.components[0] * rhs.components[1]) - (self.components[1] * rhs.components[0]),
        )
    }

    #[cfg(test)]
    pub fn assert_appeox_eq(&self, other: &Vector) {
        assert_f64_near!(self.components[0], other.components[0]);
        assert_f64_near!(self.components[1], other.components[1]);
        assert_f64_near!(self.components[2], other.components[2]);

    }
}

impl From<[f64; 4]> for Vector {
    fn from(components: [f64; 4]) -> Self {
        // TODO What if the last component isn't 0.0?
        Vector { components }
    }
}

impl Add<&Vector> for Vector {
    type Output = Self;

    fn add(self, rhs: &Vector) -> Self::Output {
        Vector::new(
            self.components[0] + rhs.components[0],
            self.components[1] + rhs.components[1],
            self.components[2] + rhs.components[2],
        )
    }
}

impl Sub<&Vector> for Vector {
    type Output = Self;

    fn sub(self, rhs: &Vector) -> Self::Output {
        Vector::new(
            self.components[0] - rhs.components[0],
            self.components[1] - rhs.components[1],
            self.components[2] - rhs.components[2],
        )
    }
}

impl Mul<f64> for Vector {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector::new(
            self.components[0] * rhs,
            self.components[1] * rhs,
            self.components[2] * rhs,
        )
    }
}

impl Div<f64> for Vector {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        &self / rhs
    }
}

impl Div<f64> for &Vector {
    type Output = Vector;

    fn div(self, rhs: f64) -> Self::Output {
        Vector::new(
            self.components[0] / rhs,
            self.components[1] / rhs,
            self.components[2] / rhs,
        )
    }
}

impl Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vector::new(
            -self.components[0],
            -self.components[1],
            -self.components[2],
        )
    }
}

#[cfg(test)]
mod test {
    use crate::vector::{Point, Vector};
    use assert_float_eq::assert_f64_near;

    #[test]
    fn test_point_add_vector() {
        Point::new(5.0, 7.0, 9.0)
            .assert_approx_eq(&(Point::new(1.0, 2.0, 3.0) + &Vector::new(4.0, 5.0, 6.0)));
    }

    #[test]
    fn test_point_sub_vector() {
        Point::new(-5.0, -3.0, -1.0)
            .assert_approx_eq(&(Point::new(1.0, 2.0, 3.0) - &Vector::new(6.0, 5.0, 4.0)));
    }

    #[test]
    fn test_point_sub_point() {
        Vector::new(-5.0, -3.0, -1.0)
            .assert_appeox_eq(&(Point::new(1.0, 2.0, 3.0) - &Point::new(6.0, 5.0, 4.0)));
    }

    #[test]
    fn test_vector_add_vector() {
        Vector::new(5.0, 7.0, 9.0)
            .assert_appeox_eq(&(Vector::new(1.0, 2.0, 3.0) + &Vector::new(4.0, 5.0, 6.0)));
    }

    #[test]
    fn test_vector_sub_vector() {
        &Vector::new(-5.0, -3.0, -1.0)
            .assert_appeox_eq(&(Vector::new(1.0, 2.0, 3.0) - &Vector::new(6.0, 5.0, 4.0)));
    }

    #[test]
    fn test_vector_neg() {
        Vector::new(1.0, -2.0, 3.0)
            .assert_appeox_eq(&(-Vector::new(-1.0, 2.0, -3.0)));
    }

    #[test]
    fn test_vector_mul() {
        Vector::new(3.5, -7.0, 10.5)
            .assert_appeox_eq(&(Vector::new(1.0, -2.0, 3.0) * 3.5));

        Vector::new(0.5, -1.0, 1.5)
            .assert_appeox_eq(&(Vector::new(1.0, -2.0, 3.0) * 0.5));
    }

    #[test]
    fn test_vector_div() {
        Vector::new(2.0, -4.0, 6.0)
            .assert_appeox_eq(&(Vector::new(1.0, -2.0, 3.0) / 0.5));

        Vector::new(0.5, -1.0, 1.5)
            .assert_appeox_eq(&(Vector::new(1.0, -2.0, 3.0) / 2.0));
    }

    #[test]
    fn test_vector_magnitude() {
        assert_f64_near!(1.0, Vector::new(1.0, 0.0, 0.0).magnitude());
        assert_f64_near!(1.0, Vector::new(0.0, 1.0, 0.0).magnitude());
        assert_f64_near!(1.0, Vector::new(0.0, 0.0, 1.0).magnitude());
        assert_f64_near!(14.0f64.sqrt(), Vector::new(1.0, -2.0, 3.0).magnitude());
    }

    #[test]
    fn test_vector_normalize() {
        Vector::new(1.0, 0.0, 0.0)
            .assert_appeox_eq(&Vector::new(4.0, 0.0, 0.0).normalize());

        let normalized = Vector::new(1.0, -2.0, 3.0).normalize();
        let sqrt14 = 14.0f64.sqrt();

        Vector::new(1.0 / sqrt14, -2.0 / sqrt14, 3.0 / sqrt14)
            .assert_appeox_eq(&normalized);

        assert_f64_near!(1.0, normalized.magnitude());
    }

    #[test]
    fn test_vector_dot() {
        assert_f64_near!(
            20.0,
            Vector::new(1.0, 2.0, 3.0).dot(&Vector::new(2.0, 3.0, 4.0))
        )
    }

    #[test]
    fn test_vector_cross() {
        let a = Vector::new(1.0, 2.0, 3.0);
        let b = Vector::new(2.0, 3.0, 4.0);

        Vector::new(-1.0, 2.0, -1.0).assert_appeox_eq(&a.cross(&b));
        Vector::new(1.0, -2.0, 1.0).assert_appeox_eq(&b.cross(&a));
    }
}
