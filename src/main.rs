use std::time::Instant;
use ray_tracer_challenge::*;
use ray_tracer_challenge::intersection::Intersections;
use ray_tracer_challenge::shapes::Sphere;

fn main() {
    let start_time = Instant::now();

    cast_ray_at_sphere();

    let elapsed_time = start_time.elapsed();

    println!("Elapsed time: {:?}", elapsed_time);
}


fn cast_ray_at_sphere() {
    let canvas_pixels = 1024;
    let wall_size = 7.0;
    let wall_z = 10.0;
    let pixel_size = wall_size / canvas_pixels as f64;
    let half = wall_size / 2.0;

    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);
    let red = Color::red();
    let sphere = Sphere::new();
    let ray_origin = Tuple::point(0.0, 0.0, -5.0);

    for y in 0..canvas_pixels {
        let world_y = half - pixel_size * (y as f64);

        for x in 0..canvas_pixels {
            let world_x = -half + pixel_size * (x as f64);

            let position = Tuple::point(world_x, world_y, wall_z);
            let ray_direction = (position - ray_origin).normalize();
            let ray = Ray::new(ray_origin, ray_direction);

            let xs = Intersections::new_from_vec(sphere.intersect(ray));

            if xs.hit().is_some() {
                canvas.write_pixel(x, y, red);
            }
        }
    }

    canvas.export(r#"C:\tmp\output.png"#).expect("Couldn't create image")
}


fn canon_example() {
    let mut canvas = Canvas::new(900, 550);
    let mut projectile = Tuple::point(10.0, 1.0, 0.0);
    let mut velocity = (Tuple::vector(1.0, 1.8, 0.0).normalize()) * 11.25;
    let gravity = Tuple::vector(0.0, -0.1, 0.0);
    let wind = Tuple::vector(-0.01, 0.0, 0.0);


    while projectile.y() >= 0.0 {
        println!("{:.2} meter", projectile.x());

        let x_pixel = projectile.x().round() as usize;
        let y_pixel = (canvas.height as f64 - projectile.y()).round() as usize;
        canvas.write_pixel(x_pixel, y_pixel, Color::red());

        projectile = projectile + velocity;
        velocity = velocity + gravity + wind;
    }

    canvas.export(r#"C:\tmp\output.png"#).expect("Couldn't create image");
    canvas.export_ppm(r#"C:\tmp"#).expect("Couldn't create image");
}

fn draw_canvas() {
    let mut canvas = Canvas::new(600, 600);
    let width = canvas.width;
    for (i, pixel) in canvas.pixels().iter_mut().enumerate() {
        let row = i / width;
        let col = i % width;

        if (row + col) % 2 == 0 {
            *pixel = Color::green();
        } else {
            *pixel = Color::blue();
        }
    }

    canvas.export(r#"C:\tmp\output.png"#).expect("Couldn't create image");
}
