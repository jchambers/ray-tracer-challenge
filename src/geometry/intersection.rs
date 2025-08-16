use crate::geometry::sphere::Sphere;

pub struct Intersection<'a> {
    distance: f64,
    sphere: &'a Sphere,
}

impl<'a> Intersection<'a> {
    pub fn new(distance: f64, sphere: &'a Sphere) -> Self {
        Intersection { distance, sphere }
    }

    pub fn distance(&self) -> f64 {
        self.distance
    }
}

pub fn hit<'s, 'i>(intersections: &'i [Intersection<'s>]) -> Option<&'i Intersection<'s>> {
    intersections
        .iter()
        .filter(|intersection| intersection.distance() >= 0.0)
        .min_by(|a, b| a.distance().partial_cmp(&b.distance()).unwrap())
}

#[cfg(test)]
mod test {
    use crate::geometry::intersection::{Intersection, hit};
    use crate::geometry::sphere::Sphere;

    #[test]
    fn test_hit() {
        let sphere = Sphere::default();

        {
            let intersections = vec![
                Intersection::new(1.0, &sphere),
                Intersection::new(2.0, &sphere),
            ];

            assert_eq!(1.0, hit(&intersections).unwrap().distance());
        }

        {
            let intersections = vec![
                Intersection::new(-1.0, &sphere),
                Intersection::new(1.0, &sphere),
            ];

            assert_eq!(1.0, hit(&intersections).unwrap().distance());
        }

        {
            let intersections = vec![
                Intersection::new(-2.0, &sphere),
                Intersection::new(-1.0, &sphere),
            ];

            assert!(hit(&intersections).is_none());
        }

        {
            let intersections = vec![
                Intersection::new(5.0, &sphere),
                Intersection::new(7.0, &sphere),
                Intersection::new(-3.0, &sphere),
                Intersection::new(2.0, &sphere),
            ];

            assert_eq!(2.0, hit(&intersections).unwrap().distance());
        }
    }
}
