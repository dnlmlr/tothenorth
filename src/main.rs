use color::*;
use image::RgbImage;
use rayon::prelude::*;

use std::time::SystemTime;

mod color;
mod op_macros;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    let input_name = match args.get(1) {
        Some(it) => it.to_owned(),
        None => "test.png".to_owned(),
    };

    let target_percent = match args.get(2) {
        Some(it) => it.parse::<u8>().unwrap(),
        None => 70,
    } as f32;

    if target_percent > 100.0 {
        panic!(
            "Target percentage can't be over 100%. Is {}",
            target_percent
        );
    }

    println!("Input file: {}", &input_name);

    let nord: Vec<RgbF32> = [
        // Polar Night
        Rgb8::from_hex("#2e3440")?,
        Rgb8::from_hex("#3b4252")?,
        Rgb8::from_hex("#434c5e")?,
        Rgb8::from_hex("#4c566a")?,
        // Snow Storm
        Rgb8::from_hex("#d8dee9")?,
        Rgb8::from_hex("#e5e9f0")?,
        Rgb8::from_hex("#eceff4")?,
        // Frost
        Rgb8::from_hex("#8fbcbb")?,
        Rgb8::from_hex("#88c0d0")?,
        Rgb8::from_hex("#81a1c1")?,
        Rgb8::from_hex("#5e81ac")?,
        // Aurora
        Rgb8::from_hex("#bf616a")?,
        Rgb8::from_hex("#d08770")?,
        Rgb8::from_hex("#ebcb8b")?,
        Rgb8::from_hex("#a3be8c")?,
        Rgb8::from_hex("#b48ead")?,
        // Dracula
        // Rgb8::from_hex("#282a36")?,
        // Rgb8::from_hex("#44475a")?,
        // Rgb8::from_hex("#f8f8f2")?,
        // Rgb8::from_hex("#6272a4")?,
        // Rgb8::from_hex("#8be9fd")?,
        // Rgb8::from_hex("#50fa7b")?,
        // Rgb8::from_hex("#ffb86c")?,
        // Rgb8::from_hex("#ff79c6")?,
        // Rgb8::from_hex("#bd93f9")?,
        // Rgb8::from_hex("#ff5555")?,
        // Rgb8::from_hex("#f1fa8c")?,
    ]
    .iter()
    .map(RgbF32::from)
    .collect();

    let t1 = SystemTime::now();

    let mut img = image::open(&input_name)?.to_rgb8();

    println!("loading the image took: {} ms", t1.elapsed()?.as_millis());

    let t_start = std::time::SystemTime::now();

    shift_to_schema(&mut img, nord, target_percent / 100.0);

    let elapsed = t_start.elapsed()?.as_millis();

    println!("Processing took: {} ms", elapsed);

    let t3 = SystemTime::now();

    img.save("output.png")?;

    println!("saving the image took: {} ms", t3.elapsed()?.as_millis());

    Ok(())
}

fn shift_to_schema(img: &mut RgbImage, palette: Vec<RgbF32>, target_multilpier: f32) {
    // Iterator over 3-byte chunks from the image buffer. Those 3-bytes are the r, g, b values as u8
    // The par prefix
    img.par_chunks_mut(3).for_each(|it| {
        // Assemble the rgb bytes into f32 rgb color
        let mut pix_color = RgbF32::new_u8(it[0], it[1], it[2]);

        // Search the color scheme to find the color closest to the current pixel color
        // This is done by calculating the distance between the colors in the 3D RGB colorspace
        // After the closest match is found, get the delta
        let mut closest_delta = palette
            .iter()
            .map(|it| {
                let delta = it - pix_color;
                (delta.dist(), delta)
            })
            .min_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
            .unwrap()
            .1;

        // Apply the factor to sepecify how close to the target color it should get
        closest_delta *= target_multilpier;

        // Apply the delta to the actual color
        pix_color += closest_delta;

        // Get the rgb values as bytes
        let raw = pix_color.to_bytes();

        // And set the rgb values in the image buffer to those values
        it[0] = raw[0];
        it[1] = raw[1];
        it[2] = raw[2];
    });
}
