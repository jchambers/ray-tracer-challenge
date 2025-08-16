use crate::geometry::intersection::Intersection;
use crate::geometry::ray::{IntersectRay, Ray};
use crate::matrix::Matrix;
use crate::vector::Point;

pub struct Sphere {
    transformation: Matrix<4>,
}

impl Default for Sphere {
    fn default() -> Self {
        Sphere {
            transformation: Matrix::<4>::identity(),
        }
    }
}

impl Sphere {
    pub fn new(transformation: Matrix<4>) -> Self {
        Sphere { transformation }
    }
}

impl IntersectRay for Sphere {
    fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        // TODO Can we tighten things up at a type level to guarantee that the transformation matrix
        //  is always an affine transform and therefore invertible?
        let ray = self.transformation.inverse().unwrap() * ray;
        let sphere_to_ray = ray.origin() - &Point::new(0.0, 0.0, 0.0);

        let a = ray.direction().dot(ray.direction());
        let b = 2.0 * ray.direction().dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;

        let discriminant = (b * b) - (4.0 * a * c);

        if discriminant < 0.0 {
            Vec::new()
        } else {
            let sqrt_discriminant = discriminant.sqrt();

            let distances = (
                (-b - sqrt_discriminant) / (2.0 * a),
                (-b + sqrt_discriminant) / (2.0 * a),
            );

            // Return intersections in ascending order of distance along ray's path
            vec![
                Intersection::new(distances.0.min(distances.1), self),
                Intersection::new(distances.0.max(distances.1), self),
            ]
        }
    }
}

#[cfg(test)]
mod test {
    use crate::geometry::ray::{IntersectRay, Ray};
    use crate::geometry::sphere::Sphere;
    use crate::transform;
    use crate::transform::Transformation;
    use crate::vector::{Point, Vector};
    use assert_float_eq::assert_f64_near;

    #[test]
    fn test_intersect_ray() {
        {
            let sphere = Sphere::default();

            let intersections = sphere.intersect(&Ray::new(
                Point::new(0.0, 0.0, -5.0),
                Vector::new(0.0, 0.0, 1.0),
            ));

            assert_eq!(2, intersections.len());
            assert_f64_near!(4.0, intersections[0].distance());
            assert_f64_near!(6.0, intersections[1].distance());
        }

        {
            let sphere = Sphere::default();

            let intersections = sphere.intersect(&Ray::new(
                Point::new(0.0, 1.0, -5.0),
                Vector::new(0.0, 0.0, 1.0),
            ));

            assert_eq!(2, intersections.len());
            assert_f64_near!(5.0, intersections[0].distance());
            assert_f64_near!(5.0, intersections[1].distance());
        }

        {
            let sphere = Sphere::default();

            let intersections = sphere.intersect(&Ray::new(
                Point::new(0.0, 2.0, -5.0),
                Vector::new(0.0, 0.0, 1.0),
            ));

            assert!(intersections.is_empty());
        }

        {
            let sphere = Sphere::default();

            let intersections = sphere.intersect(&Ray::new(
                Point::new(0.0, 0.0, 0.0),
                Vector::new(0.0, 0.0, 1.0),
            ));

            assert_eq!(2, intersections.len());
            assert_f64_near!(-1.0, intersections[0].distance());
            assert_f64_near!(1.0, intersections[1].distance());
        }

        {
            let sphere = Sphere::new(transform::transform(&[Transformation::Scale(
                2.0, 2.0, 2.0,
            )]));

            let intersections = sphere.intersect(&Ray::new(
                Point::new(0.0, 0.0, -5.0),
                Vector::new(0.0, 0.0, 1.0),
            ));

            assert_eq!(2, intersections.len());
            assert_f64_near!(3.0, intersections[0].distance());
            assert_f64_near!(7.0, intersections[1].distance());
        }

        {
            let sphere = Sphere::new(transform::transform(&[Transformation::Translate(
                5.0, 0.0, 0.0,
            )]));

            let intersections = sphere.intersect(&Ray::new(
                Point::new(0.0, 0.0, -5.0),
                Vector::new(0.0, 0.0, 1.0),
            ));

            assert!(intersections.is_empty());
        }
    }
}
