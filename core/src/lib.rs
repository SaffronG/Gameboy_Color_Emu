use random::random;

pub const SCREEN_WIDTH: usize = 160;
pub const SCREEN_HEIGHT: usize = 144;
const WINDOW_SIZE: usize = 256
const RAM_SIZE: usize = 32768;
const HRAM_SIZE: usize = 127;
const VRAM_SIZE: usize = 16384;
const OAM_SIZE: usize = 160;
const C_REG_NUM: uszize = 8;

// Tile Data
// Graphics Data is stored at $8000-$97FF is often called "Tile Number"

// REGISTER BREAKDOWN
// $: LCDC ->
// $: SCX -> Sets the X position of the viewport
// $: SCY -> Sets the Y position of the viewport
// $FF4A: WY -> Sets the Y position of the window's top border (0 == top)
// $FF4B: WX -> Sets the X position of the window (7 == left-edge)
//    NOTE: WX - 7 yields the X position, Edge cases occur at: n < 7
#[repr(u8)]
enum PpuMode {
    H-Blank = 0,
    V-Blank = 1,
    Scan = 2,
    Drawing = 3,
}

pub struct Emu {
    pc: u16,
    ram: [u8; RAM_SIZE],
    hram: [u8; HRAM_SIZE],
    vram: [u8; VRAM_SIZE],
    viewport: [u8; WINDOW_SIZE * WINDOW_SIZE],
    window: [u8; WINDOW_SIZE * WINDOW_SIZE],

    screen: [bool; SCREEN_WIDTH * SCREEN_HEIGHT]
    svbk: u32;
    
}

// the PPU operates on a pixel-basis, not on a tile-basis
struct Ppu {
    oam: [u8; OAM_SIZE], // Object Attribute Memory $FE00-$FE9F
    dma: 
    palette:
    c_reg: [u8; C_REG_NUM],

}

struct Cartridge {

}
