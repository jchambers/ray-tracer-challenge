use crate::geometry::intersection::Intersection;
use crate::matrix::Matrix;
use crate::vector::{Point, Vector};
use std::ops::Mul;

pub struct Ray {
    origin: Point,
    direction: Vector,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Self {
        Self { origin, direction }
    }

    pub fn origin(&self) -> &Point {
        &self.origin
    }

    pub fn direction(&self) -> &Vector {
        &self.direction
    }
}

impl Mul<&Ray> for Matrix<4> {
    type Output = Ray;

    fn mul(self, rhs: &Ray) -> Self::Output {
        &self * rhs
    }
}

impl Mul<&Ray> for &Matrix<4> {
    type Output = Ray;

    fn mul(self, rhs: &Ray) -> Self::Output {
        Ray {
            origin: self * &rhs.origin,
            direction: self * &rhs.direction,
        }
    }
}

pub trait IntersectRay {
    fn intersect(&self, ray: &Ray) -> Vec<Intersection>;
}

#[cfg(test)]
mod test {
    use crate::geometry::ray::Ray;
    use crate::transform;
    use crate::transform::Transformation;
    use crate::vector::{Point, Vector};

    #[test]
    fn test_transform() {
        {
            let transformation = transform::transform(&[Transformation::Translate(3.0, 4.0, 5.0)]);
            let ray =
                transformation * &Ray::new(Point::new(1.0, 2.0, 3.0), Vector::new(0.0, 1.0, 0.0));

            ray.origin.assert_approx_eq(&Point::new(4.0, 6.0, 8.0));
            ray.direction.assert_appeox_eq(&Vector::new(0.0, 1.0, 0.0));
        }

        {
            let transformation = transform::transform(&[Transformation::Scale(2.0, 3.0, 4.0)]);
            let ray =
                transformation * &Ray::new(Point::new(1.0, 2.0, 3.0), Vector::new(0.0, 1.0, 0.0));

            ray.origin.assert_approx_eq(&Point::new(2.0, 6.0, 12.0));
            ray.direction.assert_appeox_eq(&Vector::new(0.0, 3.0, 0.0));
        }
    }
}
