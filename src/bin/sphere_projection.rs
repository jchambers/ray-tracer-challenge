use ray_tracer_challenge::canvas::Canvas;
use ray_tracer_challenge::color::Color;
use ray_tracer_challenge::geometry::ray::{IntersectRay, Ray};
use ray_tracer_challenge::geometry::sphere::Sphere;
use ray_tracer_challenge::transform;
use ray_tracer_challenge::transform::Transformation;
use ray_tracer_challenge::vector::Point;

const CANVAS_WIDTH: usize = 1024;
const CANVAS_HEIGHT: usize = 1024;

const BACKDROP_WIDTH: f64 = 8.0;
const BACKDROP_HEIGHT: f64 = 8.0;

const CAMERA_Z: f64 = -3.0;
const BACKDROP_Z: f64 = 3.0;

fn main() {
    let sphere = Sphere::new(transform::transform(&[Transformation::Translate(
        BACKDROP_WIDTH / 2.0,
        BACKDROP_HEIGHT / 2.0,
        0.0,
    )]));
    let camera = Point::new(BACKDROP_WIDTH / 2.0, BACKDROP_HEIGHT / 2.0, CAMERA_Z);

    let backdrop_color = Color::new(0.0, 0.0, 0.0);
    let hit_color = Color::new(1.0, 1.0, 1.0);

    let mut canvas = Canvas::new(CANVAS_WIDTH, CANVAS_HEIGHT);

    for x_canvas in 0..CANVAS_WIDTH {
        for y_canvas in 0..CANVAS_HEIGHT {
            let x_world = (x_canvas as f64 / CANVAS_WIDTH as f64) * BACKDROP_WIDTH;
            let y_world = (y_canvas as f64 / CANVAS_HEIGHT as f64) * BACKDROP_HEIGHT;

            let target = Point::new(x_world, y_world, BACKDROP_Z);
            let camera_to_target = &target - &camera;

            let intersections = sphere.intersect(&Ray::new(camera, camera_to_target));

            let color = if intersections.is_empty() {
                backdrop_color
            } else {
                hit_color
            };

            canvas.set_pixel(x_canvas, y_canvas, color);
        }
    }

    println!("{canvas}");
}
