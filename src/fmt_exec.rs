use image::{open, DynamicImage, ImageFormat}; 

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

pub fn find_fmt(path: &str) -> &str {
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
