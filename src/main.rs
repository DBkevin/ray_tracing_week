use crate::vec3::*;
mod vec3;

fn main() {
    let image_width = 200;
    let image_height = 100;

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");
    for j in (0..image_height).rev() {
        eprintln!("Scanlines remaining:{}", j);

        for i in 0..image_width {
            let r: f64 = i as f64 / image_width as f64;
            let g: f64 = j as f64 / image_height as f64;
            let b = 0.2;
            Vec3::new(r, g, b).write_color();
            // let ir: i32 = (255.999 * r) as i32;
            // let ig: i32 = (255.999 * g) as i32;
            // let ib: i32 = (255.999 * b) as i32;
            // println!("{} {} {}", ir, ig, ib);
        }
    }
    eprintln!("Done");
}
