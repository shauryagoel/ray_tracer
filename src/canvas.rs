use crate::Color;

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub data: Vec<Color>,
}

impl Canvas {
    // Initialize a new canvas of given dimensions with black pixels
    pub fn new(width: usize, height: usize) -> Canvas {
        Canvas {
            width,
            height,
            data: vec![Color::black(); width * height],
        }
    }

    pub fn write_pixel(&mut self, col: usize, row: usize, color: Color) {
        self[row][col] = color;
    }

    pub fn pixel_at(&self, col: usize, row: usize) -> Color {
        self[row][col]
    }

    // Convert canvas to ppm format
    pub fn get_ppm(&self) -> String {
        let mut s: String = String::from("");
        let header = self.get_ppm_header();
        let pixel_values = self.get_ppm_pixel_values();
        s += &(header + &pixel_values);
        s
    }

    fn get_ppm_header(&self) -> String {
        let mut header: String = String::from("");
        header += "P3\n";
        header += &(self.width.to_string() + " " + &self.height.to_string() + "\n");
        header += "255\n";
        header
    }

    fn get_ppm_pixel_values(&self) -> String {
        let mut pixels: String = String::from("");
        for row in 0..self.height {
            for col in 0..self.width {
                let _color = self.pixel_at(col, row);
                // scale pixel value between 0 and 255
                let red_value = (_color.red * 255_f32).clamp(0.0, 255.0).round() as u8;
                let green_value = (_color.green * 255_f32).clamp(0.0, 255.0).round() as u8;
                let blue_value = (_color.blue * 255_f32).clamp(0.0, 255.0).round() as u8;
                pixels += &(red_value.to_string() + " ");
                pixels += &(green_value.to_string() + " ");
                pixels += &(blue_value.to_string() + " ");
            }
            // pixels = pixels.trim_end().to_string();
            pixels.pop(); // above might be slow as it creates a new string
            pixels += "\n";
        }
        pixels
    }
}

impl std::ops::Index<usize> for Canvas {
    type Output = [Color];

    fn index(&self, row: usize) -> &Self::Output {
        &self.data[row * self.width..(row + 1) * self.width]
    }
}

impl std::ops::IndexMut<usize> for Canvas {
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        &mut self.data[row * self.width..(row + 1) * self.width]
    }
}

#[cfg(test)]
mod canvas_tests {
    use super::*;

    #[test]
    fn canvas_init_with_black() {
        let c = Canvas::new(10, 20);
        let black_color = Color::new(0.0, 0.0, 0.0);

        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);
        for i in 1..c.height {
            for j in 1..c.width {
                assert!(c.pixel_at(j, i) == black_color);
            }
        }
    }

    #[test]
    fn writing_a_pixel_in_canvas() {
        let mut c = Canvas::new(10, 20);
        let red = Color::new(1.0, 0.0, 0.0);
        c.write_pixel(2, 3, red);
        assert!(c.pixel_at(2, 3) == red);
    }

    #[test]
    fn ppm_header() {
        let c = Canvas::new(5, 3);
        let ppm = c.get_ppm();
        let mut ppm_iter = ppm.split("\n");
        assert_eq!("P3", ppm_iter.next().unwrap());
        assert_eq!("5 3", ppm_iter.next().unwrap());
        assert_eq!("255", ppm_iter.next().unwrap());
    }

    #[test]
    fn ppm_pixel_data() {
        let mut c = Canvas::new(5, 3);
        let c1 = Color::new(1.5, 0.0, 0.0);
        let c2 = Color::new(0.0, 0.5, 0.0);
        let c3 = Color::new(-0.5, 0.0, 1.0);
        c.write_pixel(0, 0, c1);
        c.write_pixel(2, 1, c2);
        c.write_pixel(4, 2, c3);

        let ppm = c.get_ppm();
        // Skips the header as we want to compare pixel data
        let mut ppm_iter = ppm.split("\n").skip(3);
        assert_eq!("255 0 0 0 0 0 0 0 0 0 0 0 0 0 0", ppm_iter.next().unwrap());
        assert_eq!("0 0 0 0 0 0 0 128 0 0 0 0 0 0 0", ppm_iter.next().unwrap());
        assert_eq!("0 0 0 0 0 0 0 0 0 0 0 0 0 0 255", ppm_iter.next().unwrap());
    }
}
