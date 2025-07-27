use crate::matrix::Matrix;

pub fn translate(x: f64, y: f64, z: f64) -> Matrix<4> {
    Matrix::new([
        [1.0, 0.0, 0.0, x],
        [0.0, 1.0, 0.0, y],
        [0.0, 0.0, 1.0, z],
        [0.0, 0.0, 0.0, 1.0]
    ])
}

pub fn scale(x: f64, y: f64, z: f64) -> Matrix<4> {
    Matrix::new([
        [x, 0.0, 0.0, 0.0],
        [0.0, y, 0.0, 0.0],
        [0.0, 0.0, z, 0.0],
        [0.0, 0.0, 0.0, 1.0]
    ])
}

pub fn rotate_x(radians: f64) -> Matrix<4> {
    let (sin, cos) = radians.sin_cos();

    Matrix::new([
        [1.0, 0.0, 0.0, 0.0],
        [0.0, cos, -sin, 0.0],
        [0.0, sin, cos, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

pub fn rotate_y(radians: f64) -> Matrix<4> {
    let (sin, cos) = radians.sin_cos();

    Matrix::new([
        [cos, 0.0, sin, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [-sin, 0.0, cos, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

pub fn rotate_z(radians: f64) -> Matrix<4> {
    let (sin, cos) = radians.sin_cos();

    Matrix::new([
        [cos, -sin, 0.0, 0.0],
        [sin, cos, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

pub fn shear(x_y: f64, x_z: f64, y_x: f64, y_z: f64, z_x: f64, z_y: f64) -> Matrix<4> {
    Matrix::new([
        [1.0, x_y, x_z, 0.0],
        [y_x, 1.0, y_z, 0.0],
        [z_x, z_y, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

#[cfg(test)]
mod test {
    use crate::transform::{rotate_x, rotate_y, rotate_z, scale, shear, translate};
    use crate::vector::{Point, Vector};

    #[test]
    fn test_translate() {
        let translation = translate(5.0, -3.0, 2.0);

        Point::new(2.0, 1.0, 7.0)
            .assert_approx_eq(&(&translation * &Point::new(-3.0, 4.0, 5.0)));

        Vector::new(-3.0, 4.0, 5.0)
            .assert_appeox_eq(&(&translation * &Vector::new(-3.0, 4.0, 5.0)));
    }

    #[test]
    fn test_scale() {
        let scale = scale(2.0, 3.0, 4.0);

        Point::new(-8.0, 18.0, 32.0)
            .assert_approx_eq(&(&scale * &Point::new(-4.0, 6.0, 8.0)));

        Vector::new(-8.0, 18.0, 32.0)
            .assert_appeox_eq(&(&scale * &Vector::new(-4.0, 6.0, 8.0)));
    }

    #[test]
    fn test_rotate_x() {
        let point = Point::new(0.0, 1.0, 0.0);

        Point::new(0.0, 2.0f64.sqrt() / 2.0, 2.0f64.sqrt() / 2.0)
            .assert_approx_eq_epsilon(&(&rotate_x(std::f64::consts::PI / 4.0) * &point), 1e-6);

        Point::new(0.0, 0.0, 1.0)
            .assert_approx_eq_epsilon(&(&rotate_x(std::f64::consts::PI / 2.0) * &point), 1e-6);
    }

    #[test]
    fn test_rotate_y() {
        let point = Point::new(0.0, 0.0, 1.0);

        Point::new(2.0f64.sqrt() / 2.0, 0.0, 2.0f64.sqrt() / 2.0)
            .assert_approx_eq_epsilon(&(&rotate_y(std::f64::consts::PI / 4.0) * &point), 1e-6);

        Point::new(1.0, 0.0, 0.0)
            .assert_approx_eq_epsilon(&(&rotate_y(std::f64::consts::PI / 2.0) * &point), 1e-6);
    }

    #[test]
    fn test_rotate_z() {
        let point = Point::new(0.0, 1.0, 0.0);

        Point::new(-2.0f64.sqrt() / 2.0, 2.0f64.sqrt() / 2.0, 0.0)
            .assert_approx_eq_epsilon(&(&rotate_z(std::f64::consts::PI / 4.0) * &point), 1e-6);

        Point::new(-1.0, 0.0, 0.0)
            .assert_approx_eq_epsilon(&(&rotate_z(std::f64::consts::PI / 2.0) * &point), 1e-6);
    }

    #[test]
    fn test_shear() {
        let point = Point::new(2.0, 3.0, 4.0);

        Point::new(6.0, 3.0, 4.0)
            .assert_approx_eq(&(&shear(0.0, 1.0, 0.0, 0.0, 0.0, 0.0) * &point))
    }
}
