use array2::Array2;
use csc411_image::{Read, Rgb, RgbImage, Write};
use std::process::exit;
mod transform;
use transform::*;

/// Setup CLAP command line args
use clap::Parser;
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    // Flip
    #[clap(long = "flip", required = false)]
    flip: Option<String>,
    // Rotation
    #[clap(long = "rotate")]
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

/// Parses the arguments 
/// Read the RGB image and create a pixiels vec
/// Based on the arguments, determine which transformation to perform
/// Write the transformed image
fn main() {
    let args = Args::parse();
    let rotate: Option<u32> = args.rotate;
    let filename = args.filename;
    let transpose = args.transpose;
    let col_major = args.col_major;
    let flip = args.flip;

    // Set rgb image with th filename
    let img = RgbImage::read(filename.as_deref()).unwrap();

    // Get the width and height from the image
    let height2 = img.height.try_into().unwrap();
    let width2 = img.width.try_into().unwrap();

    // Create a vec of the pixels
    let pixels_vec: Vec<Rgb> = img.pixels.clone();

    let img = Array2::from_row_major(width2, height2, pixels_vec).unwrap();

    // Run the rotate_image function, flip_image function, or tranpose_image function if the given argument is present
    // Write the transformed image -- the output of those functions.
    if let Some(degrees) = rotate {
        let transformed_image = rotate_image(&img, degrees, col_major);
        let _ = RgbImage::write(&transformed_image, None);
    } else if let Some(flip_direction) = flip {
        let transformed_image = flip_image(&img, &flip_direction, col_major);
        let _ = RgbImage::write(&transformed_image, None);
    } else if transpose {
        let transformed_image = transpose_image(&img, col_major);
        let _ = RgbImage::write(&transformed_image, None);
    }
}


/// Create an RGB image based off an Array2
/// Takes a with, height, denominatior (255), and pixels vec of RGB pixels
/// 
/// # Arguments
///
/// * `arr`: an Array2 instance of type RGB, an RGB image pixel
fn from_array2(arr: &Array2<Rgb>) -> RgbImage {
    let width = arr.width() as u32;
    let height = arr.height() as u32;
    let denominator = 255;
    let pixels: Vec<Rgb> = arr.data().to_vec();

    RgbImage {
        width,
        height,
        denominator,
        pixels,
    }
}


/// Determines which rotoate function to perform based on the degrees entered 
/// and if it should be row or column-major.
/// Exits with an error if no matching angle can be found.
/// 
/// # Arguments
/// 
/// * `img`: an Array2 ref instance of type RGB
/// * `degrees`: a u32 for the degree to rotate
/// * `col_major`: a bool to determine if it should rotate in column-major order
///
/// # Returns
/// * An `RgbImage` from the result of the appropriote rotate function.
fn rotate_image(img: &Array2<Rgb>, degrees: u32, col_major: bool) -> RgbImage {
    let rotated_img = match degrees {
        0 => {
            if col_major {
                rotate0_cmo(img)
            } else {
                rotate0_rmo(img)
            }
        }
        90 => {
            if col_major {
                rotate90_cmo(img)
            } else {
                rotate90_rmo(img)
            }
        }
        180 => {
            if col_major {
                rotate180_cmo(img)
            } else {
                rotate180_rmo(img)
            }
        }
        270 => {
            if col_major {
                rotate270_cmo(img)
            } else {
                rotate270_rmo(img)
            }
        }
        _ => exit(1),
    };

    from_array2(&rotated_img)
}


/// Determines which flip function to perform based either horizontal or vertical
/// and if it should be row or column-major.
/// Exits with an error if no matching angle can be found.
/// 
/// # Arguments
/// 
/// * `img`: an Array2 ref instance of type RGB
/// * `flip_direction`: a ref str which should equal either horizontal or vertcial
/// * `col_major`: a bool to determine if it should rotate in column-major order
///
/// # Returns
/// * An `RgbImage` from the result of the appropriote flip function.
fn flip_image(img: &Array2<Rgb>, flip_direction: &str, col_major: bool) -> RgbImage {
    let flipped_img = match flip_direction {
        "horizontal" => {
            if col_major {
                flip_horizontal_cmo(img)
            } else {
                flip_horizontal_rmo(img)
            }
        }
        "vertical" => {
            if col_major {
                flip_vertical_cmo(img)
            } else {
                flip_vertical_rmo(img)
            }
        }
        _ => exit(1),
    };

    from_array2(&flipped_img)
}

/// Performs the transpose transformation on the image, as the function will only be called 
/// when that argument is present, with no other modifiers.
/// 
/// # Arguments
/// 
/// * `img`: an Array2 ref instance of type RGB
/// * `col_major`: a bool to determine if it should rotate in column-major order
///
/// # Returns
/// * An `RgbImage` from the result of the appropriote transpose function.
fn transpose_image(img: &Array2<Rgb>, col_major: bool) -> RgbImage {
    let transposed_img = if col_major {
        transpose_cmo(img)
    } else {
        transpose_rmo(img)
    };
    from_array2(&transposed_img)
}
