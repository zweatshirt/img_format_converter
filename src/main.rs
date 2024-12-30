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
    // messages for stdout and for when errors occur
    let out_warn = "Failed to flush stdout";
    let in_warn = "failed to read input";
    let enter_num = "Enter the number corresponding to your choice: ";
    let enter_path = "Please enter the path to the image you want converted: ";

    let image_fmts_map = fmt_info::get_image_fmts_map();
    println!("Choose which format to convert your image into");

    // Display options to user
    for img_fmt in image_fmts_map.keys().sorted() {
        println!("{})\t{}", 
        img_fmt, 
        fmt_info::get_thing_string(&image_fmts_map.get(img_fmt).unwrap()[0]));
    }

    // Ask user which option they would like and read
    print!("{}", enter_num);
    stdout().flush().expect(&out_warn);
    let mut line = String::new();
    stdin().read_line(&mut line).expect(&in_warn);
    
    // Grab format info based on user decision
    let line_num: i32 = line.trim().parse().unwrap();
    let user_fmt: String;
    user_fmt = fmt_info::get_thing_string(
        &image_fmts_map.get(&line_num)
        .unwrap()[0]
    );
    
    // read in and parse path to user's image
    let mut path = String::new();
    print!("{}", enter_path);
    stdout().flush().expect(&out_warn);
    stdin().read_line(&mut path).expect(&in_warn);
    let path = path.trim();

    // open the image file and return a Dynamic Image 
    let img = fmt_exec::fetch_img(path);

    // Get the new Image Format to turn user's image into
    let new_img_fmt = fmt_info::get_thing_fmt(
        &image_fmts_map.get(&line_num)
        .unwrap()[2]
    );

    // Convert the image given the desired image format from the user
    println!("Converting from {} to {}.", fmt_exec::find_fmt(path), &user_fmt);
    fmt_exec::convert_format(img, path, new_img_fmt, &user_fmt);
    println!("Done!");

    exit(0);

}


