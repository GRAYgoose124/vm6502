# vm6502 ![](https://github.com/GRAYgoose124/vm6502/actions/workflows/tests.yml/badge.svg) ![](https://img.shields.io/crates/v/vm6502.svg)
As the name suggests, this crate is a 6502 virtual machine.

It's primary application case is found in [emu6502](https://crates.io/crates/emu6502).

Currently, if you'd like more information, please check [docs.rs](https://docs.rs/vm6502/0.1.0/vm6502/)
to the vm6502 crate.
```bash
    # To run the virtual cpu tests, first cd to the `vm6502` directory then run:
    cargo test
```
## Features
`pretty', 'debug', and 'ugly' debugging modes available to emu6502.

When debugging vm6502, there are many features you can enable and disable, see the Cargo.toml for information.

## References
- [6502 Instruction Set](https://www.masswerk.at/6502/6502_instruction_set.html)
- [6502 Addressing Modes](http://www.emulator101.com/6502-addressing-modes.html)