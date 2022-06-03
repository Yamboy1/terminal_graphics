use std::fmt;

use crate::char_pixel::CharPixel;

struct CharScreen {
    hight : u16,
    width : u16,
    screen : Vec<Vec<CharPixel>>
}

impl  CharScreen {

    pub fn new(temp_hight : u16, temp_width : u16) -> CharScreen{
        let sc : Vec<Vec<CharPixel>> = vec![Vec::new()];
        CharScreen {
            hight : temp_hight,
            width : temp_width,
            screen : sc,
        }

    }

    pub fn string(&self) -> String{
        let mut s : String = String::new();

        for i in self.screen.iter(){
            for j in i.iter(){
                s.push_str(j.asTrueColour().as_str())
            }
        }
        s
    }


}






impl   fmt::Display for CharScreen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f, "{}", self.string()
        )
    }
}