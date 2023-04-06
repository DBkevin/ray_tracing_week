fn main() {
    let image_width = 200;
    let image_height = 100;

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");
    for j in (0..image_height).rev() {
        eprintln!("Scanlines remaining:{}",j);
       
        for i in 0..image_width {
            let r: f32 = i as f32 / image_width as f32;
            let g: f32 = j as f32 / image_height as f32;
            let b = 0.2;

            let ir: i32 = (255.999 * r) as i32;
            let ig: i32 = (255.999 * g) as i32;
            let ib: i32 = (255.999 * b) as i32;
            println!("{} {} {}", ir, ig, ib);
        }

    }
    eprintln!("Done");
}
