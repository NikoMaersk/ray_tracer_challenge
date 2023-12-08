use image::ImageResult;
use crate::color::Color;

pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Color>
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Canvas::new_with_color(width, height, Color::white())
    }

    pub fn new_with_color(width: usize, height: usize, color: Color) -> Self {
        let pixels: Vec<Color> = vec![color; width * height];
        Canvas { width, height, pixels }
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
        if x < self.width && y < self.height {
            self.pixels[y * self.width + x] = color
        }
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> Color {
        if x < self.width && y < self.height {
            return self.pixels[y * self.width + x];
        }

        Color::black()
    }

    pub fn pixels(&mut self) -> &mut Vec<Color> {
        &mut self.pixels
    }

    pub fn export(&self, path: &str) -> ImageResult<()> {
        let mut img = image::ImageBuffer::new(self.width as u32, self.height as u32);

        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let color = &self.pixel_at(x as usize, y as usize);
            let (r, g, b) = convert_color_u8(color);
            *pixel = image::Rgb([r, g, b]);
        }

        img.save(path)
    }
}

pub fn convert_f32_to_u8(component: f32) -> u8 {
    let component = if component < 0.0 {
        0.0
    } else if component > 1.0 {
        1.0
    } else {
        component
    };

    (component * 255.0) as u8
}

fn convert_color_u8(color: &Color) -> (u8, u8, u8) {
    (
        convert_f32_to_u8(color.r),
        convert_f32_to_u8(color.g),
        convert_f32_to_u8(color.b),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create_canvas() {
        let canvas_width = 4;
        let canvas_height = 3;
        let color = Color::new(0.0, 0.0, 0.0);
        let canvas = Canvas::new_with_color(canvas_width, canvas_height, color);

        assert_eq!(canvas.width, canvas_width);
        assert_eq!(canvas.height, canvas_height);
        assert_eq!(canvas.pixels.len(), canvas_width * canvas_height);
    }

    #[test]
    fn write_pixel_to_canvas() {
        let mut canvas = Canvas::new(600, 600);
        let red = Color::red();

        canvas.write_pixel(2, 3, red);
        let expected = Color::new(1.0, 0.0,  0.0);
        assert_eq!(canvas.pixel_at(2, 3), expected)
    }
}