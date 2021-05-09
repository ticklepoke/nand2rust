# Assembler

This module contains the assembler for translating `.asm` hack assembly code to `.hack` hack binary.

## Usage

To run the assembler

```
cargo run <filename.asm>
```

A portable assembler can also be built at `./target/release/assembler`
```
cargo build --release
```

Unit tests are included, which uses the examples provided by the textbook.
```
cargo test
```

## Instruction Format

Hack instructions fall under 2 categories: address (a) instructions and compute (c) instructions.

### Address Instruction

Address instructions are used to load literal values or values from memory address. 

Hack assembly instructions are prefixed with `@` followed by an integer or variable name.
```
@100    // literal value
@i      // variable name
```

Hack binary instructions are 15-bit values prefixed with a `0`.
```
0vvv vvvv vvvv vvvv // where v = 0 or 1
```

### Compute Instruction

Compute instructions are used for control flow and arithmetic operations on registers.

Hack assembly instructions take the following format:
```
dest=comp;jump
```

If `dest` is not present, `=` is ommitted.
If `jump` is no present, `;` is ommitted.
`Comp` is always required.

The following instructions are valid:
```
D=M
D=D-A
D;JGT
```

Hack binary instructions are 13-bit values prefixed with `111`.
```
111a c1c2c3c4 c5c6d1d2 d3j1j2j3
```

## Symbols

Jump labels and variable are supported using a hashmap. 
In the symbol-supported version of the assembler, `.asm` code goes through a 2-pass process. The first pass involves scanning out jump labels. The second pass involves translation to binary, scanning out variables and replacing all symbols with their appropriate memory addresses.
