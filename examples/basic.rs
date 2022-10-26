extern crate vm6502;
use std::time::Duration;

use vm6502::prelude::*;

fn main() {
    let mut vm = VirtualMachine::new();
    vm.set_program(0x0000, "6901690100");

    vm.run(Duration::from_millis(10));

    assert_eq!(vm.registers.ac, 0x02);
}
