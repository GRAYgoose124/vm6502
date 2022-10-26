use vm6502::opcode_name;
use vm6502::prelude::*;
use vm6502::status;
use vm6502::utils::machine_arrays::VALID_CYCLE_COUNTS;
// TODO: The problem is likely in the core and functionality.
#[test]
fn adc_imd() {
    let mut vm = VirtualMachine::new();
    vm.set_program(0x0000, "69F06901");

    vm.registers.ac = 0x0F;
    vm.step();
    assert_eq!(vm.registers.ac, 0xFF);

    vm.step();
    assert_eq!(vm.registers.sr & status!(Status::Carry), 1);
    assert_eq!(vm.registers.ac, 0x00);
}

// TODO: The problem is likely in the core and functionality.
#[test]
fn and_imd() {
    let mut vm = VirtualMachine::new();
    vm.set_program(0x0000, "29FF29FF2900");
    eprintln!("Program: {:?}...", &vm.flatmap[0x0200..0x0203]);
    vm.registers.ac = 0x00;

    vm.step();
    assert_eq!(vm.registers.ac, 0x00);
    assert_eq!(vm.get_status(Status::Zero), true);

    vm.registers.ac = 0xFF;
    eprintln!("PC byte 0: {}", vm.get_heap(vm.registers.pc));
    eprintln!("PC byte 1: {}", vm.get_heap(vm.registers.pc + 1));
    vm.step();
    assert_eq!(vm.registers.ac, 0xFF & 0xFF);
    assert_eq!(vm.get_status(Status::Zero), false);

    vm.step();
    assert_eq!(vm.registers.ac, 0x00);
    assert_eq!(vm.get_status(Status::Zero), true);
}

#[test]
fn asl_cover() {
    let mut vm = VirtualMachine::new();
    let prog = "0A0A0A0A0A0A0A0A0A";
    vm.set_program(0x0000, prog);
    vm.registers.ac = 0x01;

    for i in 1..8 {
        vm.step();
        eprintln!("i: {}, ac: {}", 1 << i, vm.registers.ac);

        assert_eq!(vm.registers.ac, 1 << i);
    }
}

#[ignore]
#[test]
fn check_cycles_used() {
    let mut vm = VirtualMachine::new();

    let last_cycles: u64 = 0;
    for (i, valid_op) in VALID_OPCODES.iter().enumerate() {
        eprintln!(
            "i: {}, op: 0x{:02X}, {:?}",
            i,
            valid_op,
            opcode_name!(*valid_op)
        );
        vm.registers.pc = 0x0000;
        vm.flatmap[vm.heap_bounds.0] = *valid_op;

        vm.step();

        assert_eq!((vm.cycles - last_cycles) as u8, VALID_CYCLE_COUNTS[i]);
    }
}
