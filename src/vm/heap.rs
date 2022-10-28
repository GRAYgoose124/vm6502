// TODO: Format heap here instead of VM mod?
// use std::fmt::{Debug, Formatter, Result};

use crate::prelude::*;

pub mod prelude {
    pub use crate::vm::heap::HeapInterface;
}

/// Provides a low level interface for accessing the heap.
///
/// It's simply a wrapper around the flatmap, using the internal [heap_bounds.0](VirtualMachine::heap_bounds) to index the heap.
pub trait HeapInterface {
    // Low level iinterface
    /// Returns the value at the heap address given.
    fn get_heap(&mut self, virt_addr: u16) -> u8;
    /// Sets the value at the heap address given.
    fn set_heap(&mut self, virt_addr: u16, byte: u8);
    /// Get the byte on the heap at PC.
    fn get_pc_byte(&mut self) -> u8;
    /// Increment the PC and return the byte at the new PC.
    fn inc_pc_and_get_byte(&mut self) -> u8;
    /// Increment the PC and return the byte at the new PC.
    fn inc_pc_and_get_addr(&mut self) -> u16;

    // Mid level interface
    /// return the bytes, 0xHH__ from the PC. More of a convenience/debug function.
    fn get_page_offset(&self) -> u8;
    /// Add an the virt_addr to the high byte of the PC - it's a "magic" jump, bypassing modes.
    fn set_page_offset(&mut self, virt_addr: u8);

    fn bounds_check(&self, virt_addr: usize) -> bool;
}

impl HeapInterface for VirtualMachine {
    // TODO: Reimplement proper bounds checking.
    fn get_heap(&mut self, virt_addr: u16) -> u8 {
        let addr = virt_addr as usize + self.heap_bounds.0;

        #[cfg(feature = "check_heap_bounds")]
        self.bounds_check(addr);

        self.flatmap[addr]
    }

    fn get_pc_byte(&mut self) -> u8 {
        self.get_heap(self.registers.pc)
    }

    fn inc_pc_and_get_byte(&mut self) -> u8 {
        self.registers.pc = self.registers.pc.wrapping_add(1);
        self.get_heap(self.registers.pc)
    }

    fn inc_pc_and_get_addr(&mut self) -> u16 {
        self.registers.pc = self.registers.pc.wrapping_add(1);
        self.registers.pc
    }

    fn set_heap(&mut self, virt_addr: u16, byte: u8) {
        let addr = virt_addr as usize + self.heap_bounds.0;

        #[cfg(feature = "check_heap_bounds")]
        self.bounds_check(addr);

        self.flatmap[addr] = byte;
    }

    /// Returns the page offset for the current PC.
    fn get_page_offset(&self) -> u8 {
        let virt_page = (self.registers.pc & 0xFF00) >> 8;

        virt_page as u8
    }

    /// Sets the PC to the given page offset.
    fn set_page_offset(&mut self, virt_addr: u8) {
        let new_pc = (self.registers.pc & 0x00FF) | (virt_addr as u16) << 8;
        #[cfg(feature = "check_heap_bounds")]
        self.bounds_check(new_pc as usize);

        self.registers.pc = new_pc;
    }

    /// Checks if the given address is within the heap bounds. TODO: Reimplement.
    fn bounds_check(&self, virt_addr: usize) -> bool {
        if virt_addr < self.heap_bounds.0 {
            #[cfg(feature = "passthrough_failure")]
            panic!("Attempted to access heap before heap bounds!");
            #[cfg(not(feature = "passthrough_failure"))]
            {
                println!("Attempted access before virtual heap bounds!");
                false
            }
        } else if virt_addr > self.heap_bounds.1 {
            #[cfg(feature = "passthrough_failure")]
            panic!("Attempted to access heap after heap bounds!");

            #[cfg(not(feature = "passthrough_failure"))]
            {
                println!("Attempted access after virtual heap bounds!");
                false
            }
        } else {
            true
        }
    }
}

pub trait HeapController {
    // High level interface
    fn alloc(&mut self);
    fn dealloc(&mut self);
}
