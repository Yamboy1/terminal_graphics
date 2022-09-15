use std::fmt;
//rust escape string \x1B[
use crate::colour::Colour;


pub struct CharPixel{
    //ESC[48;2;{r};{g};{b}m
    top_colour: Colour,
    //ESC[38;2;{r};{g};{b}m 
    bottom_colour : Colour,
    pixel : char
}

impl CharPixel{

    pub fn new_from_tuple(f : (u8, u8, u8), b:(u8, u8, u8)) -> CharPixel{
        CharPixel{
            top_colour : Colour::new(f.0, f.1, f.2),
            bottom_colour : Colour::new(b.0, b.1, b.2),
            pixel : '▄'.to_owned()
        }
    }

    pub fn new(fr: u8, fg: u8, fb: u8, br: u8, bg: u8, bb: u8) -> CharPixel{
        CharPixel{
            top_colour : Colour::new(fr, fg, fb),
            bottom_colour : Colour::new(br, bg, bb),
            pixel : '▄'.to_owned()
        }
    }

    pub fn new_from_colour(fg : Colour, bg :Colour) -> CharPixel{
        CharPixel{
            top_colour : fg,
            bottom_colour : bg,
            pixel : '▄'.to_owned()
        }
    }

    pub fn new_blank() -> CharPixel{
        CharPixel{
            top_colour : Colour::black(),
            bottom_colour : Colour::pink(),
            pixel : '▄'.to_owned()
        }
    }

    pub fn asTrueColour(&self) -> String{
        format!{
            //foreground     //background        //str //reset
            "\x1B[38;2;{};{};{}m\x1B[48;2;{};{};{}m{}\x1B[0m",
            &self.bottom_colour.r,
            &self.bottom_colour.g,
            &self.bottom_colour.b,
            &self.top_colour.r,
            &self.top_colour.g,
            &self.top_colour.b,
            &self.pixel

        }
    }

    pub fn as_256_colour(&self) -> String{
        unimplemented!()
    }

    pub fn as_8_colour(&self) -> String{
        unimplemented!()
    }

    pub fn as_2_colour(&self) -> String{
        unimplemented!()
    }


    pub fn debug_string(&self) -> (String, String){
        (
            format!("({}, {}, {})", 
                    self.top_colour.r,
                    self.top_colour.b,
                    self.top_colour.g),
            format!("({}, {}, {})", 
                    self.bottom_colour.r,
                    self.bottom_colour.b,
                    self.bottom_colour.g)
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