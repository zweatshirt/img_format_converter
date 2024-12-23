use std::{io::{stdin, stdout, Write}, process::exit};
use itertools::Itertools;
mod fmt_info;
mod fmt_exec;


// removed GenericImageViewer


/* Image processing project to learn Rust,
* as well as the study of parallel computing
* 
* What is image processing?
* - manipulation of digital images using algorithms
* - filtering, enhancment, resizing, etc
*
* Image - 2D vectors of pixels
* 
* Applications of img proc:
* - useful for computer vision
* - environment monitoring
* - video processing
* - photography
*
* Crate - fancy word for packages
* Cargo - Rust's build sys/pkg mgr
*
* Import image crate:
* `cargo add image`
*/

fn main() {
    let out_warn = "Failed to flush stdout";
    let image_fmts_map = fmt_info::get_image_fmts_map();
    println!("Choose which format to convert your image into");

    for img_fmt in image_fmts_map.keys().sorted() {
        println!("{})\t{}", 
        img_fmt, 
        fmt_info::get_thing_string(&image_fmts_map.get(img_fmt).unwrap()[0]));
    }

    print!("Enter by number: ");
    stdout().flush().expect(&out_warn);
    let mut line = String::new();
    stdin().read_line(&mut line).expect("Failed to read input");
    
    let line_num: i32 = line.trim().parse().unwrap();
    let user_fmt: String;
    user_fmt = fmt_info::get_thing_string(
        &image_fmts_map.get(&line_num)
        .unwrap()[0]);
    
    let mut path = String::new();
    print!("Please enter the path to your file: ");
    stdout().flush().expect(&out_warn);
    stdin().read_line(&mut path).expect("Failed to read input");
    let path = path.trim();
    let img = fmt_exec::fetch_img(path);
    let new_img_fmt = fmt_info::access_thing_fmt(
        &image_fmts_map.get(&line_num)
        .unwrap()[2]
    );

    println!("Converting from {} to {}.", fmt_exec::find_fmt(path), &user_fmt);
    fmt_exec::convert_format(img, path, new_img_fmt, &user_fmt);
    println!("Done!");
    exit(0);

}


