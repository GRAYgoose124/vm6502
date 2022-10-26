use vm6502::prelude::*;

#[test]
fn test_insert_bytes() {
    let mut vm = VirtualMachine::new();
    vm.insert_bytes(0x0000, vec![0x69, 0x01]);
    assert_eq!(vm.flatmap[vm.heap_bounds.0 + 0x0000], 0x69);
    assert_eq!(vm.flatmap[vm.heap_bounds.0 + 0x0001], 0x01);

    let prog = vec![0x69, 0x01, 0x69, 0x02, 0x69, 0x03];
    vm.insert_bytes(0x0F00, prog);

    assert_eq!(vm.flatmap[vm.heap_bounds.0 + 0x0F00], 0x69);
    assert_eq!(vm.flatmap[vm.heap_bounds.0 + 0x0F01], 0x01);
    assert_eq!(vm.flatmap[vm.heap_bounds.0 + 0x0F02], 0x69);
    assert_eq!(vm.flatmap[vm.heap_bounds.0 + 0x0F03], 0x02);
    assert_eq!(vm.flatmap[vm.heap_bounds.0 + 0x0F04], 0x69);
    assert_eq!(vm.flatmap[vm.heap_bounds.0 + 0x0F05], 0x03);
}

#[test]
fn test_set_interrupt_vectors() {
    let mut vm = VirtualMachine::new();
    vm.set_interrupt_vectors(0xBADA, 0xBEEF, 0xCAFE);

    assert_eq!(vm.flatmap[0xFFFA], 0xDA);
    assert_eq!(vm.flatmap[0xFFFB], 0xBA);

    assert_eq!(vm.flatmap[0xFFFC], 0xEF);
    assert_eq!(vm.flatmap[0xFFFD], 0xBE);

    assert_eq!(vm.flatmap[0xFFFE], 0xFE);
    assert_eq!(vm.flatmap[0xFFFF], 0xCA);
}

#[test]
fn test_load_program() {
    let mut vm = VirtualMachine::new();
    // Get the program from the file in binaries/square_ints.a65
    vm.load_program(0x0000, "binaries/square_ints.a65");

    assert_eq!(vm.flatmap[vm.heap_bounds.0], 0xA0);
    assert_eq!(vm.flatmap[vm.heap_bounds.0 + 1], 0x00);
    assert_eq!(vm.flatmap[vm.heap_bounds.0 + 2], 0x84);
    assert_eq!(vm.flatmap[vm.heap_bounds.0 + 3], 0x32);
}
