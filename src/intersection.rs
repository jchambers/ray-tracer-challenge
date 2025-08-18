use crate::shape::Shape;

pub struct Intersection<'a> {
    distance: f64,
    shape: &'a dyn Shape,
}

impl<'a> Intersection<'a> {
    pub fn new(distance: f64, shape: &'a dyn Shape) -> Self {
        Intersection { distance, shape }
    }

    pub fn distance(&self) -> f64 {
        self.distance
    }

    pub fn shape(&self) -> &dyn Shape {
        self.shape
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
    use crate::intersection::{Intersection, hit};
    use crate::shape::sphere::Sphere;

    #[test]
    fn test_hit() {
        let shape = Sphere::default();

        {
            let intersections = vec![
                Intersection::new(1.0, &shape),
                Intersection::new(2.0, &shape),
            ];

            assert_eq!(1.0, hit(&intersections).unwrap().distance());
        }

        {
            let intersections = vec![
                Intersection::new(-1.0, &shape),
                Intersection::new(1.0, &shape),
            ];

            assert_eq!(1.0, hit(&intersections).unwrap().distance());
        }

        {
            let intersections = vec![
                Intersection::new(-2.0, &shape),
                Intersection::new(-1.0, &shape),
            ];

            assert!(hit(&intersections).is_none());
        }

        {
            let intersections = vec![
                Intersection::new(5.0, &shape),
                Intersection::new(7.0, &shape),
                Intersection::new(-3.0, &shape),
                Intersection::new(2.0, &shape),
            ];

            assert_eq!(2.0, hit(&intersections).unwrap().distance());
        }
    }
}
