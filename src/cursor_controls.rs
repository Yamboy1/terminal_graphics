
const ESCAPE_CODE : &str = "\x1B";

pub enum cursor_type {
    DEC,
    SCO
}

pub fn home() -> (){
    print!("{}[H", ESCAPE_CODE);
}

pub fn to_x_y(x: i16, y: i16) -> (){
    print!("{}[{};{}H", ESCAPE_CODE, y, x);
}

pub fn save_pos(c:cursor_type) -> (){
    match c{
        DEC => print!("{} 7", ESCAPE_CODE),
        SCO => print!("{}[s", ESCAPE_CODE),
    }    
}

pub fn goto_saved_pos(c:cursor_type) -> (){
    match c{
        DEC => print!("{} 8", ESCAPE_CODE),
        SCO => print!("{}[u", ESCAPE_CODE),
    }
}

pub fn hide() -> (){
    print!("{}[?25l", ESCAPE_CODE);
}

pub fn show() -> (){
    print!("{}[?25h", ESCAPE_CODE);
}