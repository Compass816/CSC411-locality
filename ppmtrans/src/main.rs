use array2::Array2;
use csc411_image::{Read, Rgb, RgbImage, Write};
use std::env;
use std::process;

/*

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
// Transposition
#[clap(long = "transpose")]
transpose: bool,
}

*/
fn main() {
    let input = env::args().nth(1);
    let input2 = env::args().nth(2);
    let input3 = env::args().nth(3);

    let img = RgbImage ::read(input.as_deref()).unwrap();
    // get our values from grayimage to the correct types for our structure
    let height2 = img.height.try_into().unwrap();
    let width2 = img.width.try_into().unwrap();

    // Got to figure out how we're representing RGB
    let usize_vec: Vec<usize> = img.pixels.iter().map(|rgb| rgb.red as usize).collect();
    let try_2 = Array2::from_col_major(width2, height2, usize_vec).unwrap();

    if let Some(second_arg) = input2 {
        if second_arg == "rotate90" {
            let try_3 = rotate90_rmo(&try_2);

            for (x, y, &element) in try_3.iter_row_major() {
                println!("x:{}y:{}, {}", x, y, element)
            }
        }
    }
}

/*     if second_arg == "rotate180" {
        if let Some(value) = &input3 {
            if value == "col-major" {
            for (x, y, &element) in try_2.iter_col_major() {
                let rotated_x = width2 - 1 - x;
                let rotated_y = height2 - 1 - y;

                if rotated_x < try_3.width() && rotated_y < try_3.height() {
                    let new = try_3.get_mut(rotated_x, rotated_y);
                    if let Ok(new) = try_3.get_mut(rotated_x, rotated_y) {
                        *new = element.clone();
                    } else {
                        // Handle the case where the index is out of bounds.
                    }
                  }
                }

            for (x, y, &element) in try_3.iter_row_major(){
                println!("x:{}y:{}, {}", x,y,element)
            }


         process::exit(0);

        }
    }



        for (x, y, &element) in try_2.iter_row_major() {
            let rotated_x = width2 - 1 - x;
            let rotated_y = height2 - 1 - y;
            println!("{}", rotated_x);

            if rotated_x < try_3.width() && rotated_y < try_3.height() {
                let new = try_3.get_mut(rotated_x, rotated_y);
                if let Ok(new) = try_3.get_mut(rotated_x, rotated_y) {
                    *new = element.clone();
                } else {
                    // Handle the case where the index is out of bounds.
                }
             }
            }

        for (x, y, &element) in try_3.iter_row_major(){
            println!("x:{}y:{}, {}", x,y,element)

        }


    }
}
*/

fn rotate90_rmo(img: &Array2<usize>) -> Array2<usize> {
    let mut rotated_img = Array2::blank_state(img.height(), img.width(), 0);

    for (x, y, &element) in img.iter_row_major() {
        let rotated_x = img.height() - y - 1;
        let rotated_y = x;

        if rotated_x < rotated_img.width() && rotated_y < rotated_img.height() {
            let new = rotated_img.get_mut(rotated_x, rotated_y);
            *new = element.clone();
        }
    }

    rotated_img
}

fn rotate180_rmo(img: &Array2<usize>) -> Array2<usize> {
    let mut rotated_img = Array2::blank_state(img.width(), img.height(), 0);

    for (x, y, &element) in img.iter_row_major() {
        let rotated_x = img.width() - x - 1;
        let rotated_y = img.height() - y - 1;

        if rotated_x < rotated_img.width() && rotated_y < rotated_img.height() {
            let new = rotated_img.get_mut(rotated_x, rotated_y);
            *new = element.clone();
        }
    }

    rotated_img
}

fn rotate270_rmo(img: &Array2<usize>) -> Array2<usize> {
    let mut rotated_img = Array2::blank_state(img.height(), img.width(), 0);

    for (x, y, &element) in img.iter_row_major() {
        let rotated_x = y;
        let rotated_y = img.width() - x - 1;

        if rotated_x < rotated_img.width() && rotated_y < rotated_img.height() {
            let new = rotated_img.get_mut(rotated_x, rotated_y);
            *new = element.clone();
        }
    }

    rotated_img
}

fn rotate0_rmo(img: &Array2<usize>) -> Array2<usize> {
    img;
}

fn flip_horizontal_rmo(img: &Array2<usize>) -> Array2<usize> {
    let mut flipped_img = Array2::blank_state(img.width(), img.height(), 0);

    for (x, y, &element) in img.iter_row_major() {
        let flipped_y = img.width() - y - 1;

        if x < flipped_img.width() && flipped_y < flipped_img.height() {
            let new = flipped_img.get_mut(x, flipped_y);
            *new = element.clone();
        }
    }

    flipped_img
}

fn flipped_vertical(img: &Array2<usize>) -> Array2<usize> {
    let mut flipped_img = Array2::blank_state(img.width(), img.height(), 0);

    for (x, y, &element) in img.iter_row_major() {
        let flipped_x = img.height() - x - 1;

        if flipped_x < flipped_img.width() && y < flipped_img.height() {
            let new = flipped_img.get_mut(flipped_x, y);
            *new = element.clone();
        }
    }

    flipped_img
}

fn transpose_rmo(img: &Array2<usize>) -> Array2<usize> {
    let mut transposed_img: Array2<usize> = Array2::blank_state(img.width(), img.height(), 0);

    for (x, y, &element) in img.iter_row_major() {

        if x < transposed_img.width() && y < transposed_img.height() {
            let new = transposed_img.get_mut(y, x);
            *new = element.clone();
        }
    }

    transposed_img
}