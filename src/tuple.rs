pub struct Tuple {
    pub x: f32, // x coordinate of the Tuple
    pub y: f32, // y coordinate of the Tuple
    pub z: f32, // z coordinate of the Tuple
    pub w: f32, // indicator whether the Tuple is a vector(w=0.0) or a point(w=1.0). It is float as we need it for computation rather than an actual indicator
}

impl Tuple {
    pub fn add(&self, other: &Tuple) -> Tuple {
        Tuple {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }

    pub fn is_vector(&self) -> bool {
        self.w == 0.0
    }

    pub fn is_point(&self) -> bool {
        self.w == 1.0
    }
}
