/* Author: Zachery Linscott */

use image::{open, DynamicImage, ImageFormat}; 
use std::thread;

// open file at given path and return Dynamic Image
pub fn fetch_img(path: &str) -> DynamicImage {
    match open(path) {
        Ok(img) => { img },
        Err(image_error) => { 
            panic!("{image_error:?}") 
        }
    }
}

// convert the image file to the user's desired format
pub fn convert_format(img: DynamicImage, path: String, in_fmt_str: String, out_fmt_str: String, new_img_fmt: ImageFormat) {
    if *in_fmt_str == *out_fmt_str {
        panic!("You are trying to convert your image to the same type, please try again");
    }

    // let clone_path = path.to_string();
    // let clone_out_fmt_str = out_fmt_str.to_string();
    // let clone_in_fmt_str = in_fmt_str.to_string();

    let handle = thread::spawn(
        move || {
            img.save_with_format(
            path.replace(&in_fmt_str, &out_fmt_str), 
            new_img_fmt
            )
            .expect("Failed to convert.");
    });
    handle.join().expect("Save thread failed");
}

// find the original format from the user's provided path
pub fn find_fmt(path: &str) -> String {
    let mut original_fmt: Option<&str> = None;
    let mut i = 0;
    for char in path.chars() {
        // dodgy
        if char == '.' && i != 0 {
            original_fmt = Some(&path[i..]);
            break;
        }
        i+=1;
    }
    match original_fmt {
        None => {
            panic!("Failed to get image format from the path")
        },
        Some(val) => {
            return String::from(val);
        }
    }
}