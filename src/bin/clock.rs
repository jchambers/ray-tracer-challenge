use clap::Parser;
use ray_tracer_challenge::canvas::Canvas;
use ray_tracer_challenge::color::Color;
use ray_tracer_challenge::transform;
use ray_tracer_challenge::transform::Transformation;
use ray_tracer_challenge::vector::Point;
use std::{fs, io};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    out: String,

    #[arg(short, long, default_value_t = 128)]
    size: usize,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let mut canvas = Canvas::new(args.size, args.size);
    let white = Color::new(1.0, 1.0, 1.0);

    for hour in 0..12 {
        let transformation = transform::transform(&[
            Transformation::RotateZ((std::f64::consts::PI / 6.0) * hour as f64),
            Transformation::Scale(60.0, 60.0, 1.0),
            Transformation::Translate(64.0, 64.0, 0.0),
        ]);

        let (x, y, _) = (transformation * &Point::new(0.0, 1.0, 0.0)).coordinates();

        canvas.set_pixel(x as usize, y as usize, white);
    }


    fs::write(args.out, format!{"{canvas}"})
}
