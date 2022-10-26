use vm6502::prelude::*;

#[ignore]
#[test]
fn test_square_ints_program(){
    let mut vm = VirtualMachine::new();
    vm.load_program(0x0000, "binaries/square_ints.a65");

    let cycles = vm.execute();
    eprintln!("Cycles: {}", cycles);
}