use crate::output_datastructures::{
    ControlWord, ACCUMULATOR, AND, INSTRUCTION, LOGIC_B, LOGIC_ZERO, MEMORY, MEMORY_ADDRESS, OR,
    PROGRAM_COUNTER, SHIFT_LEFT, SHIFT_RIGHT, SHIFT_ZERO, UNCHANGED, XOR,
};

macro_rules! ctrlVec {
    ( $( $x:expr ),* ) => {
        {
            vec!(
                ControlWord {
                    read_from: PROGRAM_COUNTER,
                    write_to: MEMORY_ADDRESS,
                    ..ControlWord::empty()
                },
                ControlWord {
                    read_from: MEMORY,
                    write_to: INSTRUCTION,
                    bank_select_enable: false,
                    program_counter_enable: true,
                    ..ControlWord::empty()
                },
                $(
                    $x,
                )*
                ControlWord {
                    step_reset: true,
                    ..ControlWord::empty()
                }
            )
        }
    };
}

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

impl Keyword {
    fn controlWords(&self) -> Vec<ControlWord> {
        match self {
            Keyword::Mov(f, t) => vec![],
            Keyword::Sub(op1, op2) => ctrlVec!(ControlWord {
                read_from: ACCUMULATOR,
                write_to: (*op1) as u8,
                alu_left: (*op1) as u8,
                alu_right: (*op2) as u8,
                alu_subtract: true,
                alu_shift: UNCHANGED,
                alu_logic: LOGIC_B,
                ..ControlWord::empty()
            }),
            Keyword::Add(op1, op2) => ctrlVec!(ControlWord {
                read_from: ACCUMULATOR,
                write_to: (*op1) as u8,
                alu_left: (*op1) as u8,
                alu_right: (*op2) as u8,
                alu_shift: UNCHANGED,
                alu_logic: LOGIC_B,
                ..ControlWord::empty()
            }),
            Keyword::And(op1, op2) => ctrlVec!(ControlWord {
                read_from: ACCUMULATOR,
                write_to: (*op1) as u8,
                alu_left: (*op1) as u8,
                alu_right: (*op2) as u8,
                alu_logic: AND,
                alu_shift: SHIFT_ZERO,
                ..ControlWord::empty()
            }),
            Keyword::Or(op1, op2) => ctrlVec!(ControlWord {
                read_from: ACCUMULATOR,
                write_to: (*op1) as u8,
                alu_left: (*op1) as u8,
                alu_right: (*op2) as u8,
                alu_logic: OR,
                alu_shift: SHIFT_ZERO,
                ..ControlWord::empty()
            }),
            Keyword::Xor(op1, op2) => ctrlVec!(ControlWord {
                read_from: ACCUMULATOR,
                write_to: (*op1) as u8,
                alu_left: (*op1) as u8,
                alu_right: (*op2) as u8,
                alu_logic: XOR,
                alu_shift: SHIFT_ZERO,
                ..ControlWord::empty()
            }),
            Keyword::Cmp(op1, op2) => ctrlVec!(ControlWord {
                alu_left: (*op1) as u8,
                alu_right: (*op2) as u8,
                alu_subtract: true,
                alu_shift: UNCHANGED,
                alu_logic: LOGIC_B,
                ..ControlWord::empty()
            }),
            Keyword::Shl(op1) => ctrlVec!(ControlWord {
                read_from: ACCUMULATOR,
                write_to: (*op1) as u8,
                alu_left: (*op1) as u8,
                alu_shift: SHIFT_LEFT,
                alu_logic: LOGIC_ZERO,
                ..ControlWord::empty()
            }),
            Keyword::Shr(op1) => ctrlVec!(ControlWord {
                read_from: ACCUMULATOR,
                write_to: (*op1) as u8,
                alu_left: (*op1) as u8,
                alu_shift: SHIFT_RIGHT,
                alu_logic: LOGIC_ZERO,
                ..ControlWord::empty()
            }),
            Keyword::Jmp(_) => ctrlVec!(
                ControlWord {
                    read_from: PROGRAM_COUNTER,
                    write_to: MEMORY_ADDRESS,
                    ..ControlWord::empty()
                },
                ControlWord {
                    read_from: MEMORY,
                    write_to: PROGRAM_COUNTER,
                    ..ControlWord::empty()
                }
            ),
            Keyword::Jc(addr) => ctrlVec!(),
            Keyword::Jz(addr) => ctrlVec!(),
            Keyword::Hlt => vec![ControlWord {
                halt: true,
                ..ControlWord::empty()
            }],
            Keyword::Nop => ctrlVec!(),
        }
    }
}

#[derive(Copy, Clone)]
enum GPR {
    A = 0,
    B,
    C,
    D,
}

#[derive(Copy, Clone)]
enum MovFrom {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    BS = 5,
    Acc = 6,
}

#[derive(Copy, Clone)]
enum MovTo {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    BS = 5,
    Out = 8,
}

fn generate() {}

#[cfg(test)]
mod tests {
    use super::*;

    fn fetchCycle() -> Vec<ControlWord> {
        vec![
            ControlWord {
                read_from: PROGRAM_COUNTER,
                write_to: MEMORY_ADDRESS,
                ..ControlWord::empty()
            },
            ControlWord {
                read_from: MEMORY,
                write_to: INSTRUCTION,
                bank_select_enable: false,
                program_counter_enable: true,
                ..ControlWord::empty()
            },
            ControlWord {
                step_reset: true,
                ..ControlWord::empty()
            },
        ]
    }

    #[test]
    fn ctrlVec_inserts_fetch_cycle() {
        let x = ctrlVec!();
        assert_eq!(fetchCycle(), x);
    }

    #[test]
    fn ctrlVec_inserts_given_values_before_step_reset() {
        let last_elem = ControlWord::empty();
        let x = ctrlVec!(last_elem);
        assert_eq!(4, x.len());
        assert_eq!(ControlWord::empty(), x[2]);
    }
}
