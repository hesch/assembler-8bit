use crate::output_datastructures::{
    ControlWord, ACCUMULATOR, AND, INSTRUCTION, LOGIC_B, LOGIC_ZERO, MEMORY, MEMORY_ADDRESS, OR,
    PROGRAM_COUNTER, SHIFT_LEFT, SHIFT_RIGHT, SHIFT_ZERO, UNCHANGED, XOR,
};

use gen_microcode::GenMicrocode;
use gen_microcode_macro::gen_microcode;
use field_size_macro::FieldSize;
use field_size::FieldSize;

macro_rules! ctrl_vec {
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

#[derive(gen_microcode)]
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
    fn control_words(&self) -> Vec<ControlWord> {
        match self {
            Keyword::Mov(from, to) => ctrl_vec!(ControlWord {
                read_from: (*from) as u8,
                write_to: (*to) as u8,
                ..ControlWord::empty()
            }),
            Keyword::Sub(op1, op2) => ctrl_vec!(ControlWord {
                read_from: ACCUMULATOR,
                write_to: (*op1) as u8,
                alu_left: (*op1) as u8,
                alu_right: (*op2) as u8,
                alu_subtract: true,
                alu_shift: UNCHANGED,
                alu_logic: LOGIC_B,
                ..ControlWord::empty()
            }),
            Keyword::Add(op1, op2) => ctrl_vec!(ControlWord {
                read_from: ACCUMULATOR,
                write_to: (*op1) as u8,
                alu_left: (*op1) as u8,
                alu_right: (*op2) as u8,
                alu_shift: UNCHANGED,
                alu_logic: LOGIC_B,
                ..ControlWord::empty()
            }),
            Keyword::And(op1, op2) => ctrl_vec!(ControlWord {
                read_from: ACCUMULATOR,
                write_to: (*op1) as u8,
                alu_left: (*op1) as u8,
                alu_right: (*op2) as u8,
                alu_logic: AND,
                alu_shift: SHIFT_ZERO,
                ..ControlWord::empty()
            }),
            Keyword::Or(op1, op2) => ctrl_vec!(ControlWord {
                read_from: ACCUMULATOR,
                write_to: (*op1) as u8,
                alu_left: (*op1) as u8,
                alu_right: (*op2) as u8,
                alu_logic: OR,
                alu_shift: SHIFT_ZERO,
                ..ControlWord::empty()
            }),
            Keyword::Xor(op1, op2) => ctrl_vec!(ControlWord {
                read_from: ACCUMULATOR,
                write_to: (*op1) as u8,
                alu_left: (*op1) as u8,
                alu_right: (*op2) as u8,
                alu_logic: XOR,
                alu_shift: SHIFT_ZERO,
                ..ControlWord::empty()
            }),
            Keyword::Cmp(op1, op2) => ctrl_vec!(ControlWord {
                alu_left: (*op1) as u8,
                alu_right: (*op2) as u8,
                alu_subtract: true,
                alu_shift: UNCHANGED,
                alu_logic: LOGIC_B,
                ..ControlWord::empty()
            }),
            Keyword::Shl(op1) => ctrl_vec!(ControlWord {
                read_from: ACCUMULATOR,
                write_to: (*op1) as u8,
                alu_left: (*op1) as u8,
                alu_shift: SHIFT_LEFT,
                alu_logic: LOGIC_ZERO,
                ..ControlWord::empty()
            }),
            Keyword::Shr(op1) => ctrl_vec!(ControlWord {
                read_from: ACCUMULATOR,
                write_to: (*op1) as u8,
                alu_left: (*op1) as u8,
                alu_shift: SHIFT_RIGHT,
                alu_logic: LOGIC_ZERO,
                ..ControlWord::empty()
            }),
            Keyword::Jmp(_) => ctrl_vec!(
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
            Keyword::Jc(addr) => ctrl_vec!(),
            Keyword::Jz(addr) => ctrl_vec!(),
            Keyword::Hlt => vec![ControlWord {
                halt: true,
                ..ControlWord::empty()
            }],
            Keyword::Nop => ctrl_vec!(),
        }
    }
}

#[derive(Copy, Clone, FieldSize)]
enum GPR {
    A = 0,
    B,
    C,
    D,
}

#[derive(Copy, Clone, FieldSize)]
enum MovFrom {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    BS = 5,
    Acc = 6,
}

#[derive(Copy, Clone, FieldSize)]
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

    fn fetch_cycle() -> Vec<ControlWord> {
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
    fn ctrl_vec_inserts_fetch_cycle() {
        let x = ctrl_vec!();
        assert_eq!(fetch_cycle(), x);
    }

    #[test]
    fn ctrl_vec_inserts_given_values_before_step_reset() {
        let last_elem = ControlWord::empty();
        let x = ctrl_vec!(last_elem);
        assert_eq!(4, x.len());
        assert_eq!(ControlWord::empty(), x[2]);
    }

    #[test]
    fn test() {
        Keyword::test();
    }
}
