mod canvas;
mod color;
mod matrix;
mod matrix_small;
mod projectile;
mod ray;
mod transformation;
mod tuple;
mod utils;

pub use canvas::Canvas;
pub use color::Color;
pub use matrix::Matrix;
pub use matrix_small::{Matrix2, Matrix3};
pub use projectile::{Environment, Projectile};
pub use ray::Ray;
pub use tuple::{point, vector, Tuple};
pub use utils::Compare;
