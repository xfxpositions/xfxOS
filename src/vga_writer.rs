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
const  VGA_HEIGHT:usize = 80;
const VGA_WIDTH:usize = 25; 

struct Char{
    ascii:u8,
    color:u8
}
struct Buffer{
    charslist:&'static mut [[Char;VGA_WIDTH];VGA_HEIGHT],
}
pub struct Printer{
    DEFAULT_BG:u8,
    DEFAULT_FG:u8,
    row:u16,
    column:u16,
    vga_buffer:&'static mut Buffer,
}

impl Printer{
    pub fn init(self:&mut Self,DEFAULT_BG:u8,DEFAULT_FG:u8){
     
        self.row = 0;
        self.column = 0;
        self.DEFAULT_BG = DEFAULT_BG;
        self.DEFAULT_FG = DEFAULT_FG;      
    }
    
    pub fn print(self:&mut Self,c:u8,color:u8){
       let row = VGA_HEIGHT-1;
       let col = 0;
       self.vga_buffer.charslist[row][col] = Char{
        ascii:c,color:color
       }
    }
}
pub fn print_test(){
    let mut printer:Printer = Printer { DEFAULT_BG:0,
         DEFAULT_FG:1,
         vga_buffer: unsafe { &mut *(0xb8000 as *mut Buffer) }, 
         row: 0, column: 0,
    };
    let text :&[u8;6] = b"Deneme";
    printer.print(21, 1);
}