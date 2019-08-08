#[derive(Copy, Clone)]
enum ComponentReadAddress {
    RegisterA = 0,
    RegisterB,
    RegisterC,
    RegisterD,
    ProgramCounter,
    BankSelect,
    Accumulator,
    Memory,
}

#[derive(Copy, Clone)]
enum ComponentWriteAddress {
    RegisterA = 0,
    RegisterB,
    RegisterC,
    RegisterD,
    ProgramCounter,
    BankSelect,
    MemoryAddress,
    Memory,
    Output,
    Instruction,
}

#[derive(Copy, Clone)]
enum RegisterAddress {
    RegisterA = 0,
    RegisterB,
    RegisterC,
    RegisterD,
}

// TODO: order
#[derive(Copy, Clone)]
enum ShiftOperation {
    Zero = 0,
    ShiftLeft,
    ShiftRight,
    Unchanged,
}

// TODO: order
#[derive(Copy, Clone)]
enum LogicOperation {
    Zero = 0,
    And,
    Or,
    Xor,
    A,
    B,
    NotA,
    NotB,
    Nand,
    Nor,
    Xnor,
    AandNotB,
    BandNotA,
    AorNotB,
    BorNotA,
    One,
}

pub struct ControlWord {
    write_to: ComponentWriteAddress,
    read_from: ComponentReadAddress,
    alu_left: RegisterAddress,
    alu_right: RegisterAddress,
    alu_shift: ShiftOperation,
    alu_logic: LogicOperation,
    alu_subtract: bool,
    program_counter_enable: bool,
    bank_select_enable: bool,
    halt: bool,
    step_reset: bool,
}

impl ControlWord {
    pub fn empty() -> ControlWord {
        ControlWord {
            write_to: ComponentWriteAddress::RegisterA,
            read_from: ComponentReadAddress::RegisterA,
            alu_left: RegisterAddress::RegisterA,
            alu_right: RegisterAddress::RegisterA,
            alu_shift: ShiftOperation::Zero,
            alu_logic: LogicOperation::Zero,
            alu_subtract: false,
            program_counter_enable: false,
            bank_select_enable: false,
            halt: false,
            step_reset: false,
        }
    }

    fn most_significant_bits(&self) -> u8 {
        (self.write_to as u8) << 4 | (self.read_from as u8) << 1 | (self.alu_left as u8) >> 1
    }

    fn middle_bits(&self) -> u8 {
        (self.alu_left as u8) << 7
            | (self.alu_right as u8) << 5
            | (self.alu_shift as u8) << 3
            | (self.alu_logic as u8) >> 1
    }

