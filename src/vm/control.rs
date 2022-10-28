use bitmatch::bitmatch;
use std::fmt::{Debug, Formatter, Result};

use crate::prelude::*;

pub mod prelude {
    pub use crate::vm::control::InstructionController;
    pub use crate::vm::control::Mode;
}

/// Virtual machine addressing mode enum.
///
#[derive(PartialEq, Copy, Clone)]
pub enum Mode {
    Accumulator,
    Implied,
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Relative,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndirectX,
    IndirectY,
}

impl Debug for Mode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Mode::Accumulator => write!(f, "Accumulator"),
            Mode::Implied => write!(f, "Implied"),
            Mode::Immediate => write!(f, "Immediate"),
            Mode::ZeroPage => write!(f, "ZeroPage"),
            Mode::ZeroPageX => write!(f, "ZeroPageX"),
            Mode::ZeroPageY => write!(f, "ZeroPageY"),
            Mode::Relative => write!(f, "Relative"),
            Mode::Absolute => write!(f, "Absolute"),
            Mode::AbsoluteX => write!(f, "AbsoluteX"),
            Mode::AbsoluteY => write!(f, "AbsoluteY"),
            Mode::Indirect => write!(f, "Indirect"),
            Mode::IndirectX => write!(f, "IndirectX"),
            Mode::IndirectY => write!(f, "IndirectY"),
        }
    }
}

pub trait InstructionController {
    fn step(&mut self) -> u64;
    // TODO: Abstract matches out of step so that you can get the ops then step with opcode.
    // fn opcode(&mut self, op: &str);
    // fn opcode(&mut self, op: u8);

    // Statefully sets the mode.
    fn set_mode(&mut self) -> Mode;
    /// Statefully fetches the address under the current mode.
    fn fetch_addr(&mut self) -> u16;
    /// Uses fetch_addr internally.
    fn fetch_byte(&mut self) -> u8;

    /// Nee
    fn apply(&mut self, source: u16, operation: fn(u8) -> u8) -> u8;
}

/**
Virtual machine core control functionality.

This provides three main internal functions, `step`, `mode`, and `fetch`.

# Examples
## `step`
```
use vm6502::prelude::*;
let mut vm = VirtualMachine::new();

vm.insert_program(0x00, "69FFFF");
vm.registers.pc = 0x00;

vm.step();

assert_eq!(vm.flatmap[vm.registers.pc as usize + vm.heap_bounds.0], 0xFF);
```
## `mode`
```
use vm6502::prelude::*;

let mut vm = VirtualMachine::new();
vm.active_byte = 0x69;
let mode = vm.set_mode();

assert_eq!(mode, Mode::Immediate);
```
## `fetch`
// TODO: Unignore this later.
```
use vm6502::prelude::*;

let mut vm = VirtualMachine::new();
let byte = 0x01;

// 0x200 is heap start. See `VirtualMachine::heap_bounds`.
vm.set_heap(0x0000, 0x69);
vm.set_heap(0x0001, byte);

assert_ne!(vm.flatmap[0x0001], byte, "Byte {} was not set to 0x0201", byte);
assert_eq!(byte, vm.flatmap[0x0201], "Byte {} was not set at 0x0201", byte);

// Should PC be 0x01 or two here?
vm.registers.pc = 0x00;
vm.addr_mode = Mode::Immediate;

let fetched = vm.fetch_byte();
assert_eq!(vm.registers.pc, 0x01, "PC should be incremented by 1 after fetch");

assert_eq!(fetched, byte, "Fetched byte {} does not match expected byte {}", fetched, byte);
```
*/
impl InstructionController for VirtualMachine {
    // Apply the operation to the address and return the result.
    // probably need a source/destination distinction here.
    fn apply(&mut self, address: u16, operation: fn(u8) -> u8) -> u8 {
        let doit = |d: &mut u8| -> u8 {
            let r = operation(*d);
            *d = r;
            r
        };

        match self.addr_mode {
            Mode::Accumulator => doit(&mut self.registers.ac),
            Mode::ZeroPage => doit(&mut self.flatmap[address as usize]),
            Mode::ZeroPageX => {
                doit(&mut self.flatmap[address as usize + self.registers.x as usize])
            }
            Mode::Absolute => doit(&mut self.flatmap[address as usize]),
            Mode::AbsoluteX => {
                doit(&mut self.flatmap[address as usize + self.registers.x as usize])
            }
            Mode::AbsoluteY => {
                doit(&mut self.flatmap[address as usize + self.registers.y as usize])
            }
            Mode::Indirect => doit(&mut self.flatmap[address as usize]),
            Mode::IndirectX => {
                doit(&mut self.flatmap[address as usize + self.registers.x as usize])
            }
            Mode::IndirectY => {
                doit(&mut self.flatmap[address as usize + self.registers.y as usize])
            }
            Mode::Immediate => doit(&mut self.flatmap[address as usize]),
            Mode::Relative => doit(&mut self.flatmap[address as usize]),
            Mode::Implied => doit(&mut self.flatmap[address as usize]),
            Mode::ZeroPageY => {
                doit(&mut self.flatmap[address as usize + self.registers.y as usize])
            }
        }
    }

