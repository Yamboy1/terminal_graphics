

use super::char_pixel_screen::CharScreen;


pub struct PixelVec{
    pub map : Vec<Vec<(u8, u8, u8)>>,
    pub hight : u16,
    pub width : u16,
}

impl PixelVec{

    pub fn new(p_hight: u16, p_width: u16) -> PixelVec
        {
            PixelVec { 
                map: Vec::new(),
                hight: p_hight,
                width: p_width
            }
        }

    pub fn into_char_screen(self) -> CharScreen{
        CharScreen::new_from_vecs(self.map)
    }




}