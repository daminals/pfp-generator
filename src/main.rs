extern crate image;
use image::Rgba;
use std::str;
use image::{GenericImage, GenericImageView, ImageBuffer, RgbImage};

fn main() {
    println!("Rust Profile Picture Generator");
    //let mut img: RgbImage = ImageBuffer::new(512, 512);
    //let mut img = ImageReader::open("static/head-shape/square.png").unwrap();
    //img.save("test.png").unwrap();
    color_replace("static/head-shape/square.png");
}

fn color_replace(img_location: &str) {
    let img = image::open(img_location).unwrap();
    let img_dim = img.dimensions(); // the dimensions of the image lol
    let new_col = Rgba([255,0,0,0]);
    for x in 0..(img_dim.0) {
        for y in 0..(img_dim.1) {
            let mut pixel = img.get_pixel(x,y);
            println!("{:?}",pixel);
            if pixel == Rgba([255,255,255,0]) {
                pixel = new_col;
            }
            println!("({},{}), {:?}", x, y,pixel);

            //img.put_pixel(pixel.0.clone(), pixel.1.clone(), Rgba([255,0,0,0]));
    
        }
    }
    img.save("test.png");
}