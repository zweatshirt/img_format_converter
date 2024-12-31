/* Author: Zachery Linscott */

use std::collections::HashMap;
use image::ImageFormat;

// enum/container for image format data
#[derive(Hash, PartialEq, Eq, Debug)] 
pub enum Thing {
    Name(String),
    Details(String),
    ImageFormat(ImageFormat),

}

// returns image data relating to format and description
pub fn get_thing_string(thing: &Thing) -> String {
    match thing {
        Thing::Name(name) => { name.to_string() },
        Thing::Details(details) => { details.to_string() },
        _ => panic!("Not a string")
    }
}

// returns the actual image format enum
pub fn get_thing_fmt(thing: &Thing) -> ImageFormat {
    match thing {
        Thing::ImageFormat(img_format) => { *img_format },
        _ => panic!("Not a valid format!")
    }
}

// very much needs to be fixed/automated
// Map of vectors of image format enums
pub fn get_image_fmts_map() -> HashMap<Thing, Vec<Thing>> {
     return HashMap::from([
        (
            Thing::Name(String::from(".png")),
            vec![
                
                Thing::Details(String::from("PNG (Portable Network Graphics) is a lossless image format 
                that supports transparent backgrounds and is commonly used for web graphics, 
                offering high-quality images with efficient compression.")),
                Thing::ImageFormat(ImageFormat::Png)
            ],
        ),
        (
            Thing::Name(String::from(".jpeg")),
            vec![
                Thing::Details(String::from("JPEG (Joint Photographic Experts Group) is a widely used image format 
                that compresses photographic images with lossy compression, balancing high quality 
                and smaller file sizes." )),
                Thing::ImageFormat(ImageFormat::Jpeg)
                
            ],
        ),
        (
            Thing::Name(String::from(".webp")),
            vec![
                Thing::Details(String::from(
                "WebP is an image format developed by Google that provides both lossless and lossy compression, 
                offering high-quality images at smaller file sizes compared to JPEG and PNG.")),
                Thing::ImageFormat(ImageFormat::WebP)
            ]
        ),
        (
            Thing::Name(String::from(".pnm")),
            vec![
            Thing::Details(String::from( 
            "PNM (Portable Any Map) is a family of image formats, including PPM, PGM, and PBM, 
            that store simple grayscale or color images with an uncompressed, plain-text or binary format.")),
            Thing::ImageFormat(ImageFormat::Pnm),
        ]
        ), 
        (
            Thing::Name(String::from(".tiff")), 
            vec![
                Thing::Details(String::from("TIFF (Tagged Image File Format) is a flexible image format that supports high-quality, 
                uncompressed or compressed images, commonly used in professional photography and publishing 
                for its ability to preserve image details.")),
                Thing::ImageFormat(ImageFormat::Tiff),
            ]
        ), 
        (
            Thing::Name(String::from(".tga")), 
            vec![
                Thing::Details(String::from("TGA (Targa) is an image format that supports high-quality, uncompressed or 
                compressed images, commonly used in video editing and graphic design, 
                particularly for its support of alpha transparency.")),
                Thing::ImageFormat(ImageFormat::Tga),
            ]
        ), 
        (
            Thing::Name(String::from(".dds")), 
            vec![
                Thing::Details(String::from("DDS (DirectDraw Surface) is an image format commonly used for storing textures and cube maps in 3D graphics, 
                supporting compression techniques like DXT and enabling efficient use in real-time rendering.")),
                Thing::ImageFormat(ImageFormat::Dds),
            ]
        ), 
        (
            Thing::Name(String::from(".bmp")), 
            vec![
                Thing::Details(String::from("BMP (Bitmap) is a simple, uncompressed image format that stores pixel data in a grid, 
                commonly used in Windows environments, offering high-quality images but with large file sizes.")),
                Thing::ImageFormat(ImageFormat::Bmp)
            ]
        ), 
        (
            Thing::Name(String::from(".ico")), 
            vec![
                Thing::Details(String::from("ICO (Icon) is an image format used for storing icon files in Windows, 
                supporting multiple resolutions and color depths to display icons in various sizes and formats 
                across applications and interfaces.")),
                Thing::ImageFormat(ImageFormat::Ico),
            ]
        ),
        (
            Thing::Name(String::from(".farbfeld")), 
            vec![
                Thing::Details(String::from("Farbfeld is a simple, uncompressed image format that stores RGB and alpha channel data in a binary format, 
                designed for high-quality, lossless image storage with a focus on simplicity.")),
                Thing::ImageFormat(ImageFormat::Farbfeld)
            ]
        ), 
        (
            Thing::Name(String::from(".avif")), 
            vec![
                Thing::Details(String::from("AVIF (AV1 Image File Format) is an image format that uses the AV1 video codec for high-efficiency compression, 
                providing excellent image quality at smaller file sizes, often used for web images.")),
                Thing::ImageFormat(ImageFormat::Avif)
            ]
        ), 
        // Doesn't currently work
        // (
        //     12,
        //     vec![
        //         Thing::Name(String::from(".qoi")), 
        //         Thing::Details(String::from("QOI (Quite OK Image) is a simple and efficient lossless image format designed for fast encoding and decoding, 
        //         providing high-quality images with minimal file sizes and easy implementation.")),
        //         Thing::ImageFormat(ImageFormat::Qoi)
        //     ]
        // ),
    ]);
}
