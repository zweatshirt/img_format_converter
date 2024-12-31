use std::{env::{self}, process::exit};
use fmt_exec::find_fmt;
use fmt_info::{get_thing_fmt, Thing};
mod fmt_info;
mod fmt_exec;
mod help;
/*
* Author: Zachery Linscott
* Format: ./image_proc -i inputpath.format1 -o outputpath.format2
* E.g. ./image_proc -i home/Desktop/meme.png -o home/Desktop/meme.jpeg
*
* Run ./image_proc -h 
* for a list of format options
*/


fn main() {
    let image_fmts_map = fmt_info::get_image_fmts_map();
    let mut input_path = String::from("");
    let mut output_path = String::from("");

    let args: Vec<String> = env::args().collect();
    for i in 1..args.len() {
        match args[i].as_str() {
            "-h" | "--help" => { 
                help::display_info(&image_fmts_map); 
                return; 
            }
            "-i" => {
                if i + 1 < args.len() {
                    input_path = args[i + 1].clone();
                }
                else {
                    println!("Failed to read input path");
                    return;
                }
            }
            "-o" => {
                if i + 1 < args.len() {
                    output_path = args[i + 1].clone();
                }
                else {
                    println!("Failed to read output path");
                    return;
                }
            }
            _ => {
                if args[i].contains('-') {
                    println!("Unkown argument {}", args[i]);
                } 
            }
        }
    }

    println!("{input_path}, {output_path}");
    let in_fmt = find_fmt(&input_path);
    let out_fmt = find_fmt(&output_path);
    println!("{in_fmt}, {out_fmt}");
    let img = fmt_exec::fetch_img(&input_path);
    let new_img_fmt = &image_fmts_map.get(&Thing::Name(out_fmt.clone())).unwrap()[1];

    fmt_exec::convert_format(
        img, 
        input_path, 
        in_fmt,
        out_fmt,
        get_thing_fmt(&new_img_fmt),
    );
    println!("Done!");

    exit(0);

}



