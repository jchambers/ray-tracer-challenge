use crate::matrix::Matrix;

pub enum Transformation {
    Translate(f64, f64, f64),
    Scale(f64, f64, f64),
    RotateX(f64),
    RotateY(f64),
    RotateZ(f64),
    Shear(f64, f64, f64, f64, f64, f64),
}

impl From<&Transformation> for Matrix<4> {
    fn from(transformation: &Transformation) -> Self {
        match transformation {
            Transformation::Translate(x, y, z) => Matrix::new([
                [1.0, 0.0, 0.0, *x],
                [0.0, 1.0, 0.0, *y],
                [0.0, 0.0, 1.0, *z],
                [0.0, 0.0, 0.0, 1.0],
            ]),

            Transformation::Scale(x, y, z) => Matrix::new([
                [*x, 0.0, 0.0, 0.0],
                [0.0, *y, 0.0, 0.0],
                [0.0, 0.0, *z, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ]),

            Transformation::RotateX(radians) => {
                let (sin, cos) = radians.sin_cos();

                Matrix::new([
                    [1.0, 0.0, 0.0, 0.0],
                    [0.0, cos, -sin, 0.0],
                    [0.0, sin, cos, 0.0],
                    [0.0, 0.0, 0.0, 1.0],
                ])
            }

            Transformation::RotateY(radians) => {
                let (sin, cos) = radians.sin_cos();

                Matrix::new([
                    [cos, 0.0, sin, 0.0],
                    [0.0, 1.0, 0.0, 0.0],
                    [-sin, 0.0, cos, 0.0],
                    [0.0, 0.0, 0.0, 1.0],
                ])
            }

            Transformation::RotateZ(radians) => {
                let (sin, cos) = radians.sin_cos();

                Matrix::new([
                    [cos, -sin, 0.0, 0.0],
                    [sin, cos, 0.0, 0.0],
                    [0.0, 0.0, 1.0, 0.0],
                    [0.0, 0.0, 0.0, 1.0],
                ])
            }

            Transformation::Shear(x_y, x_z, y_x, y_z, z_x, z_y) => Matrix::new([
                [1.0, *x_y, *x_z, 0.0],
                [*y_x, 1.0, *y_z, 0.0],
                [*z_x, *z_y, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ]),
        }
    }
}

pub fn transform(transformations: &[Transformation]) -> Matrix<4> {
    let mut transformation_matrix = Matrix::<4>::identity();

    for transformation in transformations.iter().rev() {
        transformation_matrix = transformation_matrix * Into::<Matrix<4>>::into(transformation);
    }

    transformation_matrix
}

#[cfg(test)]
mod test {
    use crate::matrix::Matrix;
    use crate::transform::{Transformation, transform};
    use crate::vector::{Point, Vector};

    #[test]
    fn test_translate() {
        let translation = transform(&[Transformation::Translate(5.0, -3.0, 2.0)]);

        Point::new(2.0, 1.0, 7.0).assert_approx_eq(&(&translation * &Point::new(-3.0, 4.0, 5.0)));

        Vector::new(-3.0, 4.0, 5.0)
            .assert_approx_eq(&(&translation * &Vector::new(-3.0, 4.0, 5.0)));
    }

    #[test]
    fn test_scale() {
        let scale = transform(&[Transformation::Scale(2.0, 3.0, 4.0)]);

        Point::new(-8.0, 18.0, 32.0).assert_approx_eq(&(&scale * &Point::new(-4.0, 6.0, 8.0)));

        Vector::new(-8.0, 18.0, 32.0).assert_approx_eq(&(&scale * &Vector::new(-4.0, 6.0, 8.0)));
    }

    #[test]
    fn test_rotate_x() {
        let point = Point::new(0.0, 1.0, 0.0);

        Point::new(0.0, 2.0f64.sqrt() / 2.0, 2.0f64.sqrt() / 2.0).assert_approx_eq_epsilon(
            &(&transform(&[Transformation::RotateX(std::f64::consts::PI / 4.0)]) * &point),
            1e-6,
        );

        Point::new(0.0, 0.0, 1.0).assert_approx_eq_epsilon(
            &(&transform(&[Transformation::RotateX(std::f64::consts::PI / 2.0)]) * &point),
            1e-6,
        );
    }

    #[test]
    fn test_rotate_y() {
        let point = Point::new(0.0, 0.0, 1.0);

        Point::new(2.0f64.sqrt() / 2.0, 0.0, 2.0f64.sqrt() / 2.0).assert_approx_eq_epsilon(
            &(&transform(&[Transformation::RotateY(std::f64::consts::PI / 4.0)]) * &point),
            1e-6,
        );

        Point::new(1.0, 0.0, 0.0).assert_approx_eq_epsilon(
            &(&transform(&[Transformation::RotateY(std::f64::consts::PI / 2.0)]) * &point),
            1e-6,
        );
    }

    #[test]
    fn test_rotate_z() {
        let point = Point::new(0.0, 1.0, 0.0);

        Point::new(-2.0f64.sqrt() / 2.0, 2.0f64.sqrt() / 2.0, 0.0).assert_approx_eq_epsilon(
            &(&transform(&[Transformation::RotateZ(std::f64::consts::PI / 4.0)]) * &point),
            1e-6,
        );

        Point::new(-1.0, 0.0, 0.0).assert_approx_eq_epsilon(
            &(&transform(&[Transformation::RotateZ(std::f64::consts::PI / 2.0)]) * &point),
            1e-6,
        );
    }

    #[test]
    fn test_shear() {
        let point = Point::new(2.0, 3.0, 4.0);

        Point::new(6.0, 3.0, 4.0).assert_approx_eq(
            &(&transform(&[Transformation::Shear(0.0, 1.0, 0.0, 0.0, 0.0, 0.0)]) * &point),
        );

        Point::new(2.0, 5.0, 4.0).assert_approx_eq(
            &(&transform(&[Transformation::Shear(0.0, 0.0, 1.0, 0.0, 0.0, 0.0)]) * &point),
        );

        Point::new(2.0, 7.0, 4.0).assert_approx_eq(
            &(&transform(&[Transformation::Shear(0.0, 0.0, 0.0, 1.0, 0.0, 0.0)]) * &point),
        );

        Point::new(2.0, 3.0, 6.0).assert_approx_eq(
            &(&transform(&[Transformation::Shear(0.0, 0.0, 0.0, 0.0, 1.0, 0.0)]) * &point),
        );
    }

    #[test]
    fn test_chain_transform() {
        let point = Point::new(1.0, 0.0, 1.0);

        let rotation = Transformation::RotateX(std::f64::consts::PI / 2.0);
        let scale = Transformation::Scale(5.0, 5.0, 5.0);
        let translation = Transformation::Translate(10.0, 5.0, 7.0);

        let sequential = Into::<Matrix<4>>::into(&rotation) * &point;
        let sequential = Into::<Matrix<4>>::into(&scale) * &sequential;
        let sequential = Into::<Matrix<4>>::into(&translation) * &sequential;

        let combined = transform(&[rotation, scale, translation]) * &point;

        sequential.assert_approx_eq(&combined);
    }
}
