# bcpu
An emulator for a CPU with any instruction size. Made for experimentation

## Why?
This is mostly just an experiment with writing bytecode. I can't think of a practical use for this. It's probably going to end up being terribly slow, but in my defense, Java seems to work just fine, although it doesn't artificially slow itself down with aribtrary instruction sizes. Alas, I long for the day when CPUs will have this functionality. I'm not an engineer so there's probably a good reason why this will never happen.

## How to Use

I plan to eventually create an assembler for this, but here's how the bitcode works.

### An Instruction

Instruction Code - 5 bits

Instruction Size - 6 bits

Location (Memory | Register) - 1 bit

Memory Address (Up to 2.3e18 bytes supported) - 64 bits

Register - 4 bits

Register Starting Location - 6 bits

### Instructions

```
MOV (destination source)
ADD (location op1 op2)
SUB (location op1 op2)
MUL (location op1 op2)
DIV (location op1 op2)
MOD (location op1 op2)
ADDI (location op1 op2)
SUBI (location op1 op2)
MULI (location op1 op2)
DIVI (location op1 op2)
MODI (location op1 op2)
ADDF (location op1 op2)
SUBF (location op1 op2)
MULF (location op1 op2)
DIVF (location op1 op2)
MODF (location op1 op2)
AND (location op1 op2)
OR (location op1 op2)
XOR (location op1 op2)
NOT (location op1 op2)
CMP (op1 op2)
CMPI (op1 op2)
CMPF (op1 op2)
JEQ (instruction)
JNE (instruction)
JGT (instruction)
JLT (instruction)

# Yeah, I know these definitely wouldn't work. They're only here for my convenience

PRINT (string_size location)
PRINTC (char)
PRINTN (number)
PRINTI (number)
PRINTF (number)
```

### Registers

First 13 registers are general purpose

Register 13 - Instruction pointer

Register 14 - Stack Pointer

Register 15 - Program Status Register

#### Program Status Register

2 bits for comparison (Error | Greater | Less | Equal)

1 bit for divide by zero flag

1 bit for carry flag

1 bit for zero flag

1 bit for sign flag

1 bit for overflow flag

Rest are reserved