    fn least_significant_bits(&self) -> u8 {
        (self.alu_logic as u8) << 7
            | (self.alu_subtract as u8) << 6
            | (self.program_counter_enable as u8) << 5
            | (self.bank_select_enable as u8) << 4
            | (self.halt as u8) << 3
            | (self.step_reset as u8) << 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn standard_control_word() -> ControlWord {
        ControlWord {
            write_to: ComponentWriteAddress::RegisterA,
            read_from: ComponentReadAddress::RegisterA,
            alu_left: RegisterAddress::RegisterA,
            alu_right: RegisterAddress::RegisterA,
            alu_shift: ShiftOperation::Zero,
            alu_logic: LogicOperation::Zero,
            alu_subtract: false,
            program_counter_enable: false,
            bank_select_enable: false,
            halt: false,
            step_reset: false,
        }
    }

    #[test]
    fn msb_conversion_uses_write_to() {
        let control_word = ControlWord {
            write_to: ComponentWriteAddress::Memory,
            ..standard_control_word()
        };

        assert_eq!(0b01110000, control_word.most_significant_bits());
    }

    #[test]
    fn msb_conversion_uses_read_from() {
        let control_word = ControlWord {
            read_from: ComponentReadAddress::RegisterD,
            ..standard_control_word()
        };

        assert_eq!(0b00000110, control_word.most_significant_bits());
    }

    #[test]
    fn msb_conversion_uses_msb_from_alu_left() {
        let control_word = ControlWord {
            alu_left: RegisterAddress::RegisterC,
            ..standard_control_word()
        };

        assert_eq!(0b00000001, control_word.most_significant_bits());
    }

    #[test]
    fn msb_conversion_does_not_use_other_fields() {
        let control_word = ControlWord {
            alu_left: RegisterAddress::RegisterB,
            alu_right: RegisterAddress::RegisterD,
            alu_shift: ShiftOperation::Unchanged,
            alu_logic: LogicOperation::One,
            alu_subtract: true,
            program_counter_enable: true,
            bank_select_enable: true,
            halt: true,
            step_reset: true,
            ..standard_control_word()
        };

        assert_eq!(0b00000000, control_word.most_significant_bits());
    }

    #[test]
    fn middle_bits_conversion_uses_lsb_from_alu_left() {
        let control_word = ControlWord {
            alu_left: RegisterAddress::RegisterB,
            ..standard_control_word()
        };

        assert_eq!(0b10000000, control_word.middle_bits());
    }

    #[test]
    fn middle_bits_conversion_uses_alu_right() {
        let control_word = ControlWord {
            alu_right: RegisterAddress::RegisterD,
            ..standard_control_word()
        };

        assert_eq!(0b01100000, control_word.middle_bits());
    }

    #[test]
    fn middle_bits_conversion_uses_alu_shift() {
        let control_word = ControlWord {
            alu_shift: ShiftOperation::Unchanged,
            ..standard_control_word()
        };

        assert_eq!(0b00011000, control_word.middle_bits());
    }

    #[test]
    fn middle_bits_conversion_uses_alu_logic() {
        let control_word = ControlWord {
            alu_logic: LogicOperation::One,
            ..standard_control_word()
        };

        assert_eq!(0b00000111, control_word.middle_bits());
    }

    #[test]
    fn middle_bits_conversion_uses_no_other_fields() {
        let control_word = ControlWord {
            write_to: ComponentWriteAddress::BankSelect,
            read_from: ComponentReadAddress::BankSelect,
            alu_subtract: true,
            program_counter_enable: true,
            bank_select_enable: true,
            halt: true,
            step_reset: true,
            ..standard_control_word()
        };

        assert_eq!(0b00000000, control_word.middle_bits());
    }

    #[test]
    fn lsb_conversion_uses_alu_logic() {
        let control_word = ControlWord {
            alu_logic: LogicOperation::One,
            ..standard_control_word()
        };

        assert_eq!(0b10000000, control_word.least_significant_bits());
    }

    #[test]
    fn lsb_conversion_uses_alu_subtract() {
        let control_word = ControlWord {
            alu_subtract: true,
            ..standard_control_word()
        };

        assert_eq!(0b01000000, control_word.least_significant_bits());
    }

    #[test]
    fn lsb_conversion_uses_program_counter_enable() {
        let control_word = ControlWord {
            program_counter_enable: true,
            ..standard_control_word()
        };

        assert_eq!(0b00100000, control_word.least_significant_bits());
    }

    #[test]
    fn lsb_conversion_uses_bank_select_enable() {
        let control_word = ControlWord {
            bank_select_enable: true,
            ..standard_control_word()
        };

        assert_eq!(0b00010000, control_word.least_significant_bits());
    }

    #[test]
    fn lsb_conversion_uses_halt() {
        let control_word = ControlWord {
            halt: true,
            ..standard_control_word()
        };

        assert_eq!(0b00001000, control_word.least_significant_bits());
    }

    #[test]
    fn lsb_conversion_uses_step_reset() {
        let control_word = ControlWord {
            step_reset: true,
            ..standard_control_word()
        };

        assert_eq!(0b00000100, control_word.least_significant_bits());
    }

    #[test]
    fn lsb_conversion_does_not_use_other_flags() {
        let control_word = ControlWord {
            write_to: ComponentWriteAddress::RegisterD,
            read_from: ComponentReadAddress::RegisterD,
            alu_left: RegisterAddress::RegisterB,
            alu_right: RegisterAddress::RegisterD,
            alu_shift: ShiftOperation::Unchanged,
            alu_logic: LogicOperation::Or,
            ..standard_control_word()
        };

        assert_eq!(0b00000000, control_word.least_significant_bits());
    }
}
