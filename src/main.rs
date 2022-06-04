extern crate termsize;
extern crate image;
use std::collections::VecDeque;

use std::{thread, time};


mod graphics_screen;
mod cursor_controls;


use graphics_screen::{char_pixel::CharPixel, char_pixel_screen::CharScreen};

fn image_to_pixel_map(bytes : &[u8]) -> Vec<Vec<(u8, u8, u8)>>{
    let im = image::load_from_memory(bytes).unwrap().to_rgb8();
    let dia_im = (160, 120);
    let mut rgb : VecDeque<u8> = VecDeque::from(im.into_vec());
    let mut arr : Vec<Vec<(u8, u8, u8)>> = Vec::new();

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

    arr
}

//\x1B[H



fn main() {
    print!("\x1B[2J");

    let image_amout = 900;

    let mut images : Vec<&[u8]> = Vec::new();


    let mut vid_vec : Vec<CharScreen> = Vec::new();

    let frame_time = time::Duration::from_millis(2);

    for i in images{
        vid_vec.push(CharScreen::new_from_vecs(image_to_pixel_map(i)));
    }


    cursor_controls::hide();

    for i in vid_vec{
        cursor_controls::home();
        println!("{}", i);
        thread::sleep(frame_time)

    }
    print!("\x1B[2J");
    cursor_controls::home();
    cursor_controls::show();


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
