use std::fmt;

use super::char_pixel::CharPixel;

pub struct CharScreen {
    pub hight : u16,
    pub width : u16,
    pub screen : Vec<Vec<CharPixel>>
}

impl CharScreen {

    pub fn new(temp_hight : u16, temp_width : u16) -> CharScreen{
        CharScreen {
            hight : temp_hight,
            width : temp_width,
            screen : Vec::new(),
        }

    }

    pub fn new_from_vecs(arr : Vec<Vec<(u8, u8, u8)>>) -> CharScreen{
        //TODO add checks to make sure the arrays are of an even number
        let h: u16 = arr.len() as u16; 
        let w: u16 = arr[1].len() as u16; 

        let mut charscreen : Vec<Vec<CharPixel>> = vec![Vec::new()];

        for i in 0..(h/2){
            charscreen.push(Vec::new());
            for j in 0..(w){
                charscreen[i as usize].push(
                    CharPixel::new_from_tuple(
                        arr[(i*2) as usize][j as usize], 
                        arr[(i*2+1) as usize][j as usize]
                    )
                )
                
            }
            
        }
        CharScreen { 
            hight: h, 
            width: w, 
            screen: charscreen 
        }
    }

    //[TODO] update for relative printing
    pub fn string(&self) -> String{
        let mut s : String = String::new();

        for i in self.screen.iter(){
            for j in i.iter(){
                s.push_str(j.asTrueColour().as_str())
            }
            s.push_str(&"\n");
        }
        s
    }


}

impl fmt::Display for CharScreen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f, "{}", self.string()
        )
    }
}