use array2::Array2;
use csc411_image::Rgb;
use std::time::Instant;

/// Map the elements from an `Array2` instance to a new instance in row-major order, which represents
/// rotating an image 90 degrees clockwise.
/// It construct a new Array2 of type RGB, an RGB pixel, as a blank state Array2 set to all 0s.
/// Width and height are determined by the specific transformation.
/// It times the image transformation, printing the reuslts to std error.
/// It then computes the mapped value in the new Array2 for the specific formula for the transformation.
/// Rotating 90 degrees is (x,y) -> (h - y - 1, x), of an image w * h in size.
/// 
/// # Arguments
/// 
/// * `img`: an Array2 ref instance of type RGB
///
/// # Returns
/// * An `RgbImage` from the result of the appropriote rotate function.
pub fn rotate90_rmo(img: &Array2<Rgb>) -> Array2<Rgb> {
    // Create a new Array2. The width and height values are swapped for the new image.
    // Default value of 0 in every element.
    let mut rotated_img = Array2::blank_state(img.height(), img.width(), img.get(0, 0).clone());

    // Start timing the rotation
    let now = Instant::now();

    // Iterate in row-major order through the elements
    for (x, y, &ref element) in img.iter_row_major() {
         // Compute the new pixel location
        let rotated_x = img.height() - y - 1;

        if rotated_x < rotated_img.width() && x < rotated_img.height() {
            let new = rotated_img.get_mut(rotated_x, x);
            *new = element.clone();
        }
    }

    // Record the elpased time
    let elapsed = now.elapsed();
    eprintln!("{:.2?}", elapsed);

    rotated_img
}
// All the following functions follow the same format.


/// Map the elements from an `Array2` instance to a new instance in col-major order, which represents
/// rotating an image 90 degrees clockwise.
/// It construct a new Array2 of type RGB, an RGB pixel, as a blank state Array2 set to all 0s.
/// Width and height are determined by the specific transformation.
/// It times the image transformation, printing the freuslts to std error.
/// It then computes the mapped value in the new Array2 for the specific formula for the transformation.
/// Rotating 90 degrees is (x,y) -> (h - y - 1, x), of an image w * h in size.
/// 
/// # Arguments
/// 
/// * `img`: an Array2 ref instance of type RGB
///
/// # Returns
/// * An `RgbImage` from the result of the appropriote rotate function.
pub fn rotate90_cmo(img: &Array2<Rgb>) -> Array2<Rgb> {
    let mut rotated_img = Array2::blank_state(img.height(), img.width(), img.get(0, 0).clone());

    let now = Instant::now();

    for (x, y, &ref element) in img.iter_col_major() {
        let rotated_x = img.height() - y - 1;
        let rotated_y = x;

        if rotated_x < rotated_img.width() && rotated_y < rotated_img.height() {
            let new = rotated_img.get_mut(rotated_x, rotated_y);
            *new = element.clone();
        }
    }

    let elapsed = now.elapsed();
    eprintln!("{:.2?}", elapsed);

    rotated_img
}


/// Map the elements from an `Array2` instance to a new instance in row-major order, which represents
/// rotating an image 180 degrees clockwise.
/// It construct a new Array2 of type RGB, an RGB pixel, as a blank state Array2 set to all 0s.
/// Width and height are determined by the specific transformation.
/// It times the image transformation, printing the freuslts to std error.
/// It then computes the mapped value in the new Array2 for the specific formula for the transformation.
/// Rotating 180 degrees is (x,y) -> (w - x - 1, h - y - 1), of an image w * h in size.
/// 
/// # Arguments
/// 
/// * `img`: an Array2 ref instance of type RGB
///
/// # Returns
/// * An `RgbImage` from the result of the appropriote rotate function.
pub fn rotate180_rmo(img: &Array2<Rgb>) -> Array2<Rgb> {
    let mut rotated_img = Array2::blank_state(img.width(), img.height(), img.get(0, 0).clone());

    let now = Instant::now();

    for (x, y, &ref element) in img.iter_row_major() {
        let rotated_x = img.width() - x - 1;
        let rotated_y = img.height() - y - 1;

        if rotated_x < rotated_img.width() && rotated_y < rotated_img.height() {
            let new = rotated_img.get_mut(rotated_x, rotated_y);
            *new = element.clone();
        }
    }

    let elapsed = now.elapsed();
    eprintln!("{:.2?}", elapsed);

    rotated_img
}


