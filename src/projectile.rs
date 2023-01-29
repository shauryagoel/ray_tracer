use crate::Tuple;

pub struct Projectile {
    pub position: Tuple,
    pub velocity: Tuple,
}

pub struct Environment {
    pub gravity: Tuple,
    pub wind: Tuple,
    pub projectile: Projectile,
}

impl Environment {
    pub fn tick(&mut self) -> Projectile {
        let new_position = self.projectile.position + self.projectile.velocity;
        let new_velocity = self.projectile.velocity + self.gravity + self.wind;
        Projectile {
            position: new_position,
            velocity: new_velocity,
        }
    }
}
