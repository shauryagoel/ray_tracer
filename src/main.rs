use ray_tracer::{point, vector, Environment, Projectile, Tuple};

fn main() {
    let data1 = Tuple::new(1.0, 2.0, 3.0, 0.0);
    let data2 = Tuple::new(0.0, 1.0, 2.0, 0.0);

    let test_data: Tuple = data1 + data2;

    println!(
        "Tuple points are: x={}, y={}, z={}, w={}",
        test_data.x, test_data.y, test_data.z, test_data.w
    );

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
        environment.projectile = environment.tick();
        number_of_tick_to_hit_ground += 1;
    }
    println!(
        "Number of ticks to hit the ground: {}",
        number_of_tick_to_hit_ground
    );
}