    /// Check the opcode and set the addressing mode. This is a stateful operation.
    #[allow(clippy::bad_bit_mask)]
    #[bitmatch]
    fn set_mode(&mut self) -> Mode {
        self.addr_mode = #[bitmatch]
        match self.active_byte {
            "aaabbbcc" => match c {
                0x00 => match b {
                    0x00 => match a {
                        0x00 => Mode::Implied,
                        0x01 => Mode::Absolute,
                        0x02 | 0x03 => Mode::Implied,
                        0x05..=0x07 => Mode::Immediate,
                        _ => panic!("Illegal a value {} for cc0.(b=0x00)", a),
                    },
                    0x01 => match a {
                        0x01 => Mode::ZeroPage,
                        0x04..=0x07 => Mode::ZeroPage,
                        _ => panic!("Illegal a value {:02X} for cc0.(b=0x04..0x07)", a),
                    },
                    0x02 => Mode::Implied,
                    0x03 => match a {
                        0x00 => panic!("Illegal opcode 0x00 for cc0."),
                        0x03 => Mode::Indirect,
                        0x01 | 0x02 | 0x04..=0x07 => Mode::Absolute,
                        _ => panic!("Illegal a value {} for cc0.(b=0x01..0x07)", a),
                    },
                    0x04 => Mode::Relative,
                    0x05 => match a {
                        0x04 | 0x05 => Mode::ZeroPageX,
                        _ => panic!("Illegal a value {} for cc0.(b=0x04|0x05)", a),
                    },
                    0x06 => Mode::Implied,
                    0x07 => match a {
                        0x05 => Mode::AbsoluteX,
                        _ => panic!("Illegal a value {} for cc0.", a),
                    },
                    _ => panic!("Invalid cc0 mode: {}", b),
                },
                0x01 => match b {
                    0x00 => Mode::IndirectX,
                    0x01 => Mode::ZeroPage,
                    0x02 => match a {
                        0x04 => panic!("Illegal opcode 0x04 for cc1.(b=0x02)"),
                        _ => Mode::Immediate,
                    },
                    0x03 => Mode::Absolute,
                    0x04 => Mode::IndirectY,
                    0x05 => Mode::ZeroPageX,
                    0x06 => Mode::AbsoluteY,
                    0x07 => Mode::AbsoluteX,
                    _ => panic!("Invalid cc1 mode: {}", b),
                },
                0x02 => match b {
                    0x00 => match a {
                        0x00 => Mode::Implied,
                        0x05 => Mode::Immediate,
                        _ => panic!("Illegal a value {} for cc2(b=0x00)", a),
                    },
                    0x01 => Mode::ZeroPage,
                    0x02 => match a {
                        0x00..=0x03 => Mode::Accumulator,
                        0x04..=0x07 => Mode::Implied,
                        _ => panic!("Illegal a value {} for cc2(b=0x02)", a),
                    },
                    0x03 => Mode::Absolute,
                    0x04 => Mode::ZeroPageX,
                    0x05 => match a {
                        0x00..=0x03 | 0x06 | 0x07 => Mode::ZeroPageX,
                        0x04 | 0x05 => Mode::ZeroPageY,
                        _ => panic!("Illegal a value {} for cc2.(b=0x05)", a),
                    },
                    0x06 => match a {
                        0x04 | 0x05 => Mode::Implied,
                        _ => panic!("Illegal a value {} for cc2.(b=0x06)", a),
                    },
                    0x07 => match a {
                        0x00..=0x03 | 0x06 | 0x07 => Mode::AbsoluteX,
                        0x05 => Mode::AbsoluteY,
                        _ => panic!("Illegal a value {} for cc2.(b=0x07)", a),
                    },
                    _ => panic!("Invalid cc2 mode: {}", b),
                },
                _ => panic!("Invalid mode: {}", c),
            },
        };

        self.addr_mode
    }

