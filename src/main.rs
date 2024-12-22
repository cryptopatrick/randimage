use image::{ImageBuffer, Rgb};
use rand::Rng;
use std::env;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <output_filename>", args[0]);
        return;
    }
    let output_filename = &args[1];

    //Alternative width for testing purposes.
    //1280_320_80
    //1280_640_80
    let width = 680u32;
    let height = 20u32;
    let block_size = 10u32;

    let number_of_blocks_horizontal = width / block_size;
    println!("{}", number_of_blocks_horizontal);
    let number_of_blocks_vertical = height / block_size;

    // RGB image buffer.
    let mut img = ImageBuffer::new(width, height);
    // Fill buffer with random colors.
    let mut rng = rand::thread_rng();

    for bh in 0..number_of_blocks_horizontal {
        for bv in 0..number_of_blocks_vertical {
            let r: u8 = rng.gen_range(0..=255); // Red
            let g: u8 = rng.gen_range(0..=255); // Green
            let b: u8 = rng.gen_range(0..=255); // Blue

            let color = Rgb([r, g, b]);

            // Fill with color.
            for y in (bv * block_size)..((bv + 1) * block_size) {
                for x in (bh * block_size)..((bh + 1) * block_size) {
                    img.put_pixel(x, y, color);
                }
            }
        }
    }

    // Save image as png.
    let output_path = Path::new(output_filename);
    match img.save(output_path) {
        Ok(_) => println!("Image saved as {}", output_filename),
        Err(e) => eprintln!("Failed to save image: {}", e),
    }
}
