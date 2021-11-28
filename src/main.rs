extern crate image;
use crate::image::Pixel;
use image::Rgba;
use std::str;
use image::{GenericImage, GenericImageView, ImageBuffer, RgbaImage};

fn main() {
    println!("Rust Profile Picture Generator");
    //let mut img: RgbImage = ImageBuffer::new(512, 512);
    //let mut img = ImageReader::open("static/head-shape/square.png").unwrap();
    //img.save("test.png").unwrap();
    color_replace("static/head/square.png", "output/head.png", "blue");
}

fn color_replace(img_location: &str, final_loc: &str, color: &str) {
    // add a hashmap for colors here
    let new_col = Rgba([0,0,255,255]); // new col will equal the color corresponding to the hashmap

    let img = image::open(img_location).unwrap();
    let img_dim = img.dimensions(); // the dimensions of the image lol
    let mut colored_img: RgbaImage = ImageBuffer::new(img_dim.0,img_dim.1);
    let transparent = Rgba([255,255,255,0]);
    
    for x in 0..(img_dim.0) {
        for y in 0..(img_dim.1) {
            let mut pixel = img.get_pixel(x,y); 
            if pixel != transparent {
                pixel = new_col;
                println!("({},{}), {:?}", x, y,pixel);
            }
            colored_img.put_pixel(x, y, pixel);
        }
    }
    colored_img.save(final_loc);
}