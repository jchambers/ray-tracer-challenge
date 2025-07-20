use crate::vector::{Point, Vector};
use std::ops::{Index, Mul};

#[cfg(test)]
use assert_float_eq::assert_float_absolute_eq;

pub struct Matrix<const N: usize> {
    elements: [[f64; N]; N],
}

impl<const N: usize> Matrix<N> {
    pub fn new(elements: [[f64; N]; N]) -> Self {
        Matrix { elements }
    }

    pub fn identity() -> Self {
        let mut identity = Matrix::new([[0.0; N]; N]);

        for i in 0..N {
            identity.elements[i][i] = 1.0;
        }

        identity
    }

    pub fn transpose(&self) -> Self {
        let mut transposed = Matrix::new([[0.0; N]; N]);

        for m in 0..N {
            for n in 0..N {
                transposed.elements[n][m] = self.elements[m][n]
            }
        }

        transposed
    }

    #[cfg(test)]
    pub fn assert_approx_eq(&self, other: &Matrix<N>, epsilon: f64) {
        for m in 0..N {
            for n in 0..N {
                assert_float_absolute_eq!(self.elements[m][n], other.elements[m][n], epsilon);
            }
        }
    }
}

impl Matrix<4> {
    // TODO More reasonable error type
    pub fn inverse(&self) -> Result<Matrix<4>, ()> {
        let determinant = self.determinant();

        if determinant == 0.0 {
            // Matrix is not invertible
            return Err(());
        }

        let mut inverse = Matrix::new([[0.0; 4]; 4]);

        for row in 0..4 {
            for col in 0..4 {
                inverse.elements[col][row] = self.cofactor(row, col) / determinant
            }
        }

        Ok(inverse)
    }

    fn determinant(&self) -> f64 {
        (0..4)
            .map(|i| self.elements[0][i] * self.cofactor(0, i))
            .sum()
    }

    fn cofactor(&self, row: usize, col: usize) -> f64 {
        let minor = self.submatrix(row, col).determinant();

        if (row + col) % 2 == 0 { minor } else { -minor }
    }

    fn submatrix(&self, removed_row: usize, removed_col: usize) -> Matrix<3> {
        let mut submatrix = Matrix::new([[0.0; 3]; 3]);

        let rows = Self::remaining_indices(removed_row);
        let cols = Self::remaining_indices(removed_col);

        for row in 0..3 {
            for col in 0..3 {
                submatrix.elements[row][col] = self.elements[rows[row]][cols[col]];
            }
        }

        submatrix
    }

    fn remaining_indices(n: usize) -> [usize; 3] {
        match n {
            0 => [1, 2, 3],
            1 => [0, 2, 3],
            2 => [0, 1, 3],
            3 => [0, 1, 2],
            _ => panic!("Index out of bounds: {}", n),
        }
    }
}

impl Matrix<3> {
    pub fn determinant(&self) -> f64 {
        (0..3)
            .map(|i| self.elements[0][i] * self.cofactor(0, i))
            .sum()
    }

    fn cofactor(&self, row: usize, col: usize) -> f64 {
        let rows = Self::remaining_indices(row);
        let cols = Self::remaining_indices(col);

        let minor = (self.elements[rows[0]][cols[0]] * self.elements[rows[1]][cols[1]])
            - (self.elements[rows[0]][cols[1]] * self.elements[rows[1]][cols[0]]);

        if (row + col) % 2 == 0 { minor } else { -minor }
    }

    fn remaining_indices(n: usize) -> [usize; 2] {
        match n {
            0 => [1, 2],
            1 => [0, 2],
            2 => [0, 1],
            _ => panic!("Index out of bounds: {}", n),
        }
    }
}

impl<const N: usize> Index<(usize, usize)> for Matrix<N> {
    type Output = f64;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.elements[index.0][index.1]
    }
}

impl<const N: usize> Mul<&Matrix<N>> for Matrix<N> {
    type Output = Self;

    fn mul(self, rhs: &Matrix<N>) -> Self::Output {
        &self * rhs
    }
}

impl<const N: usize> Mul<&Matrix<N>> for &Matrix<N> {
    type Output = Matrix<N>;

    fn mul(self, rhs: &Matrix<N>) -> Self::Output {
        let mut product = Matrix::new([[0.0; N]; N]);
        let rhs = rhs.transpose();

        for m in 0..N {
            for n in 0..N {
                product.elements[m][n] = self.elements[m]
                    .iter()
                    .zip(rhs.elements[n].iter())
                    .map(|(a, b)| a * b)
                    .sum()
            }
        }

        product
    }
}

impl Mul<&Point> for Matrix<4> {
    type Output = Point;

    fn mul(self, rhs: &Point) -> Self::Output {
        let mut components = [0.0; 4];

        for m in 0..4 {
            components[m] = self.elements[m]
                .iter()
                .zip(rhs.components().iter())
                .map(|(a, b)| a * b)
                .sum();
        }

        Point::from(components)
    }
}

impl Mul<&Vector> for Matrix<4> {
    type Output = Vector;

    fn mul(self, rhs: &Vector) -> Self::Output {
        let mut components = [0.0; 4];

        for m in 0..4 {
            components[m] = self.elements[m]
                .iter()
                .zip(rhs.components().iter())
                .map(|(a, b)| a * b)
                .sum();
        }

        Vector::from(components)
    }
}

