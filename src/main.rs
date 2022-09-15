extern crate termsize;
extern crate image;

mod graphics_screen;
mod cursor_controls;
mod colour_string;
mod colour;
mod images;

use colour_string::{ColourString::ColourString, ColourString_Vec::ColourStringVec};
use colour::Colour;

fn main() {

    let mut black = Colour::black();
    let mut white = Colour::white();
    let mut pink = Colour::pink();

    let strings = vec![
        ColourString::new("hello world\n", &white, &black, None),
        ColourString::new("hello world", &white, &pink, None)
    ];

    let colour_strings : ColourStringVec = ColourStringVec::from_vec(strings);

    println!("{}", colour_strings.string_as_true_colour());
}




    // let frame_time = time::Duration::from_nanos(2);

    // let vid_vec : Vec<CharScreen> = images.iter().map(|x| CharScreen::new_from_vecs(image_to_pixel_map(x))).collect();

    // cursor_controls::hide();

    // for i in &vid_vec{
    //     cursor_controls::home();
    //     println!("{}", i);
        

    // }
    // print!("\x1B[2J");
    // cursor_controls::home();
    // cursor_controls::show();



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


    // let img = include_bytes!("test3.png");

    // let img2 : CharScreen = CharScreen::new_from_vecs(&image_to_pixel_map(img));
    // println!("{}", img2);
    
    // let mut vecs = Vec::new();

    // for i in 1..30{
    //     let mut vecs_row = Vec::new();
    //     let mut vecs_row2 = Vec::new();
    //     for j in 1..15{
    //         vecs_row.push((255, 0, 255));
    //         vecs_row.push((0, 0, 0));
    //         vecs_row2.push((0, 0, 0));
    //         vecs_row2.push((255, 0, 255));
            
    //     }
    //     vecs.push(vecs_row);
    //     vecs.push(vecs_row2);
    // }

    // let mut cs = CharScreen::new_from_vecs(&vecs);

    // println!("{}", cs)
