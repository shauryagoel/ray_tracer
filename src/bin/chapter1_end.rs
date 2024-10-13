use ray_tracer::{point, vector, Environment, Projectile};

fn main() {
    let starting_position = point(0.0, 1.0, 0.0);
    let starting_velocity = vector(1.0, 1.0, 0.0).normalize();
    let gravity = vector(0.0, -0.1, 0.0);
    let wind = vector(-0.01, 0.0, 0.0);

    let projectile = Projectile {
        position: starting_position,
        velocity: starting_velocity,
    };
    let mut environment = Environment {
        gravity,
        wind,
        projectile,
    };

    let mut number_of_tick_to_hit_ground: u32 = 0;
    while environment.projectile.position.y > 0.0 {
        println!(
            "Projectile position- x:{}, y:{}, z: {}",
            environment.projectile.position.x,
            environment.projectile.position.y,
            environment.projectile.position.z
        );
        environment.tick();
        number_of_tick_to_hit_ground += 1;
    }
    println!("Number of ticks to hit the ground: {number_of_tick_to_hit_ground}");
}