/// Map the elements from an `Array2` instance to a new instance in col-major order, which represents
/// rotating an image 180 degrees clockwise.
/// It construct a new Array2 of type RGB, an RGB pixel, as a blank state Array2 set to all 0s.
/// Width and height are determined by the specific transformation.
/// It times the image transformation, printing the freuslts to std error.
/// It then computes the mapped value in the new Array2 for the specific formula for the transformation.
/// Rotating 180 degrees is (x,y) -> (w - x - 1, h - y - 1), of an image w * h in size.
/// 
/// # Arguments
/// 
/// * `img`: an Array2 ref instance of type RGB
///
/// # Returns
/// * An `RgbImage` from the result of the appropriote rotate function.
pub fn rotate180_cmo(img: &Array2<Rgb>) -> Array2<Rgb> {
    let mut rotated_img = Array2::blank_state(img.width(), img.height(), img.get(0, 0).clone());

    let now = Instant::now();

    for (x, y, &ref element) in img.iter_col_major() {
        let rotated_x = img.width() - x - 1;
        let rotated_y = img.height() - y - 1;

        if rotated_x < rotated_img.width() && rotated_y < rotated_img.height() {
            let new = rotated_img.get_mut(rotated_x, rotated_y);
            *new = element.clone();
        }
    }

    let elapsed = now.elapsed();
    eprintln!("{:.2?}", elapsed);

    rotated_img
}


/// Map the elements from an `Array2` instance to a new instance in row-major order, which represents
/// rotating an image 270 degrees clockwise.
/// It construct a new Array2 of type RGB, an RGB pixel, as a blank state Array2 set to all 0s.
/// Width and height are determined by the specific transformation.
/// It times the image transformation, printing the freuslts to std error.
/// It then computes the mapped value in the new Array2 for the specific formula for the transformation.
/// Rotating 270 degrees is (x,y) -> (y, w - x - 1), of an image w * h in size.
/// 
/// # Arguments
/// 
/// * `img`: an Array2 ref instance of type RGB
///
/// # Returns
/// * An `RgbImage` from the result of the appropriote rotate function.
pub fn rotate270_rmo(img: &Array2<Rgb>) -> Array2<Rgb> {
    let mut rotated_img = Array2::blank_state(img.height(), img.width(), img.get(0, 0).clone());

    let now = Instant::now();

    for (x, y, &ref element) in img.iter_row_major() {
        let rotated_y = img.width() - x - 1;

        if y < rotated_img.width() && rotated_y < rotated_img.height() {
            let new = rotated_img.get_mut(y, rotated_y);
            *new = element.clone();
        }
    }

    let elapsed = now.elapsed();
    eprintln!("{:.2?}", elapsed);

    rotated_img
}


/// Map the elements from an `Array2` instance to a new instance in col-major order, which represents
/// rotating an image 270 degrees clockwise.
/// It construct a new Array2 of type RGB, an RGB pixel, as a blank state Array2 set to all 0s.
/// Width and height are determined by the specific transformation.
/// It times the image transformation, printing the freuslts to std error.
/// It then computes the mapped value in the new Array2 for the specific formula for the transformation.
/// Rotating 270 degrees is (x,y) -> (y, w - x - 1), of an image w * h in size.
/// 
/// # Arguments
/// 
/// * `img`: an Array2 ref instance of type RGB
///
/// # Returns
/// * An `RgbImage` from the result of the appropriote rotate function.
pub fn rotate270_cmo(img: &Array2<Rgb>) -> Array2<Rgb> {
    let mut rotated_img = Array2::blank_state(img.height(), img.width(), img.get(0, 0).clone());

    let now = Instant::now();

    for (x, y, &ref element) in img.iter_col_major() {
        let rotated_y = img.width() - x - 1;

        if y < rotated_img.width() && rotated_y < rotated_img.height() {
            let new = rotated_img.get_mut(y, rotated_y);
            *new = element.clone();
        }
    }

    let elapsed = now.elapsed();
    eprintln!("{:.2?}", elapsed);

    rotated_img
}


/// Map the elements from an `Array2` instance to a new instance in row-major order, which represents
/// rotating an image 0 degrees clockwise.
/// It construct a new Array2 of type RGB, an RGB pixel, as a blank state Array2 set to all 0s.
/// Width and height are determined by the specific transformation.
/// It times the image transformation, printing the freuslts to std error.
/// It then computes the mapped value in the new Array2 for the specific formula for the transformation.
/// Rotating 0 degrees doesn't change pxiel position.
/// 
/// # Arguments
/// 
/// * `img`: an Array2 ref instance of type RGB
///
/// # Returns
/// * An `RgbImage` from the result of the appropriote rotate function.
pub fn rotate0_rmo(img: &Array2<Rgb>) -> Array2<Rgb> {
    let mut rotated_img = Array2::blank_state(img.width(), img.height(), img.get(0, 0).clone());

    let now = Instant::now();

    for (x, y, &ref element) in img.iter_row_major() {

        if x < rotated_img.width() && y < rotated_img.height() {
            let new = rotated_img.get_mut(x, y);
            *new = element.clone();
        }
    }

    let elapsed = now.elapsed();
    eprintln!("{:.2?}", elapsed);

    rotated_img
}


