# BOOP - Barely Operational Optimized Processor

Esoteric CPU architecture I designed to be as simple as possible to build.

WIP (as always)

## Architecture:

The CPU contains:
- 16 bit RAM
- Fetch unit
- 3 registers:s
    - Accumulator (a)
    - Instruction Pointer (ip)
        - Starts at 0x0000
    - Stack Pointer (sp)
        - Starts at 0xFFFF
- ALU with two operations:
    - Bitwise NAND
    - Add
- Serial In and Out


## Instructions
There is exactly one instruction type in BOOP. All instructions are ALU Operations. The instructions are laid out as follows:

`AAABBCDEEFGHHIJ0`
 
 `AAA` - ALU Left Input

 `BB` - ALU Right Input

 `C` - ALU Carry In

 `D` - ALU Operation

 `EE` - ALU Output Reg

 `F` - Decrement sp (occurs before ALU write)

 `G` - Increment sp (occurs)

 `HH` - Conditional increment ip
 
 `I` - Increment ip

 `J` - Convert memory operations to serial operations. 

 TODO, complete this.