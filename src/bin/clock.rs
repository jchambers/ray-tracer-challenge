use ray_tracer_challenge::canvas::Canvas;
use ray_tracer_challenge::color::Color;
use ray_tracer_challenge::transform;
use ray_tracer_challenge::transform::Transformation;
use ray_tracer_challenge::vector::Point;

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
