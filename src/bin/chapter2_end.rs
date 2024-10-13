use ray_tracer::{point, vector, Canvas, Color, Environment, Projectile};

fn main() {
    let starting_position = point(0.0, 1.0, 0.0);
    let starting_velocity = vector(1.0, 1.8, 0.0).normalize() * 11.25;
    let projectile = Projectile {
        position: starting_position,
        velocity: starting_velocity,
    };

    let gravity = vector(0.0, -0.1, 0.0);
    let wind = vector(-0.01, 0.0, 0.0);
    let mut environment = Environment {
        gravity,
        wind,
        projectile,
    };

    let mut canvas = Canvas::new(900, 550);
    let projectile_color = Color::new(1.0, 0.5, 0.5);

    while environment.projectile.position.y > 0.0 {
        let x_location = environment.projectile.position.x as usize;
        let y_location = canvas.height - (environment.projectile.position.y as usize);

        // Write the position of the projectile to canvas
        canvas.write_pixel(x_location, y_location, projectile_color);
        // Write these to increase the pixel size when viewing in an image viewer
        if x_location > 1 {
            canvas.write_pixel(x_location - 1, y_location, projectile_color);
        }
        if y_location > 1 {
            canvas.write_pixel(x_location, y_location - 1, projectile_color);
        }
        if x_location + 1 < canvas.width {
            canvas.write_pixel(x_location + 1, y_location, projectile_color);
        }
        if y_location + 1 < canvas.height {
            canvas.write_pixel(x_location, y_location + 1, projectile_color);
        }
        environment.tick();
    }

    let ppm_string = canvas.get_ppm();
    canvas.write_ppm(&ppm_string, "chapter2_end.ppm");
}