/// Map the elements from an `Array2` instance to a new instance in col-major order, which represents
/// rotating an image 0 degrees clockwise.
/// It construct a new Array2 of type RGB, an RGB pixel, as a blank state Array2 set to all 0s.
/// Width and height are determined by the specific transformation.
/// It times the image transformation, printing the freuslts to std error.
/// It then computes the mapped value in the new Array2 for the specific formula for the transformation.
/// Rotating 0 degrees doesn't change pxiel position.
/// 
/// # Arguments
/// 
/// * `img`: an Array2 ref instance of type RGB
///
/// # Returns
/// * An `RgbImage` from the result of the appropriote rotate function.
pub fn rotate0_cmo(img: &Array2<Rgb>) -> Array2<Rgb> {
    let mut rotated_img = Array2::blank_state(img.width(), img.height(), img.get(0, 0).clone());

    let now = Instant::now();

    for (x, y, &ref element) in img.iter_col_major() {
        let rotated_x = x;
        let rotated_y = y;

        if rotated_x < rotated_img.width() && rotated_y < rotated_img.height() {
            let new = rotated_img.get_mut(rotated_x, rotated_y);
            *new = element.clone();
        }
    }

    let elapsed = now.elapsed();
    eprintln!("{:.2?}", elapsed);

    rotated_img
}


/// Map the elements from an `Array2` instance to a new instance in row-major order, which represents
/// flipping an image horizontally
/// It construct a new Array2 of type RGB, an RGB pixel, as a blank state Array2 set to all 0s.
/// Width and height are determined by the specific transformation.
/// It times the image transformation, printing the freuslts to std error.
/// It then computes the mapped value in the new Array2 for the specific formula for the transformation.
/// Flipping horizontally is (x,y) -> (w - x - 1, y), of an image w * h in size.
/// 
/// # Arguments
/// 
/// * `img`: an Array2 ref instance of type RGB
///
/// # Returns
/// * An `RgbImage` from the result of the appropriote rotate function.
pub fn flip_horizontal_rmo(img: &Array2<Rgb>) -> Array2<Rgb> {
    let mut flipped_img: Array2<Rgb> =
        Array2::blank_state(img.width(), img.height(), img.get(0, 0).clone());

    let now = Instant::now();

    for (x, y, &ref element) in img.iter_row_major() {
        let flipped_x = img.width() - x - 1;

        if flipped_x < flipped_img.width() && y < flipped_img.height() {
            let new = flipped_img.get_mut(flipped_x, y);
            *new = element.clone();
        }
    }

    let elapsed = now.elapsed();
    eprintln!("{:.2?}", elapsed);

    flipped_img
}


/// Map the elements from an `Array2` instance to a new instance in col-major order, which represents
/// flipping an image horizontally
/// It construct a new Array2 of type RGB, an RGB pixel, as a blank state Array2 set to all 0s.
/// Width and height are determined by the specific transformation.
/// It times the image transformation, printing the freuslts to std error.
/// It then computes the mapped value in the new Array2 for the specific formula for the transformation.
/// Flipping horizontally is (x,y) -> (w - x - 1, y), of an image w * h in size.
/// 
/// # Arguments
/// 
/// * `img`: an Array2 ref instance of type RGB
///
/// # Returns
/// * An `RgbImage` from the result of the appropriote rotate function.
pub fn flip_horizontal_cmo(img: &Array2<Rgb>) -> Array2<Rgb> {
    let mut flipped_img: Array2<Rgb> =
        Array2::blank_state(img.width(), img.height(), img.get(0, 0).clone());

    let now = Instant::now();

    for (x, y, &ref element) in img.iter_col_major() {
        let flipped_y = img.width() - y - 1;

        if x < flipped_img.width() && flipped_y < flipped_img.height() {
            let new = flipped_img.get_mut(x, flipped_y);
            *new = element.clone();
        }
    }

    let elapsed = now.elapsed();
    eprintln!("{:.2?}", elapsed);

    flipped_img
}


