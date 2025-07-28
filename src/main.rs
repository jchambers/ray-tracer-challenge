use crate::canvas::Canvas;
use crate::color::Color;
use crate::transform::Transformation;
use crate::vector::Point;

mod canvas;
mod color;
mod matrix;
mod vector;
mod transform;

fn main() {
    let mut canvas = Canvas::new(128, 128);
    let white = Color::new(1.0, 1.0, 1.0);

    for hour in 0..12 {
        let transformation = transform::transform(&[
            Transformation::RotateZ((std::f64::consts::PI / 6.0) * hour as f64),
            Transformation::Scale(60.0, 60.0, 1.0),
            Transformation::Translate(64.0, 64.0, 0.0)
        ]);

        let (x, y, _) = (transformation * &Point::new(0.0, 1.0, 0.0)).coordinates();

        canvas.set_pixel(x as usize, y as usize, white);
    }

    println!("{canvas}");
}
