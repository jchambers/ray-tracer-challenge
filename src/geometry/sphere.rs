use crate::geometry::intersection::Intersection;
use crate::geometry::ray::{IntersectRay, Ray};
use crate::material::Material;
use crate::matrix::Matrix;
use crate::vector::{ORIGIN, Point, Vector};

pub struct Sphere {
    transformation: Matrix<4>,
    material: Material,
}

impl Default for Sphere {
    fn default() -> Self {
        Sphere {
            transformation: Matrix::<4>::identity(),
            material: Material::default(),
        }
    }
}

impl Sphere {
    pub fn new(transformation: Matrix<4>, material: Material) -> Self {
        Sphere {
            transformation,
            material,
        }
    }

    pub fn material(&self) -> &Material {
        &self.material
    }

    pub fn normal_at(&self, world_point: &Point) -> Vector {
        let transform_inverse = self.transformation.inverse().unwrap();

        let object_point = &transform_inverse * world_point;
        let object_normal = object_point - &ORIGIN;

        let mut world_normal_components =
            &transform_inverse.transpose() * object_normal.components();
        world_normal_components[3] = 0.0;

        Vector::from(world_normal_components).normalize()
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
    use crate::material::Material;
    use crate::transform;
    use crate::transform::Transformation;
    use crate::vector::{Point, Vector};
    use assert_float_eq::{assert_f64_near, assert_float_absolute_eq};

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
            let sphere = Sphere::new(
                transform::transform(&[Transformation::Scale(2.0, 2.0, 2.0)]),
                Material::default(),
            );

            let intersections = sphere.intersect(&Ray::new(
                Point::new(0.0, 0.0, -5.0),
                Vector::new(0.0, 0.0, 1.0),
            ));

            assert_eq!(2, intersections.len());
            assert_f64_near!(3.0, intersections[0].distance());
            assert_f64_near!(7.0, intersections[1].distance());
        }

        {
            let sphere = Sphere::new(
                transform::transform(&[Transformation::Translate(5.0, 0.0, 0.0)]),
                Material::default(),
            );

            let intersections = sphere.intersect(&Ray::new(
                Point::new(0.0, 0.0, -5.0),
                Vector::new(0.0, 0.0, 1.0),
            ));

            assert!(intersections.is_empty());
        }
    }

    #[test]
    fn test_normal_at() {
        let unit_sphere = Sphere::default();

        Vector::new(1.0, 0.0, 0.0)
            .assert_approx_eq(&unit_sphere.normal_at(&Point::new(1.0, 0.0, 0.0)));

        Vector::new(0.0, 1.0, 0.0)
            .assert_approx_eq(&unit_sphere.normal_at(&Point::new(0.0, 1.0, 0.0)));

        Vector::new(0.0, 0.0, 1.0)
            .assert_approx_eq(&unit_sphere.normal_at(&Point::new(0.0, 0.0, 1.0)));

        let sqrt_3_3 = 3.0f64.sqrt() / 3.0;

        Vector::new(sqrt_3_3, sqrt_3_3, sqrt_3_3)
            .assert_approx_eq(&unit_sphere.normal_at(&Point::new(sqrt_3_3, sqrt_3_3, sqrt_3_3)));
    }

    #[test]
    fn test_normal_at_transformed() {
        let sqrt_2_2 = 2.0f64.sqrt() / 2.0;

        {
            let translated_sphere = Sphere::new(
                transform::transform(&[Transformation::Translate(0.0, 1.0, 0.0)]),
                Material::default(),
            );

            Vector::new(0.0, sqrt_2_2, -sqrt_2_2).assert_approx_eq(
                &translated_sphere.normal_at(&Point::new(0.0, 1.0 + sqrt_2_2, -sqrt_2_2)),
            );
        }

        {
            let translated_sphere = Sphere::new(
                transform::transform(&[
                    Transformation::RotateZ(std::f64::consts::PI / 5.0),
                    Transformation::Scale(1.0, 0.5, 1.0),
                ]),
                Material::default(),
            );

            let normal = translated_sphere.normal_at(&Point::new(0.0, sqrt_2_2, -sqrt_2_2));

            assert_f64_near!(0.0, normal.components()[0]);
            assert_float_absolute_eq!(0.97014, normal.components()[1], 1e-5);
            assert_float_absolute_eq!(-0.24254, normal.components()[2], 1e-5);
        }
    }
}