/// Map the elements from an `Array2` instance to a new instance in row-major order, which represents
/// flipping an image vertically
/// It construct a new Array2 of type RGB, an RGB pixel, as a blank state Array2 set to all 0s.
/// Width and height are determined by the specific transformation.
/// It times the image transformation, printing the freuslts to std error.
/// It then computes the mapped value in the new Array2 for the specific formula for the transformation.
/// Flipping vertically is (x,y) -> (x, h - y - 1), of an image w * h in size.
/// 
/// # Arguments
/// 
/// * `img`: an Array2 ref instance of type RGB
///
/// # Returns
/// * An `RgbImage` from the result of the appropriote rotate function.
pub fn flip_vertical_rmo(img: &Array2<Rgb>) -> Array2<Rgb> {
    let mut flipped_img: Array2<Rgb> =
        Array2::blank_state(img.width(), img.height(), img.get(0, 0).clone());

    let now = Instant::now();

    for (x, y, &ref element) in img.iter_row_major() {
        let flipped_y = img.height() - y - 1;

        if x < flipped_img.width() && flipped_y < flipped_img.height() {
            let new = flipped_img.get_mut(x, flipped_y);
            *new = element.clone();
        }
    }

    let elapsed = now.elapsed();
    eprintln!("{:.2?}", elapsed);

    flipped_img
}


/// Map the elements from an `Array2` instance to a new instance in col-major order, which represents
/// flipping an image vertically
/// It construct a new Array2 of type RGB, an RGB pixel, as a blank state Array2 set to all 0s.
/// Width and height are determined by the specific transformation.
/// It times the image transformation, printing the freuslts to std error.
/// It then computes the mapped value in the new Array2 for the specific formula for the transformation.
/// Flipping vertically is (x,y) -> (x, h - y - 1), of an image w * h in size.
/// 
/// # Arguments
/// 
/// * `img`: an Array2 ref instance of type RGB
///
/// # Returns
/// * An `RgbImage` from the result of the appropriote rotate function.
pub fn flip_vertical_cmo(img: &Array2<Rgb>) -> Array2<Rgb> {
    let mut flipped_img: Array2<Rgb> =
        Array2::blank_state(img.width(), img.height(), img.get(0, 0).clone());

    let now = Instant::now();

    for (x, y, &ref element) in img.iter_col_major() {
        let flipped_y = img.height() - y - 1;

        if x < flipped_img.width() && flipped_y < flipped_img.height() {
            let new = flipped_img.get_mut(x, flipped_y);
            *new = element.clone();
        }
    }

    let elapsed = now.elapsed();
    eprintln!("{:.2?}", elapsed);

    flipped_img
}


/// Map the elements from an `Array2` instance to a new instance in row-major order, which represents
/// transposing an image diagonally from upper-left to bottom-right.
/// It construct a new Array2 of type RGB, an RGB pixel, as a blank state Array2 set to all 0s.
/// Width and height are determined by the specific transformation.
/// It times the image transformation, printing the freuslts to std error.
/// It then computes the mapped value in the new Array2 for the specific formula for the transformation.
/// Transpose is (x,y) -> (y, x), of an image w * h in size.
/// 
/// # Arguments
/// 
/// * `img`: an Array2 ref instance of type RGB
///
/// # Returns
/// * An `RgbImage` from the result of the appropriote rotate function.
pub fn transpose_rmo(img: &Array2<Rgb>) -> Array2<Rgb> {
    let mut transposed_img: Array2<Rgb> =
        Array2::blank_state(img.height(), img.width(), img.get(0, 0).clone());
    
    let now = Instant::now();

    for (x, y, &ref element) in img.iter_row_major() {
        if y < transposed_img.width() && x < transposed_img.height() {
            let new = transposed_img.get_mut(y, x);
            *new = element.clone();
        }
    }

    let elapsed = now.elapsed();
    eprintln!("{:.2?}", elapsed);

    transposed_img
}


/// Map the elements from an `Array2` instance to a new instance in col-major order, which represents
/// transposing an image diagonally from upper-left to bottom-right.
/// It construct a new Array2 of type RGB, an RGB pixel, as a blank state Array2 set to all 0s.
/// Width and height are determined by the specific transformation.
/// It times the image transformation, printing the freuslts to std error.
/// It then computes the mapped value in the new Array2 for the specific formula for the transformation.
/// Transpose is (x,y) -> (y, x), of an image w * h in size.
/// 
/// # Arguments
/// 
/// * `img`: an Array2 ref instance of type RGB
///
/// # Returns
/// * An `RgbImage` from the result of the appropriote rotate function.
pub fn transpose_cmo(img: &Array2<Rgb>) -> Array2<Rgb> {
    let mut transposed_img: Array2<Rgb> =
        Array2::blank_state(img.height(), img.width(), img.get(0, 0).clone());
    
    let now = Instant::now();

    for (x, y, &ref element) in img.iter_col_major() {
        if y < transposed_img.width() && x < transposed_img.height() {
            let new = transposed_img.get_mut(y, x);
            *new = element.clone();
        }
    }

    let elapsed = now.elapsed();
    eprintln!("{:.2?}", elapsed);

    transposed_img
}
