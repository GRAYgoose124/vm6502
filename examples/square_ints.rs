extern crate vm6502;
use std::time::Duration;

use vm6502::prelude::*;

fn main() {
    let mut vm = VirtualMachine::new();
    vm.load_program(0x0000, "asm65/square_ints.a65");

    // It shouldn't take more than 10ms on any modern machine to run this program.
    // If it does, something is wrong. This is a sane assumption... Right? Right.
    vm.run(Duration::from_millis(1000));

    println!("Final: {:?}", vm);
}
