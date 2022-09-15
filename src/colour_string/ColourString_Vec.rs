use super::ColourString::ColourString;
use super::Colour;





pub struct ColourStringVec <'a> {
    strings : Vec<ColourString<'a>>,
}


impl<'a> ColourStringVec<'a> {

    pub fn new<'b>() -> ColourStringVec<'b>{
        ColourStringVec{
            strings : Vec::new(),
        }
    }

    pub fn from_vec(vecs : Vec<ColourString>) -> ColourStringVec{
        ColourStringVec { strings : vecs}
    }

    // yes hashmaps would make this faster although the amount of checks 
    // will be like 1000 at the max so its not really worth it since
    // the strings still need to be ordered
    pub fn change_bg_group_colour(self, group : &str, colour : Colour) -> () {
        for string in self.strings {
            if string.group == group{
                string.change_bg_colour(&colour)
            }
        }
    }

    pub fn change_fg_group_colour(self, group : &str, colour : Colour) -> () {
        for string in self.strings {
            if string.group == group{
                string.change_fg_colour(&colour)
            }
        }
    }

    //make sure you add newlines to the ColourStrings or else you will print one huge line
    pub fn string_as_true_colour(&self) -> String{
        self.strings.iter()
                    .map(|s| s.as_true_colour())
                    .collect()
    }







}






