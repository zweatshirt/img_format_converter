/* Author: Zachery Linscott */

use crate::fmt_info::{self, Thing};
use std::collections::HashMap;
use crate::fmt_info::get_thing_string;

pub fn display_info(image_fmts_map: &HashMap<Thing, Vec<Thing>>) {
    // Display options to user
    println!("DESCRIPTION:\n");
    println!("\tThis tool allows you to convert an image from one format to another.");
    println!("\tProvide an input image and specify the desired output path and format.");
    println!("\tFor example: `imgfmt -i meme.jpg -o /path/to/output/meme.png`.");
    println!("\tEnsure the output file has the correct format extension.");
    println!("\t Usage of full paths are recommended over relative paths.");
    println!("\nAVAILABLE OPTIONS:\n");

    for img_fmt in image_fmts_map.keys() {
        println!("{}:\n\t{}",
        get_thing_string(img_fmt),
        fmt_info::get_thing_string(&image_fmts_map.get(img_fmt).unwrap()[0]));
    }
}
