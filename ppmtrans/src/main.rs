
use array2::Array2;
use csc411_image::{Read, Rgb, RgbImage, Write};
use std::env;
use std::process;



use clap::Parser;
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
// Flip
#[clap(long = "flip", required = false)]
flip: Option<String>,
// Rotation
#[clap(short = 'r', long = "rotate")]
rotate: Option<u32>,
// Row-Major
#[clap(long = "row-major")]
row_major: bool,
// Col-Major
#[clap(long = "col-major")]
col_major: bool,
// Transposition
#[clap(long = "transpose")]
transpose: bool,
// FileName
filename: Option<String>,
}


fn main() {
    let args = Args::parse();
    let rotate = args.rotate;
    let filename = args.filename;
    let transpose = args.transpose;
    let row_major = args.row_major;
    let col_major = args.col_major;
    let flip = args.flip;


    let img = RgbImage ::read(filename.as_deref()).unwrap();
   
    // get our values from grayimage to the correct types for our structure
    let height2 = img.height.try_into().unwrap();
    let width2 = img.width.try_into().unwrap();

    // Got to figure out how we're representing RGB
    let usize_vec: Vec<Rgb> = img.pixels.clone();
    
    //iter().map(|rgb| (rgb.red, rgb.green, rgb.blue)).collect();
    let try_2 = Array2::from_row_major(width2, height2, usize_vec).unwrap();
    /*for (x, y, &ref element) in try_2.iter_row_major(){
        println!("{}, {}, {:?}", x, y, element);
    }*/
   
 match rotate {
    Some(90) => {
        if col_major {
            // Call rotate90_cmo function
            let result = rotate90_cmo(&try_2);
            let image = RgbImage {
                width: result.width() as u32,
                height: result.height() as u32,
                denominator: 255,
                pixels: result.data().to_vec(),
            };
            let _ = RgbImage::write(&image, None);

            // Handle the result if needed
        } else {
            // Call rotate90_rmo function
            let result = rotate90_rmo(&try_2);
            let image = RgbImage {
                width: result.width() as u32,
                height: result.height() as u32,
                denominator: 255,
                pixels: result.data().to_vec(),
    
    
            };
            let _ = RgbImage::write(&image, None);

            // Handle the result if needed
        }
        

}
    Some(180) => {
        if col_major {
            // Call rotate180_cmo function
            let result = rotate180_cmo(&try_2);
            let image = RgbImage {
                width: result.width() as u32,
                height: result.height() as u32,
                denominator: 255,
                pixels: result.data().to_vec(),
    
    
            };
            let _ = RgbImage::write(&image, None);
            // Handle the result if needed
        } else {
            // Call rotate180_rmo function
            let result = rotate180_rmo(&try_2);
            let image = RgbImage {
                width: result.width() as u32,
                height: result.height() as u32,
                denominator: 255,
                pixels: result.data().to_vec(),
    
    
            };
            let _ = RgbImage::write(&image, None);
            // Handle the result if needed
        }
    }
    // Add more cases for other rotation angles if needed
    Some(270) => {
        if col_major {
            // Call rotate180_cmo function
            let result: Array2<Rgb> = rotate270_cmo(&try_2);
            let image = RgbImage {
                width: result.width() as u32,
                height: result.height() as u32,
                denominator: 255,
                pixels: result.data().to_vec(),
            };
            let _ = RgbImage::write(&image, None);
            // Handle the result if needed
        } else {
            // Call rotate180_rmo function
            let result = rotate270_rmo(&try_2);
            let image = RgbImage {
                width: result.width() as u32,
                height: result.height() as u32,
                denominator: 255,
                pixels: result.data().to_vec(),
            };
            let _ = RgbImage::write(&image, None);

            // Handle the result if needed
        }

    }

    Some(0) => {
        if col_major {
            // Call rotate180_cmo function
            let result: Array2<Rgb> = rotate0_cmo(&try_2);
            let image = RgbImage {
                width: result.width() as u32,
                height: result.height() as u32,
                denominator: 255,
                pixels: result.data().to_vec(),
            };
            let _ = RgbImage::write(&image, None);
            // Handle the result if needed
        } else {
            // Call rotate180_rmo function
            let result = rotate0_rmo(&try_2);
            let image = RgbImage {
                width: result.width() as u32,
                height: result.height() as u32,
                denominator: 255,
                pixels: result.data().to_vec(),
            };
            let _ = RgbImage::write(&image, None);

            // Handle the result if needed
        }

    }

    None => todo!(),
    _ => {
        // Handle all other cases here
    }
}




fn rotate90_rmo(img: &Array2<Rgb>) -> Array2<Rgb> {
    let mut rotated_img = Array2::blank_state(img.height(), img.width(), img.get(0, 0).clone());

    for (x, y, &ref element) in img.iter_row_major() {
        let rotated_x = img.height() - y - 1;
        let rotated_y = x;

        if rotated_x < rotated_img.width() && rotated_y < rotated_img.height() {
            let new = rotated_img.get_mut(rotated_x, rotated_y);
            *new = element.clone();
        }
    }
   
    rotated_img
}

fn rotate90_cmo(img: &Array2<Rgb>) -> Array2<Rgb> {
    let mut rotated_img = Array2::blank_state(img.height(), img.width(), img.get(0, 0).clone());

    for (x, y, &ref element) in img.iter_col_major() {
        let rotated_x = img.height() - y - 1;
        let rotated_y = x;

        if rotated_x < rotated_img.width() && rotated_y < rotated_img.height() {
            let new = rotated_img.get_mut(rotated_x, rotated_y);
            *new = element.clone();
        }
    }

    rotated_img
}

fn rotate180_rmo(img: &Array2<Rgb>) -> Array2<Rgb> {
    let mut rotated_img = Array2::blank_state(img.width(), img.height(), img.get(0, 0).clone());

    for (x, y, &ref element) in img.iter_row_major() {
        let rotated_x = img.width() - x - 1;
        let rotated_y = img.height() - y - 1;

        if rotated_x < rotated_img.width() && rotated_y < rotated_img.height() {
            let new = rotated_img.get_mut(rotated_x, rotated_y);
            *new = element.clone();
        }
    }

    rotated_img
}

fn rotate180_cmo(img: &Array2<Rgb>) -> Array2<Rgb> {
    let mut rotated_img = Array2::blank_state(img.width(), img.height(), img.get(0, 0).clone());

    for (x, y, &ref element) in img.iter_col_major() {
        let rotated_x = img.width() - x - 1;
        let rotated_y = img.height() - y - 1;

        if rotated_x < rotated_img.width() && rotated_y < rotated_img.height() {
            let new = rotated_img.get_mut(rotated_x, rotated_y);
            *new = element.clone();
        }
    }

    rotated_img
}


fn rotate270_rmo(img: &Array2<Rgb>) -> Array2<Rgb> {
    let mut rotated_img = Array2::blank_state(img.height(), img.width(), img.get(0, 0).clone());

    for (x, y, &ref element) in img.iter_row_major() {
        let rotated_x = y;
        let rotated_y = img.width() - x - 1;

        if rotated_x < rotated_img.width() && rotated_y < rotated_img.height() {
            let new = rotated_img.get_mut(rotated_x, rotated_y);
            *new = element.clone();
        }
    }

    rotated_img
}

fn rotate270_cmo(img: &Array2<Rgb>) -> Array2<Rgb> {
    let mut rotated_img = Array2::blank_state(img.height(), img.width(), img.get(0, 0).clone());

    for (x, y, &ref element) in img.iter_col_major() {
        let rotated_x = y;
        let rotated_y = img.width() - x - 1;

        if rotated_x < rotated_img.width() && rotated_y < rotated_img.height() {
            let new = rotated_img.get_mut(rotated_x, rotated_y);
            *new = element.clone();
        }
    }

    rotated_img
}


fn rotate0_rmo(img: &Array2<Rgb>) -> Array2<Rgb> {
    let mut rotated_img = Array2::blank_state(img.width(), img.height(), img.get(0, 0).clone());

    for (x, y, &ref element) in img.iter_row_major() {
        let rotated_x = x;
        let rotated_y = y;

        if rotated_x < rotated_img.width() && rotated_y < rotated_img.height() {
            let new = rotated_img.get_mut(rotated_x, rotated_y);
            *new = element.clone();
        }
    }

    rotated_img
}
fn rotate0_cmo(img: &Array2<Rgb>) -> Array2<Rgb> {
    let mut rotated_img = Array2::blank_state(img.width(), img.height(), img.get(0, 0).clone());

    for (x, y, &ref element) in img.iter_col_major() {
        let rotated_x = x;
        let rotated_y = y;

        if rotated_x < rotated_img.width() && rotated_y < rotated_img.height() {
            let new = rotated_img.get_mut(rotated_x, rotated_y);
            *new = element.clone();
        }
    }

    rotated_img
}

fn flip_horizontal_rmo(img: &Array2<Rgb>) -> Array2<Rgb> {
    let mut flipped_img: Array2<Rgb> = Array2::blank_state(img.width(), img.height(), img.get(0, 0).clone());

    for (x, y, &ref element) in img.iter_row_major() {
        let flipped_y = img.width() - y - 1;

        if x < flipped_img.width() && flipped_y < flipped_img.height() {
            let new = flipped_img.get_mut(x, flipped_y);
            *new = element.clone();
        }
    }
    flipped_img
}

}
