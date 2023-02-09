#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum VGACOLORS{
    BLACK = 0,
    BLUE = 1,
    GREEN = 2,
    CYAN = 3,
    RED = 4,
    MAGENTA = 5,
    BROWN = 6,
    LIGHTGRAY = 7,
    DARKGRAY = 8,
    LIGHTBLUE = 9,
    LIGHTGREEN = 10,
    LIGHTCYAN = 11,
    LIGHTRED = 12,
    PINK = 13,
    YELLOW = 14,
    WHITE = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct Char{
    ascii:u8,
    color:u8
}
impl Char {
    pub fn create_color_code(fg:VGACOLORS,bg:VGACOLORS)-> u8{
        return (bg as u8) << 4 | (fg as u8);
    }
}
#[repr(transparent)]
pub struct Buffer{
    charlist:[[Char;80];25]
}
pub struct Printer{
    pub vga_buffer:&'static mut Buffer,
    pub row_position:usize,
    pub col_position:usize,
    pub default_fg:VGACOLORS,
    pub default_bg:VGACOLORS
}

impl Printer{
    pub fn print_chr(self:&mut Self,chr:u8,fg:VGACOLORS,bg:VGACOLORS){
        if (chr == '\n' as u8){
            self.row_position += 1;
            return
        }else{
            let color = Char::create_color_code(fg, bg);
            let col = self.col_position;
            let row = self.row_position;
            self.vga_buffer.charlist[row][col] = Char{ascii:chr,color:color};
            self.col_position = self.col_position+1;
        }
    }
    pub fn print(self:&mut Self, str:&str){
        for chr in str.bytes(){
            self.print_chr(chr, self.default_fg, self.default_bg)
        }
    }
    pub fn println(self:&mut Self, str:&str){
        for chr in str.bytes(){
            self.print_chr(chr, self.default_fg, self.default_bg)
        }
        self.col_position = 0;
        self.row_position+=1;
    }
    pub fn println_colored(self:&mut Self, str:&str,fg:VGACOLORS,bg:VGACOLORS){
        for chr in str.bytes(){
            self.print_chr(chr, fg, bg)
        }
        self.col_position = 0;
        self.row_position+=1;
    }
}