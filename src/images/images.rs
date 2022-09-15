use std::collections::VecDeque;
use crate::graphics_screen::char_pixel_screen::CharScreen;


//doesnt work for pngs and only tested with pngs
pub fn image_from_path(path : &str) -> CharScreen{
    let image = image::open(path).unwrap();
    let size = ((&image.height()).to_owned(), (&image.width()).to_owned());
    let im = image::load_from_memory(image.as_bytes());
    let mut rgb : VecDeque<u8> = VecDeque::from(im.unwrap()
                                                  .to_rgb8()
                                                  .into_vec()
                                                );
    let mut arr : Vec<Vec<(u8, u8, u8)>> = Vec::new();
    for i in 0..(size.1){
        arr.push(Vec::new());
        for _ in 0..(size.0){
            arr[i as usize].push(
                (rgb.pop_front().unwrap_or(0),
                 rgb.pop_front().unwrap_or(0),
                 rgb.pop_front().unwrap_or(0)
                )
            )
        }
    }

    CharScreen::new_from_vecs(arr) 
}

fn image_to_pixel_map(bytes : &[u8]) -> Vec<Vec<(u8, u8, u8)>>{
    let im = image::load_from_memory(bytes).unwrap().to_rgb8();
    let dia_im = (200, 157);
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