#[cfg(test)]
mod test {
    use crate::matrix::Matrix;
    use crate::vector::Point;
    use assert_float_eq::assert_f64_near;

    #[test]
    fn test_index() {
        let matrix = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5],
        ]);

        assert_f64_near!(1.0, matrix[(0, 0)]);
        assert_f64_near!(7.5, matrix[(1, 2)]);
        assert_f64_near!(13.5, matrix[(3, 0)]);
    }

    #[test]
    fn test_mul_matrix() {
        let a = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);

        let b = Matrix::new([
            [-2.0, 1.0, 2.0, 3.0],
            [3.0, 2.0, 1.0, -1.0],
            [4.0, 3.0, 6.0, 5.0],
            [1.0, 2.0, 7.0, 8.0],
        ]);

        Matrix::new([
            [20.0, 22.0, 50.0, 48.0],
            [44.0, 54.0, 114.0, 108.0],
            [40.0, 58.0, 110.0, 102.0],
            [16.0, 26.0, 46.0, 42.0],
        ]).assert_approx_eq(&(a * &b), 1e-16)
    }

    #[test]
    fn test_transpose() {
        Matrix::new([
            [0.0, 9.0, 1.0, 0.0],
            [9.0, 8.0, 8.0, 0.0],
            [3.0, 0.0, 5.0, 5.0],
            [0.0, 8.0, 3.0, 8.0],
        ]).assert_approx_eq(&Matrix::new([
            [0.0, 9.0, 3.0, 0.0],
            [9.0, 8.0, 0.0, 8.0],
            [1.0, 8.0, 5.0, 3.0],
            [0.0, 0.0, 5.0, 8.0],
        ])
            .transpose(), 0.0);
    }

    #[test]
    fn test_mul_point() {
        let point = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        ]) * &Point::new(1.0, 2.0, 3.0);

        let expected_components = [18.0, 24.0, 33.0, 1.0];

        for i in 0..4 {
            assert_f64_near!(expected_components[i], point.components()[i]);
        }
    }

    #[test]
    fn test_identity() {
        let original = Matrix::new([
            [0.0, 1.0, 2.0, 4.0],
            [1.0, 2.0, 4.0, 8.0],
            [2.0, 4.0, 8.0, 16.0],
            [4.0, 8.0, 16.0, 32.0],
        ]);

        original.assert_approx_eq(&(&original * &Matrix::<4>::identity()), 1e-16);
    }

    #[test]
    fn test_cofactor_3() {
        let matrix = Matrix::new([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);

        assert_f64_near!(-12.0, matrix.cofactor(0, 0));
        assert_f64_near!(-25.0, matrix.cofactor(1, 0));
    }

    #[test]
    fn test_determinant_3() {
        assert_f64_near!(
            -196.0,
            Matrix::new([[1.0, 2.0, 6.0], [-5.0, 8.0, -4.0], [2.0, 6.0, 4.0],]).determinant()
        )
    }

    #[test]
    fn test_submatrix_4() {
        Matrix::new([
            [-6.0, 1.0, 6.0],
            [-8.0, 8.0, 6.0],
            [-7.0, -1.0, 1.0]]
        ).assert_approx_eq(&Matrix::new([
            [-6.0, 1.0, 1.0, 6.0],
            [-8.0, 5.0, 8.0, 6.0],
            [-1.0, 0.0, 8.0, 2.0],
            [-7.0, 1.0, -1.0, 1.0],
        ])
            .submatrix(2, 1), 0.0);
    }

    #[test]
    fn test_determinant_4() {
        let matrix = Matrix::new([
            [-2.0, -8.0, 3.0, 5.0],
            [-3.0, 1.0, 7.0, 3.0],
            [1.0, 2.0, -9.0, 6.0],
            [-6.0, 7.0, 7.0, -9.0],
        ]);

        assert_f64_near!(690.0, matrix.cofactor(0, 0));
        assert_f64_near!(447.0, matrix.cofactor(0, 1));
        assert_f64_near!(210.0, matrix.cofactor(0, 2));
        assert_f64_near!(51.0, matrix.cofactor(0, 3));

        assert_f64_near!(-4071.0, matrix.determinant());
    }

    #[test]
    fn test_inverse_4() {
        assert!(
            Matrix::new([
                [-4.0, 2.0, -2.0, -3.0],
                [9.0, 6.0, 2.0, 6.0],
                [0.0, -5.0, 1.0, -5.0],
                [0.0, 0.0, 0.0, 0.0],
            ])
            .inverse()
            .is_err()
        );

        let original = Matrix::new([
            [-5.0, 2.0, 6.0, -8.0],
            [1.0, -5.0, 1.0, 8.0],
            [7.0, 7.0, -6.0, -7.0],
            [1.0, -3.0, 7.0, 4.0],
        ]);

        let inverse = original.inverse().unwrap();

        assert_f64_near!(532.0, original.determinant());
        assert_f64_near!(-160.0, original.cofactor(2, 3));
        assert_f64_near!(-160.0 / 532.0, inverse.elements[3][2]);
        assert_f64_near!(105.0, original.cofactor(3, 2));
        assert_f64_near!(105.0 / 532.0, inverse.elements[2][3]);
        assert_f64_near!(128.0, original.cofactor(2, 0));

        Matrix::<4>::identity()
            .assert_approx_eq(&(&original * &original.inverse().unwrap()), 1e-15);
    }
}
