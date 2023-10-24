use array2::Array2;
use csc411_image::{GrayImage, Read};
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


    let img = GrayImage::read(input.as_deref()).unwrap();
    // get our values from grayimage to the correct types for our structure
    let height2 = img.height.try_into().unwrap();
    let width2 = img.width.try_into().unwrap();
    let usize_vec: Vec<usize> = img.pixels.iter().map(|gray| gray.value as usize).collect();
    let try_2 = Array2::from_col_major(width2, height2, usize_vec).unwrap();
    let mut try_3 = Array2::blank_state(height2, width2, 0);
    
    if let Some(second_arg) = input2 {
    if second_arg == "rotate90" {
     
            
         
        

            for (x, y, &element) in try_2.iter_row_major() {
                let rotated_x= try_2.height() - y - 1;
                let rotated_y = x;
        
                if rotated_x < try_3.width() && rotated_y < try_3.height() {
                    let new = try_3.get_mut(rotated_x, rotated_y);
                    *new = element.clone();
                    
                  }
                }
        
            for (x, y, &element) in try_3.iter_row_major(){
                println!("x:{}y:{}, {}", x,y,element)
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
}
}
   
  
   

 

