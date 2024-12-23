use std::{collections::HashMap, io::{stdin, stdout, Stdout, Write}, process::exit};
use itertools::Itertools;

// removed GenericImageViewer
use image::{open, DynamicImage, ImageFormat}; 

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
    let image_fmts_map = get_image_fmts_map();
    println!("Choose which format to convert your image into");

    for img_fmt in image_fmts_map.keys().sorted() {
        println!("{})\t{}", 
        img_fmt, 
        access_thing_string(&image_fmts_map.get(img_fmt).unwrap()[0]));
    }
    print!("Enter by number: ");
    stdout().flush().expect(&out_warn);
    let mut line = String::new();
    stdin().read_line(&mut line).expect("Failed to read input");
    
    let line_num: i32 = line.trim().parse().unwrap();
    let user_fmt: String;
    user_fmt = access_thing_string(&image_fmts_map.get(&line_num).unwrap()[0]);
    
    let mut path = String::new();
    print!("Please enter path to your file: ");
    stdout().flush().expect(&out_warn);
    stdin().read_line(&mut path).expect("Failed to read input");
    let path = path.trim();
    let img = fetch_img(path);
    let new_img_fmt = access_thing_fmt(&image_fmts_map.get(&line_num).unwrap()[2]);

    println!("Converting from {} to {}.", find_fmt(path), &user_fmt);
    convert_format(img, path, new_img_fmt, &user_fmt);

    exit(0);

}

pub fn fetch_img(path: &str) -> DynamicImage {
    match open(path) {
        Ok(img) => { img },
        Err(image_error) => { 
            panic!("{image_error:?}") 
        }
    }
}

pub fn convert_format(img:DynamicImage, path: &str, new_img_fmt: ImageFormat, new_fmt_str: &String) {
    let orig_fmt = find_fmt(path);
    img.save_with_format(
        path.replace(orig_fmt, new_fmt_str), 
        new_img_fmt)
        .expect("Failed to convert to PNG");
}

fn find_fmt(path: &str) -> &str {
    let mut original_fmt: Option<&str> = None;
    let mut i = 0;
    for char in path.chars() {
        
        if char == '.' {
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
            return val;
        }
    }
}


pub enum Thing {
    Name(String),
    Details(String),
    ImageFormat(ImageFormat),

}

pub fn access_thing_string(thing: &Thing) -> String {
    match thing {
        Thing::Name(name) => { name.to_string() },
        Thing::Details(details) => { details.to_string() },
        _ => panic!("Not a string")
    }
}

pub fn access_thing_fmt(thing: &Thing) -> ImageFormat {
    match thing {
        Thing::ImageFormat(img_format) => { *img_format },
        _ => panic!("Not a valid format!")
    }
}

