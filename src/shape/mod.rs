pub mod sphere;

use crate::geometry::intersection::Intersection;
use crate::geometry::ray::Ray;
use crate::material::Material;
use crate::matrix::Matrix;
use crate::vector::{Point, Vector};

pub trait Shape {
    fn inverse_transformation(&self) -> &Matrix<4>;

    fn material(&self) -> &Material;

    fn normal_at(&self, world_point: &Point) -> Vector;

    fn intersect(&self, world_ray: &Ray) -> Vec<Intersection>;
}
