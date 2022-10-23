use crate::prelude::*;

pub mod prelude {
    pub use crate::vm::stack::StackInterface;
}

pub trait StackInterface {
    fn pop(&mut self) -> u8;
    fn peek(&mut self) -> u8; // Not congruent with spec.

    fn push(&mut self, value: u8);
}

impl StackInterface for VirtualMachine {
    fn pop(&mut self) -> u8 {
        let value = self.flatmap[self.stack_bounds.1 - self.registers.sp as usize];

        if cfg!(debug_assertions) {
            println!("Popped value: {}. SP: {}", value, self.registers.sp);
        }

        // Move to next stack entry, TODO: Check the wraps.
        self.registers.sp = self.registers.sp.saturating_sub(1);

        value
    }

    // Debug / Not Spec
    fn peek(&mut self) -> u8 {
        self.flatmap[self.stack_bounds.1 - self.registers.sp as usize]
    }

    fn push(&mut self, value: u8) {
        self.flatmap[self.stack_bounds.1 - (self.registers.sp as usize)] = value;

        self.registers.sp = self.registers.sp.saturating_add(1);
    }
}
