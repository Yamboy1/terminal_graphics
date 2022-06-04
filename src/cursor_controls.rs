
const escape_code : &str = "\x1B";

pub enum cursor_type {
    DEC,
    SCO
}

pub fn home() -> (){
    print!("{}[H", escape_code);
}

pub fn to_x_y(x: i16, y: i16) -> (){
    print!("{}[{};{}H", escape_code, y, x);
}

pub fn save_pos(c:cursor_type) -> (){
    match c{
        DEC => print!("{} 7", escape_code),
        SCO => print!("{}[s", escape_code),
    }    
}

pub fn goto_saved_pos(c:cursor_type) -> (){
    match c{
        DEC => print!("{} 8", escape_code),
        SCO => print!("{}[u", escape_code),
    }
}

pub fn hide() -> (){
    print!("{}[?25l", escape_code);
}

pub fn show() -> (){
    print!("{}[?25h", escape_code);
}