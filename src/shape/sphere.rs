use crate::intersection::Intersection;
use crate::material::Material;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::shape::Shape;
use crate::transform;
use crate::transform::Transformation;
use crate::vector::{ORIGIN, Point, Vector};

pub struct Sphere {
    inverse_transformation: Matrix<4>,
    material: Material,
}

impl Sphere {
    pub fn with_transformations(transformations: &[Transformation], material: Material) -> Self {
        Sphere {
            inverse_transformation: transform::transform(transformations).inverse().unwrap(),
            material,
        }
    }
}

impl Shape for Sphere {
    fn inverse_transformation(&self) -> &Matrix<4> {
        &self.inverse_transformation
    }

    fn material(&self) -> &Material {
        &self.material
    }

    fn normal_at(&self, world_point: &Point) -> Vector {
        let object_point = &self.inverse_transformation * world_point;
        let object_normal = object_point - &ORIGIN;

        let mut world_normal_components =
            &self.inverse_transformation.transpose() * object_normal.components();

        // TODO Possible optimization opportunity: invert a 3x3 matrix instead?
        world_normal_components[3] = 0.0;

        Vector::from(world_normal_components).normalize()
    }

    fn intersect(&self, world_ray: &Ray) -> Vec<Intersection> {
        let ray = &self.inverse_transformation * world_ray;
        let sphere_to_ray = ray.origin() - &ORIGIN;

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

impl Default for Sphere {
    fn default() -> Self {
        Sphere {
            inverse_transformation: Matrix::<4>::identity(),
            material: Material::default(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::material::Material;
    use crate::ray::Ray;
    use crate::shape::Shape;
    use crate::shape::sphere::Sphere;
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
            let sphere = Sphere::with_transformations(
                &[Transformation::Scale(2.0, 2.0, 2.0)],
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
            let sphere = Sphere::with_transformations(
                &[Transformation::Translate(5.0, 0.0, 0.0)],
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
            let translated_sphere = Sphere::with_transformations(
                &[Transformation::Translate(0.0, 1.0, 0.0)],
                Material::default(),
            );

            Vector::new(0.0, sqrt_2_2, -sqrt_2_2).assert_approx_eq(
                &translated_sphere.normal_at(&Point::new(0.0, 1.0 + sqrt_2_2, -sqrt_2_2)),
            );
        }

        {
            let translated_sphere = Sphere::with_transformations(
                &[
                    Transformation::RotateZ(std::f64::consts::PI / 5.0),
                    Transformation::Scale(1.0, 0.5, 1.0),
                ],
                Material::default(),
            );

            let normal = translated_sphere.normal_at(&Point::new(0.0, sqrt_2_2, -sqrt_2_2));

            assert_f64_near!(0.0, normal.components()[0]);
            assert_float_absolute_eq!(0.97014, normal.components()[1], 1e-5);
            assert_float_absolute_eq!(-0.24254, normal.components()[2], 1e-5);
        }
    }
}
