extern crate termsize;
extern crate image;
use std::collections::VecDeque;


mod graphics_screen;

use graphics_screen::{char_pixel::CharPixel, char_pixel_screen::CharScreen};

fn main() {
    
    let path = "src\\test4.png";

    let im = image::open(path).unwrap().to_rgb8();
    let dia_im = image::image_dimensions(path).unwrap();
    let mut rgb : VecDeque<u8> = VecDeque::from(im.into_vec());
    let mut arr : Vec<Vec<(u8, u8, u8)>> = Vec::new();
    println!("({}, {}, {})", rgb[0], rgb[1], rgb[2]);

    for i in 0..(dia_im.1){
        arr.push(Vec::new());
        for _ in 0..(dia_im.0){
            arr[i as usize].push(
                (rgb.pop_front().unwrap_or(0),
                 rgb.pop_front().unwrap_or(0),
                 rgb.pop_front().unwrap_or(0)
                )
            )
        }
    }
    let c : CharScreen = CharScreen::new_from_vecs(arr);

    println!("{}", c);
}



// makes a checkerboard
    // let mut arr : Vec<Vec<(u8, u8, u8)>> = vec![Vec::new()];
        
    // for i in 0..(64){
    //     arr.push(Vec::new());
    //     arr.push(Vec::new());
    //     for _ in 0..(64){
    //         arr[(i*2) as usize].push((0, 0, 0));
    //         arr[(i*2) as usize].push((255, 255, 255));
    //         arr[(i*2+1) as usize].push((255, 255, 255));
    //         arr[(i*2+1) as usize].push((0, 0, 0));
    //     }
        
    // }

    //makes a gradint
    // let mut r : u8 = 0;    
    // let mut g : u8 = 0;
    // let mut b : u8 = 0;

    // for i in 0..64{
    //     arr.push(Vec::new());
    //     b = 0;
    //     for _ in 0..64{
    //         arr[i].push((255, b, g));
    //         b += 3;
    //     }
    //     g += 3;
    // }
