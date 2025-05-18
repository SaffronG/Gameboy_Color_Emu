use std::io::Read;

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

#[repr(usize)]
enum Reg {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
    H = 6,
    L = 7,
}

#[repr(usize)]
enum Regu16 {
    AF = 0,
    BC = 1,
    DE = 2,
    HL = 3,
}

fn as_u16(d: (u8, u8)) -> u16 {
    (((0 as u16) & (d.0 as u16)) << 4) & d.1 as u16
}

pub struct Cpu {
    // 8Bit: General Purpose Registers
    reg_8b: [u8; 8], // a, b, c, d, e, f, h, l
                     
    // 16Bit: General Purpose Registers
    reg_16b: [(u8,u8); 4],
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
    fn new() -> Cpu {
        Cpu {
            reg_8b: [0; 8],
            reg_16b: Self::init_regs(),
            sp: 0, // SET TO CORRECT INITIAL VALUE LATER
            pc: 0,
            zero: 0,
            sub: 0,
            h_carry: 0,
            carry: 0,
            ram: [0; RAM_SIZE],
            hram: [0; HRAM_SIZE],
            vram: [0; VRAM_SIZE],
            viewport: [0; WINDOW_SIZE * WINDOW_SIZE],
            window: [0; WINDOW_SIZE * WINDOW_SIZE],
            screen: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
            ppu: Ppu::new(),
            cartridge: None,
        }
    }
    fn init_regs() -> [(u8,u8); 4] {
        [(0,0); 4]
    }
    fn execute(&mut self, op: u8) {
       let opcode: u8 = self.ram[self.pc as usize] & 0xFF;
       let r16_mask: u8 = 0xCF;
       let r8_mask: u8 = 0b00111000;
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
            0x02 => {
                // Ld [BC], A
                self.ram[as_u16(self.reg_16b[(op & 0xCF >> 4) as usize]) as usize] = self.ram[self.pc.wrapping_add(1) as usize];
            },
            0x03 => {
                // INC BC
                let reg = self.reg_16b[(op & r16_mask >> 4) as usize];
                match reg.0.checked_add(1) {
                    Some(_) => (),
                    None => { let _ = reg.0.checked_add(1); },
                }
            }
            0x04 => {
                // INC B
                let reg = self.reg_8b[(op & r8_mask >> 3) as usize];
                match reg.checked_add(1) {
                    Some(_) => (),
                    None => { let _ = reg.checked_add(1); },
                }
            },
            0x05 => {
                // DEC B
                let reg = self.reg_8b[(op & r8_mask >> 3) as usize];
                match reg.checked_sub(1) {
                    Some(_) => (),
                    None => { let _ = reg.checked_sub(1); },
                }
            },
            0x06 => {
                // LD B n8
                let mut reg = self.reg_8b[(op & r8_mask >> 3) as usize];
                reg = self.ram[self.pc.wrapping_add(1) as usize];
            },
            0x07 => {
                // RLCA

            },
            0x08 => {
                // LD [a16], SP
            },
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

impl Ppu {
    fn new() -> Ppu {
        Ppu {
            oam: [0; OAM_SIZE],
        }
    }
}

struct Cartridge {
   memory: Vec<u8>, 
}

impl Cartridge {
    fn read(&mut self, fname: String) -> std::io::Result<()> {
        let mut std_fd = std::fs::File::open(fname)?;
        match std_fd.read_to_end(&mut self.memory) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}
