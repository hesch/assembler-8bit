const REGISTER_A: u8 = 0;
const REGISTER_B: u8 = 1;
const REGISTER_C: u8 = 2;
const REGISTER_D: u8 = 3;
const PROGRAM_COUNTER: u8 = 4;
const BANK_SELECT: u8 = 5;
const ACCUMULATOR: u8 = 6;
const MEMORY_ADDRESS: u8 = 6;
const MEMORY: u8 = 7;
const OUTPUT: u8 = 8;
const INSTRUCTION: u8 = 9;

// TODO: choose right values
const SHIFT_ZERO: u8 = 0;
const SHIFT_LEFT: u8 = 1;
const SHIFT_RIGHT: u8 = 2;
const UNCHANGED: u8 = 3;

// TODO: choose right values
const LOGIC_ZERO: u8 = 0;
const AND: u8 = 0;
const OR: u8 = 0;
const XOR: u8 = 0;
const NAND: u8 = 0;
const NOR: u8 = 0;
const XNOR: u8 = 0;
const LOGIC_A: u8 = 0;
const LOGIC_B: u8 = 0;
const NOT_A: u8 = 0;
const NOT_B: u8 = 0;
const A_AND_NOT_B: u8 = 0;
const B_AND_NOT_A: u8 = 0;
const A_OR_NOT_B: u8 = 0;
const B_OR_NOT_A: u8 = 0;
const ONES: u8 = 15;

pub struct ControlWord {
    write_to: u8,
    read_from: u8,
    alu_left: u8,
    alu_right: u8,
    alu_shift: u8,
    alu_logic: u8,
    alu_subtract: bool,
    program_counter_enable: bool,
    bank_select_enable: bool,
    halt: bool,
    step_reset: bool,
}

impl ControlWord {
    pub fn empty() -> ControlWord {
        ControlWord {
            write_to: REGISTER_A,
            read_from: REGISTER_A,
            alu_left: REGISTER_A,
            alu_right: REGISTER_A,
            alu_shift: SHIFT_ZERO,
            alu_logic: LOGIC_ZERO,
            alu_subtract: false,
            program_counter_enable: false,
            bank_select_enable: false,
            halt: false,
            step_reset: false,
        }
    }

    fn most_significant_bits(&self) -> u8 {
        self.write_to << 4 | self.read_from << 1 | self.alu_left >> 1
    }

    fn middle_bits(&self) -> u8 {
        self.alu_left << 7 | self.alu_right << 5 | self.alu_shift << 3 | self.alu_logic >> 1
    }

    fn least_significant_bits(&self) -> u8 {
        self.alu_logic << 7
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
            write_to: REGISTER_A,
            read_from: REGISTER_A,
            alu_left: REGISTER_A,
            alu_right: REGISTER_A,
            alu_shift: SHIFT_ZERO,
            alu_logic: LOGIC_ZERO,
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
            write_to: MEMORY,
            ..standard_control_word()
        };

        assert_eq!(0b01110000, control_word.most_significant_bits());
    }

    #[test]
    fn msb_conversion_uses_read_from() {
        let control_word = ControlWord {
            read_from: REGISTER_D,
            ..standard_control_word()
        };

        assert_eq!(0b00000110, control_word.most_significant_bits());
    }

    #[test]
    fn msb_conversion_uses_msb_from_alu_left() {
        let control_word = ControlWord {
            alu_left: REGISTER_C,
            ..standard_control_word()
        };

        assert_eq!(0b00000001, control_word.most_significant_bits());
    }

    #[test]
    fn msb_conversion_does_not_use_other_fields() {
        let control_word = ControlWord {
            alu_left: REGISTER_B,
            alu_right: REGISTER_D,
            alu_shift: UNCHANGED,
            alu_logic: ONES,
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
            alu_left: REGISTER_B,
            ..standard_control_word()
        };

        assert_eq!(0b10000000, control_word.middle_bits());
    }

    #[test]
    fn middle_bits_conversion_uses_alu_right() {
        let control_word = ControlWord {
            alu_right: REGISTER_D,
            ..standard_control_word()
        };

        assert_eq!(0b01100000, control_word.middle_bits());
    }

    #[test]
    fn middle_bits_conversion_uses_alu_shift() {
        let control_word = ControlWord {
            alu_shift: UNCHANGED,
            ..standard_control_word()
        };

        assert_eq!(0b00011000, control_word.middle_bits());
    }

    #[test]
    fn middle_bits_conversion_uses_alu_logic() {
        let control_word = ControlWord {
            alu_logic: ONES,
            ..standard_control_word()
        };

        assert_eq!(0b00000111, control_word.middle_bits());
    }

    #[test]
    fn middle_bits_conversion_uses_no_other_fields() {
        let control_word = ControlWord {
            write_to: BANK_SELECT,
            read_from: BANK_SELECT,
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
            alu_logic: ONES,
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
            write_to: REGISTER_D,
            read_from: REGISTER_D,
            alu_left: REGISTER_B,
            alu_right: REGISTER_D,
            alu_shift: UNCHANGED,
            alu_logic: OR,
            ..standard_control_word()
        };

        assert_eq!(0b00000000, control_word.least_significant_bits());
    }
}
