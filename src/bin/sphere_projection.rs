use std::{fs, io};
use clap::Parser;
use ray_tracer_challenge::canvas::Canvas;
use ray_tracer_challenge::color::Color;
use ray_tracer_challenge::geometry::ray::{IntersectRay, Ray};
use ray_tracer_challenge::geometry::sphere::Sphere;
use ray_tracer_challenge::transform;
use ray_tracer_challenge::transform::Transformation;
use ray_tracer_challenge::vector::Point;

const BACKDROP_WIDTH: f64 = 8.0;
const BACKDROP_HEIGHT: f64 = 8.0;

const CAMERA_Z: f64 = -3.0;
const BACKDROP_Z: f64 = 3.0;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    out: String,

    #[arg(short, long, default_value_t = 1024)]
    size: usize,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let sphere = Sphere::new(transform::transform(&[Transformation::Translate(
        BACKDROP_WIDTH / 2.0,
        BACKDROP_HEIGHT / 2.0,
        0.0,
    )]));

    let camera = Point::new(BACKDROP_WIDTH / 2.0, BACKDROP_HEIGHT / 2.0, CAMERA_Z);

    let backdrop_color = Color::new(0.0, 0.0, 0.0);
    let hit_color = Color::new(1.0, 1.0, 1.0);

    let mut canvas = Canvas::new(args.size, args.size);

    for x_canvas in 0..canvas.width() {
        for y_canvas in 0..canvas.height() {
            let x_world = (x_canvas as f64 / canvas.width() as f64) * BACKDROP_WIDTH;
            let y_world = (y_canvas as f64 / canvas.height() as f64) * BACKDROP_HEIGHT;

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

    fs::write(args.out, format!{"{canvas}"})
}
