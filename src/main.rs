use std::time::Instant;
use ray_tracer_challenge::*;
use ray_tracer_challenge::intersection::Intersections;
use ray_tracer_challenge::shapes::{Shape, Sphere};
use std::f64::consts::PI;
use std::fs::{OpenOptions};
use std::io;
use std::io::Write;
use std::sync::Mutex;
use rayon::prelude::*;
use ray_tracer_challenge::shapes::shape_enum::*;

fn main() -> io::Result<()> {
    let size = 2048;

    let start_time = Instant::now();

    cast_ray_at_sphere(size);

    let elapsed_time = start_time.elapsed();

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(r#"C:\tmp\tests.log"#)?;

    write!(file, "Rendering {:?} pixels. Elapsed time: {:?}\n", size * size, elapsed_time)?;

    println!("Elapsed time: {:?}", elapsed_time);

    Ok(())
}


fn cast_ray_at_sphere(size: usize) {
    let canvas_pixels = size;
    let wall_size = 7.0;
    let wall_z = 10.0;
    let pixel_size = wall_size / canvas_pixels as f64;
    let half = wall_size / 2.0;

    let mut canvas_mutex = Mutex::new(Canvas::new(canvas_pixels, canvas_pixels));
    let mut sphere = Sphere::new().with_transform(rotation_x(PI / 6.0) * scaling(1.0, 0.8, 1.1));
    let ray_origin = Tuple::point(0.0, 0.0, -5.0);

    // Chapter 6 additions
    sphere.material.color = Color::new(1.0, 0.0, 1.0);
    let light_position = Tuple::point(10.0, 0.0, -10.0);
    let light_color = Color::white();
    let light = Light::new(light_position, light_color);

    (0..canvas_pixels).into_par_iter().for_each(|y| {
        (0..canvas_pixels).into_par_iter().for_each(|x| {
            let world_y = half - pixel_size * (y as f64);
            let world_x = -half + pixel_size * (x as f64);

            let position = Tuple::point(world_x, world_y, wall_z);

            let ray_direction = (position - ray_origin).normalize();
            let ray = Ray::new(ray_origin, ray_direction);

            let xs = Intersections::new_from_vec(sphere.intersect(ray));

            if xs.hit().is_some() {
                let hit = xs.hit().unwrap();
                let s = match hit.object {
                    Shape::Sphere(sphere) => sphere
                };

                let point = ray.position(hit.t);
                let normal = s.normal_at(point);
                let eye = -ray.direction;

                let color = s.material.lighting(light, point, eye, normal);

                let mut canvas = canvas_mutex.lock().unwrap();
                canvas.write_pixel(x, y, color);
            }
        });
    });

    let mut canvas = canvas_mutex.lock().unwrap();

    canvas.export(r#"C:\tmp\output.png"#).expect("Couldn't create image");

    drop(canvas)
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
