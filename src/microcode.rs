enum Keyword {
    Mov(MovFrom, MovTo),
    Sub(GPR, GPR),
    Add(GPR, GPR),
    And(GPR, GPR),
    Or(GPR, GPR),
    Xor(GPR, GPR),
    Cmp(GPR, GPR),
    Shl(GPR),
    Shr(GPR),
    Jmp(u8),
    Jc(u8),
    Jz(u8),
    Hlt,
    Nop,
}

enum GPR {
    A = 0,
    B,
    C,
    D,
}

enum MovFrom  {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    BS = 5,
    Acc = 6,
    Addr(u8),
}

enum MovTo  {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    BS = 5,
    Out = 9,
    Addr(u8),
}

fn generate() {
}


