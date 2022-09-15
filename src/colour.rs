
pub struct Colour{
    pub r : u8,
    pub b : u8,
    pub g : u8,
}

impl Colour{

    pub fn new(r: u8, g: u8, b: u8) -> Colour{
        Colour{
            r,
            g,
            b
        }
    }

    pub fn black() -> Colour{
        Colour{
            r : 0,
            b : 0,
            g : 0
        }
    }

    pub fn white() -> Colour{
        Colour{
            r : 255,
            b : 255,
            g : 255
        }
    }

    pub fn pink() -> Colour{
        Colour{
            r : 255,
            b : 255, 
            g : 0
        }
    }


}