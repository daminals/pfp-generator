extern crate image;
//use crate::image::Pixel;
use image::Rgba;
use std::str;
use std::collections::HashMap;
use image::{GenericImage, GenericImageView, ImageBuffer, RgbaImage};

fn main() {
    println!("Rust Profile Picture Generator");
    //let mut img: RgbImage = ImageBuffer::new(512, 512);
    //let mut img = ImageReader::open("static/head-shape/square.png").unwrap();
    //img.save("test.png").unwrap();
    color_replace("static/head/square.png", "output/head.png", "green");
}

fn color_replace(img_location: &str, final_loc: &str, color: &str) {
    // add a hashmap for colors here
    let new_col = return_color(color); // new col will equal the color corresponding to the hashmap

    let img = image::open(img_location).unwrap();
    let img_dim = img.dimensions(); // the dimensions of the image lol
    let mut colored_img: RgbaImage = ImageBuffer::new(img_dim.0,img_dim.1);
    let transparent = Rgba([255,255,255,0]);
    
    for x in 0..(img_dim.0) {
        for y in 0..(img_dim.1) {
            let mut pixel = img.get_pixel(x,y); 
            if pixel != transparent {
                pixel = new_col;
                //println!("({},{}), {:?}", x, y,pixel);
            }
            colored_img.put_pixel(x, y, pixel);
        }
    }
    colored_img.save(final_loc);
}

fn return_color(new_col: &str)-> image::Rgba<u8> {
    let color_hm = HashMap::from([
        ("blue", Rgba([0,0,255,255])),
        ("red",Rgba([255,0,0,255])),
        ("green",Rgba([0,255,0,255])),
        ("white",Rgba([255,255,255,255])),
        ("yellow",Rgba([255,255,0,255])),
        ("orange",Rgba([255,128,0,255])),
        ("purple",Rgba([204,0,204,255])),
        ("pink",Rgba([255,51,153,255]))
    ]);

    if color_hm.contains_key(new_col) {
        return color_hm[new_col]
    } else {
        panic!("Color does not exist")
    }
}