use std::path::Path;

use image::{DynamicImage, ImageBuffer, RgbaImage, Rgba, GenericImageView};

const IMAGE_PATH: &str ="tux.png";

mod lcg;

fn read_image() -> image::DynamicImage {
    return image::open(IMAGE_PATH).unwrap();
}

// Encrypt the entire pixel with a single random number
fn xor_pixel(pixel: Rgba<u8>, random_number: u64) -> Rgba<u8> {
    image::Rgba([
                // Red 
                pixel[0] ^ random_number as u8,
                // Blue
                pixel[1] ^ random_number as u8,
                // Green
                pixel[2] ^ random_number as u8,
                // Alpha
                255
    ])
}

// Encrypt all components of the pixel with different random numbers.
fn xor_all_components_pixel(pixel: Rgba<u8>, random_numbers: &[u64]) -> Rgba<u8> {
    let encrypt_alpha: bool = false;
    image::Rgba([
                // Red 
                pixel[0] ^ random_numbers[0] as u8,
                // Blue
                pixel[1] ^ random_numbers[1] as u8,
                // Green
                pixel[2] ^ random_numbers[2] as u8,
                // Alpha
                match encrypt_alpha {
                    true => 255 ^ random_numbers[3] as u8,
                    false => 255
                }
    ])
}

fn generate_encrypted_image(image: DynamicImage, random_numbers: Vec<u64>) -> ImageBuffer<Rgba<u8>, Vec<u8>>{
    let mut enc_image: RgbaImage = ImageBuffer::new(image.width(), image.height());

    let mut j: usize = 0;
    for (x, y, enc_pixel) in enc_image.enumerate_pixels_mut() {
        let pixel = image.get_pixel(x, y);
        // *enc_pixel = xor_pixel(pixel, random_numbers[j]);
        *enc_pixel = xor_all_components_pixel(pixel, &random_numbers[j..j+4]);
        j += 1;
    }

    return enc_image;
}

fn main() {
    // 1. Read all image pixels
    let image: DynamicImage = read_image();
    // 2. Generate random numbers from the LCG.
    // The calculation is times 4 because we need a random number for each pixel
    // component. RGBA.
    let random_numbers: Vec<u64> = lcg::get_random_numbers((image.width() * image.height() * 4) as i64);
    // 3. Then XOR all pixels with random numbers generated from LCG.
    let enc_image = generate_encrypted_image(image, random_numbers);
    // 4. Write all the pixels to an image.
    enc_image.save("enc_image.png").unwrap();
}
