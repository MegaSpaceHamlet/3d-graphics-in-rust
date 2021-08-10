extern crate image;

use image::{GenericImage, GenericImageView, ImageBuffer, RgbImage, Rgb};
use graphics::imaging::canvas::Canvas;

fn main() {
    let mut img: RgbImage = ImageBuffer::new(512, 512);

    for x in 0..512 {
        for y in 0..512 {
            img.put_pixel(x, y, Rgb([255, 0, 0]));
        }
    }

    match img.save("img.jpeg") {
        Ok(img) => println!("Success!"),
        Err(_) => println!("Something went wrong")
    }

    let mut c: Canvas = Canvas::new();
    c.put_pixel(0, 0, [255, 0 , 0]);
    match c.save("canvas.png") {
        Ok(c) => println!("Success!"),
        Err(_) => println!("Hmmm"),
    };
}