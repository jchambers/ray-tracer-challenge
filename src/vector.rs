use std::ops::{Add, Div, Mul, Neg, Sub};

pub struct Point {
    components: [f64; 4],
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            components: [x, y, z, 1.0],
        }
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

    fn assert_points_eq(a: &Point, b: &Point) {
        assert_f64_near!(a.components[0], b.components[0], 1);
        assert_f64_near!(a.components[1], b.components[1], 1);
        assert_f64_near!(a.components[2], b.components[2], 1);
    }

    fn assert_vectors_eq(a: &Vector, b: &Vector) {
        assert_f64_near!(a.components[0], b.components[0], 1);
        assert_f64_near!(a.components[1], b.components[1], 1);
        assert_f64_near!(a.components[2], b.components[2], 1);
    }

    #[test]
    fn test_point_add_vector() {
        assert_points_eq(
            &Point::new(5.0, 7.0, 9.0),
            &(Point::new(1.0, 2.0, 3.0) + &Vector::new(4.0, 5.0, 6.0)),
        )
    }

    #[test]
    fn test_point_sub_vector() {
        assert_points_eq(
            &Point::new(-5.0, -3.0, -1.0),
            &(Point::new(1.0, 2.0, 3.0) - &Vector::new(6.0, 5.0, 4.0)),
        )
    }

    #[test]
    fn test_point_sub_point() {
        assert_vectors_eq(
            &Vector::new(-5.0, -3.0, -1.0),
            &(Point::new(1.0, 2.0, 3.0) - &Point::new(6.0, 5.0, 4.0)),
        )
    }

    #[test]
    fn test_vector_add_vector() {
        assert_vectors_eq(
            &Vector::new(5.0, 7.0, 9.0),
            &(Vector::new(1.0, 2.0, 3.0) + &Vector::new(4.0, 5.0, 6.0)),
        )
    }

    #[test]
    fn test_vector_sub_vector() {
        assert_vectors_eq(
            &Vector::new(-5.0, -3.0, -1.0),
            &(Vector::new(1.0, 2.0, 3.0) - &Vector::new(6.0, 5.0, 4.0)),
        )
    }

    #[test]
    fn test_vector_neg() {
        assert_vectors_eq(
            &Vector::new(1.0, -2.0, 3.0),
            &(-Vector::new(-1.0, 2.0, -3.0)),
        )
    }

    #[test]
    fn test_vector_mul() {
        assert_vectors_eq(
            &Vector::new(3.5, -7.0, 10.5),
            &(Vector::new(1.0, -2.0, 3.0) * 3.5),
        );

        assert_vectors_eq(
            &Vector::new(0.5, -1.0, 1.5),
            &(Vector::new(1.0, -2.0, 3.0) * 0.5),
        );
    }

    #[test]
    fn test_vector_div() {
        assert_vectors_eq(
            &Vector::new(2.0, -4.0, 6.0),
            &(Vector::new(1.0, -2.0, 3.0) / 0.5),
        );

        assert_vectors_eq(
            &Vector::new(0.5, -1.0, 1.5),
            &(Vector::new(1.0, -2.0, 3.0) / 2.0),
        );
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
        assert_vectors_eq(
            &Vector::new(1.0, 0.0, 0.0),
            &Vector::new(4.0, 0.0, 0.0).normalize(),
        );

        let normalized = Vector::new(1.0, -2.0, 3.0).normalize();
        let sqrt14 = 14.0f64.sqrt();

        assert_f64_near!(1.0 / sqrt14, normalized.components[0]);
        assert_f64_near!(-2.0 / sqrt14, normalized.components[1]);
        assert_f64_near!(3.0 / sqrt14, normalized.components[2]);
        assert_f64_near!(1.0, normalized.magnitude());
    }

    #[test]
    fn test_vector_dot() {
        assert_f64_near!(20.0, Vector::new(1.0, 2.0, 3.0).dot(&Vector::new(2.0, 3.0, 4.0)))
    }

    #[test]
    fn test_vector_cross() {
        let a = Vector::new(1.0, 2.0, 3.0);
        let b = Vector::new(2.0, 3.0, 4.0);

        assert_vectors_eq(
            &Vector::new(-1.0, 2.0, -1.0),
            &a.cross(&b)
        );

        assert_vectors_eq(
            &Vector::new(1.0, -2.0, 1.0),
            &b.cross(&a)
        );
    }
}
