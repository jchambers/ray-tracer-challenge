use crate::geometry::ray::{IntersectRay, Ray};
use crate::vector::Point;

pub struct Sphere {}

impl Sphere {
    pub fn new() -> Self {
        Sphere {}
    }
}

impl IntersectRay for Sphere {
    fn intersect(&self, ray: &Ray) -> Vec<f64> {
        let sphere_to_ray = ray.origin() - &Point::new(0.0, 0.0, 0.0);

        let a = ray.direction().dot(ray.direction());
        let b = 2.0 * ray.direction().dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;

        let discriminant = (b * b) - (4.0 * a * c);

        if discriminant < 0.0 {
            Vec::new()
        } else {
            let sqrt_discriminant = discriminant.sqrt();

            let intersections = (
                (-b - sqrt_discriminant) / (2.0 * a),
                (-b + sqrt_discriminant) / (2.0 * a),
            );

            // Return intersections in ascending order of distance along ray's path
            vec![
                intersections.0.min(intersections.1),
                intersections.0.max(intersections.1),
            ]
        }
    }
}

#[cfg(test)]
mod test {
    use crate::geometry::ray::{IntersectRay, Ray};
    use crate::geometry::sphere::Sphere;
    use crate::vector::{Point, Vector};
    use assert_float_eq::assert_f64_near;

    #[test]
    fn test_intersect_ray() {
        {
            let intersections = Sphere::new().intersect(&Ray::new(
                Point::new(0.0, 0.0, -5.0),
                Vector::new(0.0, 0.0, 1.0),
            ));

            assert_eq!(2, intersections.len());
            assert_f64_near!(4.0, intersections[0]);
            assert_f64_near!(6.0, intersections[1]);
        }

        {
            let intersections = Sphere::new().intersect(&Ray::new(
                Point::new(0.0, 1.0, -5.0),
                Vector::new(0.0, 0.0, 1.0),
            ));

            assert_eq!(2, intersections.len());
            assert_f64_near!(5.0, intersections[0]);
            assert_f64_near!(5.0, intersections[1]);
        }

        {
            let intersections = Sphere::new().intersect(&Ray::new(
                Point::new(0.0, 2.0, -5.0),
                Vector::new(0.0, 0.0, 1.0),
            ));

            assert!(intersections.is_empty());
        }

        {
            let intersections = Sphere::new().intersect(&Ray::new(
                Point::new(0.0, 0.0, 0.0),
                Vector::new(0.0, 0.0, 1.0),
            ));

            assert_eq!(2, intersections.len());
            assert_f64_near!(-1.0, intersections[0]);
            assert_f64_near!(1.0, intersections[1]);
        }
    }
}
