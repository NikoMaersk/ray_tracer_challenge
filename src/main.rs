
use ray_tracer_challenge::*;

fn main() {
    let mut projectile = Tuple::point(0.0, 1.0, 0.0);
    let mut velocity = Tuple::vector(1.0, 1.0, 0.0);
    let gravity = Tuple::vector(0.0, -0.1, 0.0);
    let wind = Tuple::vector(-0.01, 0.0, 0.0);

    while projectile.y() >= 0.0 {
        println!("{:.2} meter", projectile.x());
        projectile = projectile + velocity;
        velocity = velocity + gravity + wind;
    }
}
