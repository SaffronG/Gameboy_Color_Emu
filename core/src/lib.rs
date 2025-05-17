pub const SCREEN_WIDTH: usize = 160;
pub const SCREEN_HEIGHT: usize = 144;
const WINDOW_SIZE: usize = 256;
const RAM_SIZE: usize = 32768;
const HRAM_SIZE: usize = 127;
const VRAM_SIZE: usize = 16384;
const OAM_SIZE: usize = 160;
const C_REG_NUM: usize = 8;

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
    H_Blank = 0, // "Pads" the remainder of the scanline
    V_Blank = 1, // V-blank but happens at the end of each scanline
    Scan    = 2, // Searches the OAM memory for sprites to render before they are drawn
    Drawing = 3, // Transfers pixels to the LCD
}

pub struct Cpu {
    // 8Bit: General Purpose Registers
    reg_8b: [u8; 8], // a, b, c, d, e, f, h, l
                     
    // 16Bit: General Purpose Registers
    reg_16b: [(u8,u8); 8],
        //af: u16, // Hi - a
        //bc: u16, // Hi - B, Lo - C
        //de: u16, // Hi - D, Lo - E
        //hl: u16, // Hi - H, Lo - L
    sp: u16,
    pc: u16,
    // Flag Registers (lower 8 bits of AF register)
    zero: u8,
    sub: u8,
    h_carry: u8,
    carry: u8,
    // Storage
    ram: [u8; RAM_SIZE],
    hram: [u8; HRAM_SIZE],
    vram: [u8; VRAM_SIZE],
    viewport: [u8; WINDOW_SIZE * WINDOW_SIZE],
    window: [u8; WINDOW_SIZE * WINDOW_SIZE],
    screen: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
    ppu: Ppu,
    cartridge: Option<Cartridge>,
}

impl Cpu {
    
    fn execute(&mut self, op: u8) {
       let opcode: u8 = self.ram[self.pc as usize] & 0xFF;
       match opcode {
            0x00 => {
                // NOP
                let _ = self.pc.wrapping_add(1);
            },
            0x01 => {
                // LD r16, n16 -> load n16 into r16
                // cycles: 3
                // bytes: 3
                let mut dest = self.reg_16b[(op & 0xCF >> 4) as usize];
                dest.0 = self.ram[self.pc.wrapping_add(1) as usize];
                dest.1 = self.ram[self.pc.wrapping_add(1) as usize];
            }
            _ => todo!(),
       }
    }
}

// the PPU operates on a pixel-basis, not on a tile-basis
struct Ppu {
    oam: [u8; OAM_SIZE], // Object Attribute Memory $FE00-$FE9F
    // Pixel properties:
        // Color -> The color number (IGNORING the palette, from tile color)
        // Palette -> Value between 0-7 (0BP0 / 0BP1)
        // Sprite Priority -> Only relevant for sprits on the CGB
        // Background Priority -> Only relevant for sprites, keeps the value of bit 7
}

struct Cartridge {

}
