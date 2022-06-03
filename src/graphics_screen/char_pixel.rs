use std::fmt;

//rust escape string \x1B[

pub struct CharPixel{
    //ESC[48;2;{r};{g};{b}m
    top_colour: (u8, u8, u8),
    //ESC[38;2;{r};{g};{b}m 
    bottom_colour : (u8, u8, u8),
    pixel : String
}

impl CharPixel{

    // pub fn new(fr: Option<u8>, fg: Option<u8>, fb: Option<u8>, br: Option<u8>, bg: Option<u8>, bb: Option<u8>) -> PixelTrueColour{
    //     PixelTrueColour{
    //         top_colour : (fr.unwrap_or(255), fg.unwrap_or(255), fb.unwrap_or(255)),
    //         bottom_colour : (br.unwrap_or(255), bg.unwrap_or(255), bb.unwrap_or(255)),
    //         pixel : "▄".to_owned()
    //     }
    // }

    pub fn newfromtuple(f : (u8, u8, u8), b:(u8, u8, u8)) -> CharPixel{
        CharPixel{
            top_colour : f,
            bottom_colour : b,
            pixel : "▄".to_owned()
        }
    }

    pub fn new(fr: u8, fg: u8, fb: u8, br: u8, bg: u8, bb: u8) -> CharPixel{
        CharPixel{
            top_colour : (fr, fg, fb),
            bottom_colour : (br, bg, bb),
            pixel : "▄".to_owned()
        }
    }


    pub fn new_blank() -> CharPixel{
        CharPixel{
            top_colour : (255, 255, 255),
            bottom_colour : (255, 0, 255),
            pixel : "▄".to_owned()
        }
    }

    pub fn asTrueColour(&self) -> String{
        format!{
            //foreground     //background   //str //reset
            "\x1B[38;2;{};{};{}m\x1B[48;2;{};{};{}m{}\x1B[0m",
            &self.top_colour.0,
            &self.top_colour.1,
            &self.top_colour.2,
            &self.bottom_colour.0,
            &self.bottom_colour.1,
            &self.bottom_colour.2,
            &self.pixel

        }
    }

    pub fn as256Colour(&self) -> String{
        unimplemented!()
    }

    pub fn as8colour(&self) -> String{
        unimplemented!()
    }

    pub fn as2Colour(&self) -> String{
        unimplemented!()
    }


    pub fn debug_string(&self) -> (String, String){
        (
            format!("({}, {}, {})", 
                    self.top_colour.0,
                    self.top_colour.1,
                    self.top_colour.2),
            format!("({}, {}, {})", 
                    self.bottom_colour.0,
                    self.bottom_colour.1,
                    self.bottom_colour.2)
        )
    }
}

impl fmt::Display for CharPixel{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f, "{}", self.asTrueColour()
        )
    }
}