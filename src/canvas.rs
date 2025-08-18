use crate::color;
use crate::color::Color;
use png::EncodingError;
use std::fs::File;
use std::io::BufWriter;
use std::iter;

pub struct Canvas {
    width: usize,
    pixels: Vec<Color>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            pixels: iter::repeat_n(color::BLACK, width * height).collect(),
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        let index = self.index(x, y);
        self.pixels[index] = color
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> &Color {
        &self.pixels[self.index(x, y)]
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.pixels.len() / self.width
    }

    fn index(&self, x: usize, y: usize) -> usize {
        if x >= self.width || y >= self.height() {
            panic!(
                "Position ({}, {}) out of bounds (width = {}, height = {}",
                x,
                y,
                self.width,
                self.height()
            );
        }

        (self.width * y) + x
    }

    pub fn write_as_png(&self, file: File, width: u32, height: u32) -> Result<(), EncodingError> {
        let writer = BufWriter::new(file);
        let mut encoder = png::Encoder::new(writer, width, height);

        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);

        let mut writer = encoder.write_header()?;

        writer.write_image_data(&self.to_rgba())
    }

    fn to_rgba(&self) -> Vec<u8> {
        self.pixels
            .iter()
            .flat_map(|pixel| {
                [
                    (pixel.components()[0].clamp(0.0, 1.0) * 255.0).round() as u8,
                    (pixel.components()[1].clamp(0.0, 1.0) * 255.0).round() as u8,
                    (pixel.components()[2].clamp(0.0, 1.0) * 255.0).round() as u8,
                    255,
                ]
                .into_iter()
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use crate::canvas::Canvas;
    use crate::color::Color;

    #[test]
    fn test_dimensions() {
        let width = 17;
        let height = 19;

        let canvas = Canvas::new(width, height);

        assert_eq!(width, canvas.width());
        assert_eq!(height, canvas.height());
    }

    #[test]
    fn test_set_get_pixel() {
        let mut canvas = Canvas::new(1, 1);
        let red = Color::new(1.0, 0.0, 0.0);

        canvas.set_pixel(0, 0, red);

        assert_eq!(&red, canvas.get_pixel(0, 0));
    }

    #[test]
    fn test_to_rgba() {
        let mut canvas = Canvas::new(2, 2);
        canvas.set_pixel(0, 0, Color::new(1.5, 0.0, 0.0));
        canvas.set_pixel(0, 1, Color::new(0.0, 0.5, 0.0));
        canvas.set_pixel(1, 1, Color::new(-0.5, 0.0, 1.0));

        #[rustfmt::skip]
        assert_eq!(
            vec![255, 0,   0,   255,
                 0,   0,   0,   255,
                 0,   128, 0,   255,
                 0,   0,   255, 255],
            canvas.to_rgba());
    }
}