    /// Fetch the address to be used for the current instruction.
    ///
    /// It sets self.mode_addr, but this may be deprecated.
    fn fetch_addr(&mut self) -> u16 {
        self.mode_addr = match self.addr_mode {
            Mode::Absolute => {
                // Todo can we move increments to get heap? We have to fix Relative to be
                // parallel. I don't think so, because indirect fetching.
                let ll = self.inc_pc_and_get_byte() as u16;
                let hh = self.inc_pc_and_get_byte() as u16;

                (hh << 2) | ll
            }
            // OPC $LLHH,X
            Mode::AbsoluteX => {
                let ll = self.inc_pc_and_get_byte() as u16;
                let hh = self.inc_pc_and_get_byte() as u16;

                (hh << 2) | (ll + self.registers.x as u16)
            }
            // OPC $LLHH,Y
            Mode::AbsoluteY => {
                let ll = self.inc_pc_and_get_byte() as u16;
                let hh = self.inc_pc_and_get_byte() as u16;

                (hh << 2) | (ll + self.registers.y as u16)
            }
            // Apply handles the accumulator.
            Mode::Accumulator => 0,
            // OPC $LLHH
            // operand is address $HHLL
            // OPC #$BB
            Mode::Immediate => {
                /*
                TODO: Can we factor pc addition into get_heap?
                For some reason this is off by one, control.rs:93
                seems to be showing that we're fetching the previous byte.
                */
                // I believe we can also increment the cycles:
                self.inc_pc_and_get_addr()
            }
            // OPC
            // Apply handles implied.
            Mode::Implied => 0,
            // OPC ($LLHH)
            Mode::Indirect => {
                let ll = self.inc_pc_and_get_byte() as u16;
                let hh = self.inc_pc_and_get_byte() as u16;

                (hh << 2) | ll
            }
            /*
            OPC ($LL, X)
            operand is zeropage address; effective address is word in (LL + X, LL + X + 1),
            inc. without carry: C.w(0LL + X)
            */
            Mode::IndirectX => {
                let ll = self.inc_pc_and_get_byte() as u16;
                let ell = self.get_heap(ll + self.registers.x as u16) as u16;
                let ehh = self.get_heap(ll + self.registers.x as u16 + 1) as u16;

                (ehh << 2) | ell
            }
            /*
            OPC ($LL), Y
            operand is zeropage address; effective address is word in (LL, LL + 1)
            incremented by Y with carry: C.w(0LL) + Y
            TODO: check if this is correct.
            */
            Mode::IndirectY => {
                let ll = self.inc_pc_and_get_byte() as u16;
                let ell = self.get_heap(ll) as u16;
                let ehh = self.get_heap(ll) as u16;

                (ehh << 2) | (ell + self.registers.y as u16)
            }
            // OPC $BB
            Mode::Relative => {
                // TODO: Check if i should be setting this
                self.inc_pc_and_get_byte() as u16
            }
            // OPC $LL
            Mode::ZeroPage => self.inc_pc_and_get_byte() as u16,
            // OPC $LL, X
            Mode::ZeroPageX => {
                let ll = self.inc_pc_and_get_byte() as u16;
                ll + self.registers.x as u16
            }
            // OPC $LL, Y
            Mode::ZeroPageY => {
                let ll = self.inc_pc_and_get_byte() as u16;
                ll + self.registers.y as u16
            }
        };

        self.mode_addr
    }