// very much needs to be fixed/automated
pub fn get_image_fmts_map() -> HashMap<i32, Vec<Thing>> {
     return HashMap::from([
        (
            1,
            vec![
                Thing::Name(String::from(".png")),
                Thing::Details(String::from("PNG (Portable Network Graphics) is a lossless image format 
                that supports transparent backgrounds and is commonly used for web graphics, 
                offering high-quality images with efficient compression.")),
                Thing::ImageFormat(ImageFormat::Png)
            ],
        ),
        (
            2,
            vec![
                Thing::Name(String::from(".jpeg")),
                Thing::Details(String::from("JPEG (Joint Photographic Experts Group) is a widely used image format 
                that compresses photographic images with lossy compression, balancing high quality 
                and smaller file sizes." )),
                Thing::ImageFormat(ImageFormat::Jpeg)
                
            ],
        ),
        (   
            3,
            vec![
                Thing::Name(String::from(".gif")),
                Thing::Details(String::from(
                "GIF (Graphics Interchange Format) is an image format that supports animation 
                and lossless compression, making it ideal for simple graphics 
                and short, looping animations.")),
                Thing::ImageFormat(ImageFormat::Gif)
            ] 
        ),
        (
            4,
            vec![
                Thing::Name(String::from(".webp")),
                Thing::Details(String::from(
                "WebP is an image format developed by Google that provides both lossless and lossy compression, 
                offering high-quality images at smaller file sizes compared to JPEG and PNG.")),
                Thing::ImageFormat(ImageFormat::WebP)
            ]
        ),
        (
        5,
        vec![
            Thing::Name(String::from(".pnm")),
            Thing::Details(String::from( 
            "PNM (Portable Any Map) is a family of image formats, including PPM, PGM, and PBM, 
            that store simple grayscale or color images with an uncompressed, plain-text or binary format.")),
            Thing::ImageFormat(ImageFormat::Pnm),
        ]
    ), 
    (
        6,
        vec![
            Thing::Name(String::from(".tiff")), 
            Thing::Details(String::from("TIFF (Tagged Image File Format) is a flexible image format that supports high-quality, 
            uncompressed or compressed images, commonly used in professional photography and publishing 
            for its ability to preserve image details.")),
            Thing::ImageFormat(ImageFormat::Tiff),
        ]
    ), 
    (
        7, 
        vec![
            Thing::Name(String::from(".tga")), 
            Thing::Details(String::from("TGA (Targa) is an image format that supports high-quality, uncompressed or 
            compressed images, commonly used in video editing and graphic design, 
            particularly for its support of alpha transparency.")),
            Thing::ImageFormat(ImageFormat::Tga),
        ]
    ), 
    (
        8,
        vec![
            Thing::Name(String::from(".dds")), 
            Thing::Details(String::from("DDS (DirectDraw Surface) is an image format commonly used for storing textures and cube maps in 3D graphics, 
            supporting compression techniques like DXT and enabling efficient use in real-time rendering.")),
            Thing::ImageFormat(ImageFormat::Dds),
        ]
    ), 
    (
        9,
        vec![
            Thing::Name(String::from(".bmp")), 
            Thing::Details(String::from("BMP (Bitmap) is a simple, uncompressed image format that stores pixel data in a grid, 
            commonly used in Windows environments, offering high-quality images but with large file sizes.")),
            Thing::ImageFormat(ImageFormat::Bmp)
        ]
    ), 
    (
        10,
        vec![
            Thing::Name(String::from(".ico")), 
            Thing::Details(String::from("ICO (Icon) is an image format used for storing icon files in Windows, 
            supporting multiple resolutions and color depths to display icons in various sizes and formats 
            across applications and interfaces.")),
            Thing::ImageFormat(ImageFormat::Ico),
        ]
    ),
    (
        11,
        vec![
            Thing::Name(String::from(".farbfeld")), 
            Thing::Details(String::from("Farbfeld is a simple, uncompressed image format that stores RGB and alpha channel data in a binary format, 
            designed for high-quality, lossless image storage with a focus on simplicity.")),
            Thing::ImageFormat(ImageFormat::Farbfeld)
        ]
    ), 
    (
        12,
        vec![
            Thing::Name(String::from(".avif")), 
            Thing::Details(String::from("AVIF (AV1 Image File Format) is an image format that uses the AV1 video codec for high-efficiency compression, 
            providing excellent image quality at smaller file sizes, often used for web images.")),
            Thing::ImageFormat(ImageFormat::Avif)
        ]
    ), 
    (
        13,
        vec![
            Thing::Name(String::from(".qoi")), 
            Thing::Details(String::from("QOI (Quite OK Image) is a simple and efficient lossless image format designed for fast encoding and decoding, 
            providing high-quality images with minimal file sizes and easy implementation.")),
            Thing::ImageFormat(ImageFormat::Qoi)
        ]
    ),
    (
        14,
        vec![
            Thing::Name(String::from(".pcx")), 
            Thing::Details(String::from("PCX (Personal Computer Exchange) is an early image format that supports lossless compression, 
            commonly used in DOS-based graphics applications, known for its simplicity and support for indexed 
            color and grayscale images.")),
            Thing::ImageFormat(ImageFormat::Pcx),
        ] 
    )
    ]);
}
