use image::Rgba;
use std::{str, io};
use std::collections::HashMap;
use image::{GenericImageView, ImageBuffer, RgbaImage};
use colored::{Colorize};
use threadpool::ThreadPool;
mod tb; // table.rs file

fn main() {
    println!("{}\n\n", "Rust Profile Picture Generator".magenta().underline().bold());
    head_create();
}

fn head_create() {
    let pool = ThreadPool::new(10);
    tb::head_shape_print();
    println!("Head shape: "); // let the user select the head shape
    let head_shape = return_user_input();
    let head_shape = return_head_shape(&head_shape[..]); // select the path of shape
    println!("\n");

    tb::color_print();
    println!("Color of head: ");
    // what color user wants?
    let user_v = return_user_input();
    // replace the color with the desired color
    color_replace(head_shape, "output/head.png",&user_v) // want to throw color replace in a thread but something about headshape borrowed var not living long enough
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
    //let resultant = match colored_img.save(final_loc)
    let result_save = colored_img.save(final_loc);
    match result_save {
        Ok(_) => println!("Successfully generated image"),
        Err(e) => println!("Failed to generate image. Error code: {:?}",e),
    }
}

fn return_user_input() -> String {
    let mut input_output = String::new();
    io::stdin()
    .read_line(&mut input_output)
    .expect("Failed to read line");
    
    input_output.trim().to_string()
}

fn return_color(new_col: &str)-> image::Rgba<u8> {
    let color_hm = HashMap::from([
        ("blue", Rgba([0,0,255,255])),
        ("red",Rgba([255,0,0,255])),
        ("green",Rgba([0,255,0,255])),
        ("white",Rgba([255,255,255,255])), // TODO: if white don't run col change func for speed purposes
        ("yellow",Rgba([255,255,0,255])),
        ("orange",Rgba([255,128,0,255])),
        ("purple",Rgba([204,0,204,255])),
        ("pink",Rgba([255,51,153,255])),
        // colors should be accessible by index also:
        ("0", Rgba([0,0,255,255])),
        ("1",Rgba([255,0,0,255])),
        ("2",Rgba([0,255,0,255])),
        ("3",Rgba([255,51,153,255])),
        ("4",Rgba([255,255,0,255])),
        ("5",Rgba([255,128,0,255])),
        ("6",Rgba([204,0,204,255])),
        ("7",Rgba([255,255,255,255]))
    ]);

    if color_hm.contains_key(new_col) {
        return color_hm[new_col]
    } else {
        panic!("Color does not exist")
    }
}
fn return_head_shape(head_shape: &str) -> &str {
    let head_hm = HashMap::from([
        ("square", "static/head/square.png"),
        ("circle","static/head/circle.png"),
        ("triangle1","static/head/triangle1.png"),
        ("triangle2","static/head/triangle2.png"),
        ("octagon", "static/head/octagon.png"),
        ("star", "static/head/star.png"),
        // indexes should be included as well
        ("0", "static/head/square.png"),
        ("1","static/head/circle.png"),
        ("2","static/head/triangle1.png"),
        ("3","static/head/triangle2.png"),
        ("4", "static/head/octagon.png"),
        ("5", "static/head/star.png")
    ]);
    if head_hm.contains_key(head_shape) {
        return head_hm[head_shape]
    } else {
        panic!("Head shape does not exist")
    }
}