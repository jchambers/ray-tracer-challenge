use crate::color::Color;
use std::fmt::{Display, Formatter};
use std::iter;

pub struct Canvas {
    width: usize,
    pixels: Vec<Color>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            pixels: iter::repeat_with(Color::default)
                .take(width * height)
                .collect(),
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
}

impl Display for Canvas {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "P3")?;
        writeln!(f, "{} {}", self.width, self.height())?;
        writeln!(f, "255")?;

        // PPM files want a maximum width of 70 columns. If we allow a width of three characters
        // for each color component and a space between each, then each color takes up 11
        // characters by itself. If we put 5 colors on each line, that's 55 characters for the
        // colors, plus 4 spaces for a total of 59. One more full color would put us at 71, or just
        // over the limit.
        for row in self.pixels.chunks(5) {
            writeln!(
                f,
                "{}",
                row.iter()
                    .flat_map(|color| color.components().iter())
                    .map(|component| format!("{:>3}", (component.clamp(0.0, 1.0) * 255.0).round()))
                    .collect::<Vec<String>>()
                    .join(" ")
            )?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::canvas::Canvas;
    use crate::color::Color;
    use indoc::indoc;

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
    fn test_display() {
        let mut canvas = Canvas::new(5, 3);
        canvas.set_pixel(0, 0, Color::new(1.5, 0.0, 0.0));
        canvas.set_pixel(2, 1, Color::new(0.0, 0.5, 0.0));
        canvas.set_pixel(4, 2, Color::new(-0.5, 0.0, 1.0));

        assert_eq!(
            indoc! {"
            P3
            5 3
            255
            255   0   0   0   0   0   0   0   0   0   0   0   0   0   0
              0   0   0   0   0   0   0 128   0   0   0   0   0   0   0
              0   0   0   0   0   0   0   0   0   0   0   0   0   0 255
        "},
            format!("{canvas}")
        );
    }
}
