use std::{fmt, ops::Add};

use crate::colour::{Colour, self};

pub struct ColourString<'a>{
    pub string : String,
    pub bg_colour : &'a Colour,
    pub fg_colour : &'a Colour,
    pub group : &'a str
}

impl<'a> ColourString<'a>{

    pub fn new<'b>(string: &'b str, bg: &'b Colour, fg: &'b Colour, group : Option< &'b str>) -> ColourString<'b>{
        ColourString{
            string : string.to_owned(),
            bg_colour : bg,
            fg_colour : fg,
            group : group.unwrap_or("")
        }
    }

    pub fn change_bg_colour(mut self, colour : &'a Colour){
        self.bg_colour = colour;
    }

    pub fn change_fg_colour(mut self, colour : &'a Colour){
        self.fg_colour = colour;
    }

    pub fn as_true_colour(&self) -> String{
        let colour_part = format!{
            "\x1B[38;2;{};{};{}m\x1B[48;2;{};{};{}m",
            &self.bg_colour.r,
            &self.bg_colour.b,
            &self.bg_colour.g,
            &self.fg_colour.r,
            &self.fg_colour.g,
            &self.fg_colour.b,
        };
        
        if self.string.ends_with("\n"){
                let mut string = self.string.to_owned();
                string.pop();
                return format!{
                    "{}{}\x1B[0m\n",
                    colour_part,
                    string
                };  
        }
        format!{
            "{}{}\x1B[0m",
            colour_part,
            &self.string
        }
    }

    pub fn true_length(&self) -> usize{
        self.as_true_colour().len()
    }

    pub fn len(&self) -> usize{
        self.string.len().to_owned()
    }

}

impl fmt::Display for ColourString<'_>{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f, "{}", self.as_true_colour()
        )
    }
}

impl<'a> Add for ColourString<'a>{

    fn add(self, string: ColourString<'a>) -> ColourString<'a>{
        ColourString { 
            string: self.string + &string.string,
            bg_colour: self.bg_colour,
            fg_colour: self.fg_colour, 
            group : self.group
        }
    }

    type Output = ColourString<'a>;
}