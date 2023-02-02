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
                assert!(c[i][j] == black_color);
            }
        }
    }

    #[test]
    fn writing_a_pixel_in_canvas() {
        let mut c = Canvas::new(10, 20);
        let red_color = Color::new(1.0, 0.0, 0.0);
        c[3][2] = red_color; // As it is more easier to do row,column indexing
        assert!(c[3][2] == red_color);
    }
}
