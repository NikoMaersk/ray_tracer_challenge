use ray_tracer_challenge::*;

fn main() {

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
        let y_pixel = (canvas.height as f32 - projectile.y()).round() as usize;
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
