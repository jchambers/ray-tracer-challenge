use clap::Parser;
use png::EncodingError;
use ray_tracer_challenge::canvas::Canvas;
use ray_tracer_challenge::color::Color;
use ray_tracer_challenge::geometry::intersection;
use ray_tracer_challenge::geometry::ray::{IntersectRay, Ray};
use ray_tracer_challenge::geometry::sphere::Sphere;
use ray_tracer_challenge::light::PointLight;
use ray_tracer_challenge::material::Material;
use ray_tracer_challenge::transform::Transformation;
use ray_tracer_challenge::vector::Point;
use ray_tracer_challenge::{color, transform};
use std::fs::File;

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

fn main() -> Result<(), EncodingError> {
    let args = Args::parse();

    let sphere = Sphere::new(
        transform::transform(&[Transformation::Translate(
            BACKDROP_WIDTH / 2.0,
            BACKDROP_HEIGHT / 2.0,
            0.0,
        )]),
        Material::new(Color::new(1.0, 0.2, 1.0), 0.1, 0.9, 0.9, 20.0),
    );

    let camera = Point::new(BACKDROP_WIDTH / 2.0, BACKDROP_HEIGHT / 2.0, CAMERA_Z);
    let mut canvas = Canvas::new(args.size, args.size);

    let light = PointLight::new(Point::new(0.0, 0.0, CAMERA_Z * 2.0), color::WHITE);

    for x_canvas in 0..canvas.width() {
        for y_canvas in 0..canvas.height() {
            let x_world = (x_canvas as f64 / canvas.width() as f64) * BACKDROP_WIDTH;
            let y_world = (y_canvas as f64 / canvas.height() as f64) * BACKDROP_HEIGHT;

            let target = Point::new(x_world, y_world, BACKDROP_Z);
            let camera_to_target = (&target - &camera).normalize();
            let ray = Ray::new(camera, camera_to_target);

            let color = if let Some(intersection) = intersection::hit(&sphere.intersect(&ray)) {
                let position = ray.position(intersection.distance());

                sphere.material().lighting(
                    &light,
                    &position,
                    &-camera_to_target,
                    &intersection.sphere().normal_at(&position),
                )
            } else {
                color::BLACK
            };

            canvas.set_pixel(x_canvas, y_canvas, color);
        }
    }

    canvas.write_as_png(File::create(args.out)?, args.size as u32, args.size as u32)
}