    fn fetch_byte(&mut self) -> u8 {
        let addr = self.fetch_addr();
        self.get_heap(addr)
    }
    /// Execute an arbitrary op. It returns the vm's current `cycle` count.
    #[bitmatch]
    fn step(&mut self) -> u64 {
        // Get current op TODO: Implement internal virtual bounds.
        self.active_byte = self.get_pc_byte();
        self.set_mode();

        #[cfg(feature = "show_vm_step")]
        println!(
            "step: 0x{:04X}: OP=0x{:02X}, {:?}",
            self.registers.pc, self.active_byte, self.addr_mode
        );

        // Increment PC for the OP fetched.
        // Logic says this should be done before, but maybe after?
        // self.registers.pc = self.registers.pc.wrapping_add(1);
        // if self.registers.pc == 0 {
        //     self.registers.pc = self.heap_bounds.0 as u16;
        // } // We don't need this, fetch i

        // Push the current program counter to the stack for a relative jump.
        // This is for procedures, move.
        /*if self.addr_mode == Mode::Relative {
            let old_ac = self.registers.ac;
            let bytes = self.registers.pc.to_be_bytes();

            self.registers.ac = bytes[1];
            self.push();
            self.registers.ac = bytes[0];gg
            self.push();

            self.registers.ac = old_ac;
        }*/

        #[allow(unused_variables)]
        #[bitmatch]
        match self.active_byte {
            "00000000" => {
                #[cfg(feature = "show_vm_instr_tick_match")]
                println!("BRK");
                self.brk()
            }
            "01000000" => {
                #[cfg(feature = "show_vm_instr_tick_match")]
                println!("RTI");
                self.rti()
            }
            "01100000" => {
                #[cfg(feature = "show_vm_instr_tick_match")]
                println!("RTS");
                self.rts()
            }
            "aaabbb01" => {
                #[cfg(feature = "show_vm_tick_arms")]
                println!("\taaabbb01 arm, a={:02X}, b={:02X}", a, b);

                match a {
                    0x00 => {
                        #[cfg(feature = "show_vm_instr_tick_match")]
                        println!("\t\tORA");
                        self.ora()
                    }
                    0x01 => {
                        #[cfg(feature = "show_vm_instr_tick_match")]
                        println!("\t\tAND");
                        self.and()
                    }
                    0x02 => {
                        #[cfg(feature = "show_vm_instr_tick_match")]
                        println!("\t\tEOR");
                        self.eor()
                    }
                    0x03 => {
                        //#[cfg(feature = "show_vm_instr_tick_match")]
                        //println!("\t\tADC");
                        self.adc()
                    }
                    0x04 => {
                        #[cfg(feature = "show_vm_instr_tick_match")]
                        println!("\t\tSTA");
                        self.sta()
                    }
                    0x05 => {
                        #[cfg(feature = "show_vm_instr_tick_match")]
                        println!("\t\tLDA");
                        self.lda()
                    }
                    0x06 => {
                        #[cfg(feature = "show_vm_instr_tick_match")]
                        println!("\t\tCMP");
                        self.cmp()
                    }
                    0x07 => {
                        #[cfg(feature = "show_vm_instr_tick_match")]
                        println!("\t\tSBC");
                        self.sbc();
                    }
                    _ => self.nop(),
                }
            }
            "aaabbb10" => {
                #[cfg(feature = "show_vm_tick_arms")]
                println!("\taaabbb10 arm, a={:02X}, b={:02X}", a, b);

                match a {
                    0x00 => {
                        #[cfg(feature = "show_vm_instr_tick_match")]
                        println!("\t\tASL");
                        self.asl()
                    }
                    0x01 => {
                        #[cfg(feature = "show_vm_instr_tick_match")]
                        println!("\t\tROL");
                        self.rol()
                    }
                    0x02 => {
                        #[cfg(feature = "show_vm_instr_tick_match")]
                        println!("\t\tLSR");
                        self.lsr()
                    }
                    0x03 => {
                        #[cfg(feature = "show_vm_instr_tick_match")]
                        println!("\t\tROR");
                        self.ror()
                    }
                    0x04 => {
                        #[cfg(feature = "show_vm_instr_tick_match")]
                        println!("\t\tSTX");
                        self.stx()
                    }
                    0x05 => {
                        #[cfg(feature = "show_vm_instr_tick_match")]
                        println!("\t\tLDX");
                        self.ldx()
                    }
                    0x06 => {
                        #[cfg(feature = "show_vm_instr_tick_match")]
                        println!("\t\tDEC");
                        self.dec()
                    }
                    0x07 => {
                        #[cfg(feature = "show_vm_instr_tick_match")]
                        println!("\t\tINC");
                        self.inc()
                    }
                    _ => self.nop(),
                }
            }
            "aaabbb00" => {
                #[cfg(feature = "show_vm_tick_arms")]
                println!("\taaabbb00 arm, a={:02X}, b={:02X}", a, b);

                // This is the only arm that triggers when op maps to Mode::Relative.
                // Therefore, lets make the assumption that we can do the relative offset calculation here instead of in the fetch function.
                // We are setting the PC
                let offset = self.fetch_addr() as i8;
                println!("\t    Relative offset: 0x{:02X}", offset);

                if b == 0b100 {
                    match a {
                        0x00 => {
                            self.bpl(offset);
                            #[cfg(feature = "show_vm_instr_tick_match")]
                            println!("\t\tBPL");
                        }
                        0x01 => {
                            self.bmi(offset);
                            #[cfg(feature = "show_vm_instr_tick_match")]
                            println!("\t\tBMI");
                        }
                        0x02 => {
                            self.bvc(offset);
                            #[cfg(feature = "show_vm_instr_tick_match")]
                            println!("\t\tBVC");
                        }
                        0x03 => {
                            self.bvs(offset);
                            #[cfg(feature = "show_vm_instr_tick_match")]
                            println!("\t\tBVS");
                        }
                        0x04 => {
                            self.bcc(offset);
                            #[cfg(feature = "show_vm_instr_tick_match")]
                            println!("\t\tBCC");
                        }
                        0x05 => {
                            self.bcs(offset);
                            #[cfg(feature = "show_vm_instr_tick_match")]
                            println!("\t\tBCS");
                        }
                        0x06 => {
                            self.bne(offset);
                            #[cfg(feature = "show_vm_instr_tick_match")]
                            println!("\t\tBNE");
                        }
                        0x07 => {
                            self.beq(offset);
                            #[cfg(feature = "show_vm_instr_tick_match")]
                            println!("\t\tBEQ");
                        }
                        _ => self.nop(),
                    }
                } else {
                    match a {
                        0x00 => {
                            #[cfg(feature = "show_vm_instr_tick_match")]
                            println!("\t\tBIT");
                            self.bit()
                        }
                        0x01 => {
                            #[cfg(feature = "show_vm_instr_tick_match")]
                            println!("\t\tJSR");
                            self.jsr()
                        }
                        0x02 => {
                            #[cfg(feature = "show_vm_instr_tick_match")]
                            println!("\t\tJMP");
                            self.jmp()
                        }
                        0x03 => {
                            #[cfg(feature = "show_vm_instr_tick_match")]
                            println!("\t\tSTY");
                            self.sty()
                        }
                        0x04 => {
                            #[cfg(feature = "show_vm_instr_tick_match")]
                            println!("\t\tLDY");
                            self.ldy()
                        }
                        0x05 => {
                            #[cfg(feature = "show_vm_instr_tick_match")]
                            println!("\t\tCPY");
                            self.cpy()
                        }
                        0x06 => {
                            #[cfg(feature = "show_vm_instr_tick_match")]
                            println!("\t\tCPX");
                            self.cpx()
                        }
                        _ => self.nop(),
                    }
                }
            }
            // conditional jumps = aab10000
            "xxx10000" => {
                #[cfg(feature = "show_vm_tick_arms")]
                println!("\txxx10000 arm, x={:02X}", x);
            }
            "00001000" => {
                self.php();
                #[cfg(feature = "show_vm_instr_tick_match")]
                println!("\t\tPHP");
            }
            "00101000" => {
                self.plp();
                #[cfg(feature = "show_vm_instr_tick_match")]
                println!("\t\tPLP");
            }
            "01001000" => {
                self.pha();
                #[cfg(feature = "show_vm_instr_tick_match")]
                println!("\t\tPHA");
            }
            "01101000" => {
                self.pla();
                #[cfg(feature = "show_vm_instr_tick_match")]
                println!("\t\tPLA");
            }
            "10001000" => {
                self.dey();
                #[cfg(feature = "show_vm_instr_tick_match")]
                println!("\t\tDEY");
            }
            "10101000" => {
                self.tay();
                #[cfg(feature = "show_vm_instr_tick_match")]
                println!("\t\tTAY");
            }
            "01001100" => {
                self.iny();
                #[cfg(feature = "show_vm_instr_tick_match")]
                println!("\t\tINY");
            }
            "11101000" => {
                self.inx();
                #[cfg(feature = "show_vm_instr_tick_match")]
                println!("\t\tINX");
            }
            "00011000" => {
                self.clc();
                #[cfg(feature = "show_vm_instr_tick_match")]
                println!("\t\tCLC");
            }
            "00111000" => {
                self.sec();
                #[cfg(feature = "show_vm_instr_tick_match")]
                println!("\t\tSEC");
            }
            "01011000" => {
                self.cli();
                #[cfg(feature = "show_vm_instr_tick_match")]
                println!("\t\tCLI");
            }
            "01111000" => {
                self.sei();
                #[cfg(feature = "show_vm_instr_tick_match")]
                println!("\t\tSEI");
            }
            "10011000" => {
                self.tya();
                #[cfg(feature = "show_vm_instr_tick_match")]
                println!("\t\tTYA");
            }
            "10111000" => {
                self.clv();
                #[cfg(feature = "show_vm_instr_tick_match")]
                println!("\t\tCLV");
            }
            "11011000" => {
                self.cld();
                #[cfg(feature = "show_vm_instr_tick_match")]
                println!("\t\tCLD");
            }
            "11111000" => {
                self.sed();
                #[cfg(feature = "show_vm_instr_tick_match")]
                println!("\t\tSED");
            }
            "10001010" => {
                self.txa();
                #[cfg(feature = "show_vm_instr_tick_match")]
                println!("\t\tTXA");
            }
            "10011010" => {
                self.txs();
                #[cfg(feature = "show_vm_instr_tick_match")]
                println!("\t\tTXS");
            }
            "10101010" => {
                self.tax();
                #[cfg(feature = "show_vm_instr_tick_match")]
                println!("\t\tTAX");
            }
            "10111010" => {
                self.tsx();
                #[cfg(feature = "show_vm_instr_tick_match")]
                println!("\t\tTSX");
            }
            "11001010" => {
                self.dex();
                #[cfg(feature = "show_vm_instr_tick_match")]
                println!("\t\tDEX");
            }
            _ => {
                self.nop();
                #[cfg(feature = "show_vm_instr_tick_match")]
                println!("\t\tNOP");
            }
        };

        #[cfg(feature = "show_vm_post_op")]
        println!("{:?}", self);

        // This should be counting the consumed ops.
        // self.cycles += 1;
        self.registers.pc = self.registers.pc.wrapping_add(1);

        // TODO: This should be updated (along with the PC) by the above commands.
        self.cycles
    }
}
