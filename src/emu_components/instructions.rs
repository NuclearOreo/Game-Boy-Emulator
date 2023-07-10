#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AddrMode {
    AM_IMP,
    AM_R_D16,
    AM_R_R,
    AM_MR_R,
    AM_R,
    AM_R_D8,
    AM_R_MR,
    AM_R_HLI,
    AM_R_HLD,
    AM_HLI_R,
    AM_HLD_R,
    AM_R_A8,
    AM_A8_R,
    AM_HL_SPR,
    AM_D16,
    AM_D8,
    AM_D16_R,
    AM_MR_D8,
    AM_MR,
    AM_A16_R,
    AM_R_A16,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RegType {
    RT_NONE,
    RT_A,
    RT_F,
    RT_B,
    RT_C,
    RT_D,
    RT_E,
    RT_H,
    RT_L,
    RT_AF,
    RT_BC,
    RT_DE,
    RT_HL,
    RT_SP,
    RT_PC,
}

impl RegType {
    pub fn is_16bit(self) -> bool {
        match self {
            RegType::RT_AF
            | RegType::RT_BC
            | RegType::RT_DE
            | RegType::RT_HL
            | RegType::RT_SP
            | RegType::RT_PC => true,
            _ => false,
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InType {
    IN_NONE,
    IN_NOP,
    IN_LD,
    IN_INC,
    IN_DEC,
    IN_RLCA,
    IN_ADD,
    IN_RRCA,
    IN_STOP,
    IN_RLA,
    IN_JR,
    IN_RRA,
    IN_DAA,
    IN_CPL,
    IN_SCF,
    IN_CCF,
    IN_HALT,
    IN_ADC,
    IN_SUB,
    IN_SBC,
    IN_AND,
    IN_XOR,
    IN_OR,
    IN_CP,
    IN_POP,
    IN_JP,
    IN_PUSH,
    IN_RET,
    IN_CB,
    IN_CALL,
    IN_RETI,
    IN_LDH,
    IN_JPHL,
    IN_DI,
    IN_EI,
    IN_RST,
    IN_ERR,
    //CB instructions...
    IN_RLC,
    IN_RRC,
    IN_RL,
    IN_RR,
    IN_SLA,
    IN_SRA,
    IN_SWAP,
    IN_SRL,
    IN_BIT,
    IN_RES,
    IN_SET,
}

impl std::fmt::Display for InType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let val = format!("{:?}", self);
        write!(f, "{:<7}", &val[3..])
    }
}

#[allow(non_camel_case_types, dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CondType {
    CT_NONE,
    CT_NZ,
    CT_Z,
    CT_NC,
    CT_C,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Instruction {
    pub i_type: InType,
    pub mode: AddrMode,
    pub reg_1: RegType,
    pub reg_2: RegType,
    pub cond: CondType,
    pub param: u8,
}

static mut INSTRUCTIONS: [Instruction; 0x100] = [Instruction {
    i_type: InType::IN_NONE,
    mode: AddrMode::AM_IMP,
    reg_1: RegType::RT_NONE,
    reg_2: RegType::RT_NONE,
    cond: CondType::CT_NONE,
    param: 0,
}; 0x100];

pub unsafe fn set_instructions() {
    INSTRUCTIONS[0x00] = Instruction {
        i_type: InType::IN_NOP,
        mode: AddrMode::AM_IMP,
        reg_1: RegType::RT_NONE,
        reg_2: RegType::RT_NONE,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0x01] = Instruction {
        i_type: InType::IN_LD,
        mode: AddrMode::AM_R_D16,
        reg_1: RegType::RT_BC,
        reg_2: RegType::RT_NONE,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0x02] = Instruction {
        i_type: InType::IN_LD,
        mode: AddrMode::AM_MR_R,
        reg_1: RegType::RT_BC,
        reg_2: RegType::RT_A,
        cond: CondType::CT_NONE,
        param: 0,
    };

    INSTRUCTIONS[0x05] = Instruction {
        i_type: InType::IN_DEC,
        mode: AddrMode::AM_R,
        reg_1: RegType::RT_B,
        reg_2: RegType::RT_NONE,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0x06] = Instruction {
        i_type: InType::IN_LD,
        mode: AddrMode::AM_R_D8,
        reg_1: RegType::RT_B,
        reg_2: RegType::RT_NONE,
        cond: CondType::CT_NONE,
        param: 0,
    };

    INSTRUCTIONS[0x08] = Instruction {
        i_type: InType::IN_LD,
        mode: AddrMode::AM_A16_R,
        reg_1: RegType::RT_NONE,
        reg_2: RegType::RT_SP,
        cond: CondType::CT_NONE,
        param: 0,
    };

    INSTRUCTIONS[0x0A] = Instruction {
        i_type: InType::IN_LD,
        mode: AddrMode::AM_R_MR,
        reg_1: RegType::RT_A,
        reg_2: RegType::RT_BC,
        cond: CondType::CT_NONE,
        param: 0,
    };

    INSTRUCTIONS[0x0E] = Instruction {
        i_type: InType::IN_LD,
        mode: AddrMode::AM_R_D8,
        reg_1: RegType::RT_C,
        reg_2: RegType::RT_NONE,
        cond: CondType::CT_NONE,
        param: 0,
    };

    // 0x1X
    INSTRUCTIONS[0x11] = Instruction {
        i_type: InType::IN_LD,
        mode: AddrMode::AM_R_D16,
        reg_1: RegType::RT_DE,
        reg_2: RegType::RT_NONE,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0x12] = Instruction {
        i_type: InType::IN_LD,
        mode: AddrMode::AM_MR_R,
        reg_1: RegType::RT_DE,
        reg_2: RegType::RT_A,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0x15] = Instruction {
        i_type: InType::IN_DEC,
        mode: AddrMode::AM_R,
        reg_1: RegType::RT_D,
        reg_2: RegType::RT_NONE,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0x16] = Instruction {
        i_type: InType::IN_LD,
        mode: AddrMode::AM_R_D8,
        reg_1: RegType::RT_D,
        reg_2: RegType::RT_NONE,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0x1A] = Instruction {
        i_type: InType::IN_LD,
        mode: AddrMode::AM_R_MR,
        reg_1: RegType::RT_A,
        reg_2: RegType::RT_DE,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0x1E] = Instruction {
        i_type: InType::IN_LD,
        mode: AddrMode::AM_R_D8,
        reg_1: RegType::RT_E,
        reg_2: RegType::RT_NONE,
        cond: CondType::CT_NONE,
        param: 0,
    };

    //0x2X
    INSTRUCTIONS[0x21] = Instruction {
        i_type: InType::IN_LD,
        mode: AddrMode::AM_R_D16,
        reg_1: RegType::RT_HL,
        reg_2: RegType::RT_NONE,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0x22] = Instruction {
        i_type: InType::IN_LD,
        mode: AddrMode::AM_HLI_R,
        reg_1: RegType::RT_HL,
        reg_2: RegType::RT_A,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0x25] = Instruction {
        i_type: InType::IN_DEC,
        mode: AddrMode::AM_R,
        reg_1: RegType::RT_H,
        reg_2: RegType::RT_NONE,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0x26] = Instruction {
        i_type: InType::IN_LD,
        mode: AddrMode::AM_R_D8,
        reg_1: RegType::RT_H,
        reg_2: RegType::RT_NONE,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0x2A] = Instruction {
        i_type: InType::IN_LD,
        mode: AddrMode::AM_R_HLI,
        reg_1: RegType::RT_A,
        reg_2: RegType::RT_HL,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0x2E] = Instruction {
        i_type: InType::IN_LD,
        mode: AddrMode::AM_R_D8,
        reg_1: RegType::RT_L,
        reg_2: RegType::RT_NONE,
        cond: CondType::CT_NONE,
        param: 0,
    };

    //0x3X
    INSTRUCTIONS[0x31] = Instruction {
        i_type: InType::IN_LD,
        mode: AddrMode::AM_R_D16,
        reg_1: RegType::RT_SP,
        reg_2: RegType::RT_NONE,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0x32] = Instruction {
        i_type: InType::IN_LD,
        mode: AddrMode::AM_HLD_R,
        reg_1: RegType::RT_HL,
        reg_2: RegType::RT_A,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0x35] = Instruction {
        i_type: InType::IN_DEC,
        mode: AddrMode::AM_R,
        reg_1: RegType::RT_HL,
        reg_2: RegType::RT_NONE,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0x36] = Instruction {
        i_type: InType::IN_LD,
        mode: AddrMode::AM_MR_D8,
        reg_1: RegType::RT_HL,
        reg_2: RegType::RT_NONE,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0x3A] = Instruction {
        i_type: InType::IN_LD,
        mode: AddrMode::AM_R_HLD,
        reg_1: RegType::RT_A,
        reg_2: RegType::RT_HL,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0x3E] = Instruction {
        i_type: InType::IN_LD,
        mode: AddrMode::AM_R_D8,
        reg_1: RegType::RT_A,
        reg_2: RegType::RT_NONE,
        cond: CondType::CT_NONE,
        param: 0,
    };

    //0x4X
    INSTRUCTIONS[0x40] = Instruction {
        i_type: InType::IN_LD,
        mode: AddrMode::AM_R_R,
        reg_1: RegType::RT_B,
        reg_2: RegType::RT_B,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0x41] = Instruction {
        i_type: InType::IN_LD,
        mode: AddrMode::AM_R_R,
        reg_1: RegType::RT_B,
        reg_2: RegType::RT_C,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0x42] = Instruction {
        i_type: InType::IN_LD,
        mode: AddrMode::AM_R_R,
        reg_1: RegType::RT_B,
        reg_2: RegType::RT_D,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0x43] = Instruction {
        i_type: InType::IN_LD,
        mode: AddrMode::AM_R_R,
        reg_1: RegType::RT_B,
        reg_2: RegType::RT_E,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0x44] = Instruction {
        i_type: InType::IN_LD,
        mode: AddrMode::AM_R_R,
        reg_1: RegType::RT_B,
        reg_2: RegType::RT_H,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0x45] = Instruction {
        i_type: InType::IN_LD,
        mode: AddrMode::AM_R_R,
        reg_1: RegType::RT_B,
        reg_2: RegType::RT_L,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0x46] = Instruction {
        i_type: InType::IN_LD,
        mode: AddrMode::AM_R_MR,
        reg_1: RegType::RT_B,
        reg_2: RegType::RT_HL,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0x47] = Instruction {
        i_type: InType::IN_LD,
        mode: AddrMode::AM_R_R,
        reg_1: RegType::RT_B,
        reg_2: RegType::RT_A,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0x48] = Instruction {
        i_type: InType::IN_LD,
        mode: AddrMode::AM_R_R,
        reg_1: RegType::RT_C,
        reg_2: RegType::RT_B,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0x49] = Instruction {
        i_type: InType::IN_LD,
        mode: AddrMode::AM_R_R,
        reg_1: RegType::RT_C,
        reg_2: RegType::RT_C,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0x4A] = Instruction {
        i_type: InType::IN_LD,
        mode: AddrMode::AM_R_R,
        reg_1: RegType::RT_C,
        reg_2: RegType::RT_D,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0x4B] = Instruction {
        i_type: InType::IN_LD,
        mode: AddrMode::AM_R_R,
        reg_1: RegType::RT_C,
        reg_2: RegType::RT_E,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0x4C] = Instruction {
        i_type: InType::IN_LD,
        mode: AddrMode::AM_R_R,
        reg_1: RegType::RT_C,
        reg_2: RegType::RT_H,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0x4D] = Instruction {
        i_type: InType::IN_LD,
        mode: AddrMode::AM_R_R,
        reg_1: RegType::RT_C,
        reg_2: RegType::RT_L,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0x4E] = Instruction {
        i_type: InType::IN_LD,
        mode: AddrMode::AM_R_MR,
        reg_1: RegType::RT_C,
        reg_2: RegType::RT_HL,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0x4F] = Instruction {
        i_type: InType::IN_LD,
        mode: AddrMode::AM_R_R,
        reg_1: RegType::RT_C,
        reg_2: RegType::RT_A,
        cond: CondType::CT_NONE,
        param: 0,
    };

    //0x5X
    INSTRUCTIONS[0x50] = Instruction {
        i_type: InType::IN_LD,
        mode: AddrMode::AM_R_R,
        reg_1: RegType::RT_D,
        reg_2: RegType::RT_B,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0x51] = Instruction {
        i_type: InType::IN_LD,
        mode: AddrMode::AM_R_R,
        reg_1: RegType::RT_D,
        reg_2: RegType::RT_C,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0x52] = Instruction {
        i_type: InType::IN_LD,
        mode: AddrMode::AM_R_R,
        reg_1: RegType::RT_D,
        reg_2: RegType::RT_D,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0x53] = Instruction {
        i_type: InType::IN_LD,
        mode: AddrMode::AM_R_R,
        reg_1: RegType::RT_D,
        reg_2: RegType::RT_E,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0x54] = Instruction {
        i_type: InType::IN_LD,
        mode: AddrMode::AM_R_R,
        reg_1: RegType::RT_D,
        reg_2: RegType::RT_H,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0x55] = Instruction {
        i_type: InType::IN_LD,
        mode: AddrMode::AM_R_R,
        reg_1: RegType::RT_D,
        reg_2: RegType::RT_L,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0x56] = Instruction {
        i_type: InType::IN_LD,
        mode: AddrMode::AM_R_MR,
        reg_1: RegType::RT_D,
        reg_2: RegType::RT_HL,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0x57] = Instruction {
        i_type: InType::IN_LD,
        mode: AddrMode::AM_R_R,
        reg_1: RegType::RT_D,
        reg_2: RegType::RT_A,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0x58] = Instruction {
        i_type: InType::IN_LD,
        mode: AddrMode::AM_R_R,
        reg_1: RegType::RT_E,
        reg_2: RegType::RT_B,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0x59] = Instruction {
        i_type: InType::IN_LD,
        mode: AddrMode::AM_R_R,
        reg_1: RegType::RT_E,
        reg_2: RegType::RT_C,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0x5A] = Instruction {
        i_type: InType::IN_LD,
        mode: AddrMode::AM_R_R,
        reg_1: RegType::RT_E,
        reg_2: RegType::RT_D,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0x5B] = Instruction {
        i_type: InType::IN_LD,
        mode: AddrMode::AM_R_R,
        reg_1: RegType::RT_E,
        reg_2: RegType::RT_E,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0x5C] = Instruction {
        i_type: InType::IN_LD,
        mode: AddrMode::AM_R_R,
        reg_1: RegType::RT_E,
        reg_2: RegType::RT_H,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0x5D] = Instruction {
        i_type: InType::IN_LD,
        mode: AddrMode::AM_R_R,
        reg_1: RegType::RT_E,
        reg_2: RegType::RT_L,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0x5E] = Instruction {
        i_type: InType::IN_LD,
        mode: AddrMode::AM_R_MR,
        reg_1: RegType::RT_E,
        reg_2: RegType::RT_HL,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0x5F] = Instruction {
        i_type: InType::IN_LD,
        mode: AddrMode::AM_R_R,
        reg_1: RegType::RT_E,
        reg_2: RegType::RT_A,
        cond: CondType::CT_NONE,
        param: 0,
    };

    //0x6X
    INSTRUCTIONS[0x60] = Instruction {
        i_type: InType::IN_LD,
        mode: AddrMode::AM_R_R,
        reg_1: RegType::RT_H,
        reg_2: RegType::RT_B,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0x61] = Instruction {
        i_type: InType::IN_LD,
        mode: AddrMode::AM_R_R,
        reg_1: RegType::RT_H,
        reg_2: RegType::RT_C,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0x62] = Instruction {
        i_type: InType::IN_LD,
        mode: AddrMode::AM_R_R,
        reg_1: RegType::RT_H,
        reg_2: RegType::RT_D,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0x63] = Instruction {
        i_type: InType::IN_LD,
        mode: AddrMode::AM_R_R,
        reg_1: RegType::RT_H,
        reg_2: RegType::RT_E,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0x64] = Instruction {
        i_type: InType::IN_LD,
        mode: AddrMode::AM_R_R,
        reg_1: RegType::RT_H,
        reg_2: RegType::RT_H,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0x65] = Instruction {
        i_type: InType::IN_LD,
        mode: AddrMode::AM_R_R,
        reg_1: RegType::RT_H,
        reg_2: RegType::RT_L,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0x66] = Instruction {
        i_type: InType::IN_LD,
        mode: AddrMode::AM_R_MR,
        reg_1: RegType::RT_H,
        reg_2: RegType::RT_HL,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0x67] = Instruction {
        i_type: InType::IN_LD,
        mode: AddrMode::AM_R_R,
        reg_1: RegType::RT_H,
        reg_2: RegType::RT_A,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0x68] = Instruction {
        i_type: InType::IN_LD,
        mode: AddrMode::AM_R_R,
        reg_1: RegType::RT_L,
        reg_2: RegType::RT_B,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0x69] = Instruction {
        i_type: InType::IN_LD,
        mode: AddrMode::AM_R_R,
        reg_1: RegType::RT_L,
        reg_2: RegType::RT_C,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0x6A] = Instruction {
        i_type: InType::IN_LD,
        mode: AddrMode::AM_R_R,
        reg_1: RegType::RT_L,
        reg_2: RegType::RT_D,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0x6B] = Instruction {
        i_type: InType::IN_LD,
        mode: AddrMode::AM_R_R,
        reg_1: RegType::RT_L,
        reg_2: RegType::RT_E,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0x6C] = Instruction {
        i_type: InType::IN_LD,
        mode: AddrMode::AM_R_R,
        reg_1: RegType::RT_L,
        reg_2: RegType::RT_H,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0x6D] = Instruction {
        i_type: InType::IN_LD,
        mode: AddrMode::AM_R_R,
        reg_1: RegType::RT_L,
        reg_2: RegType::RT_L,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0x6E] = Instruction {
        i_type: InType::IN_LD,
        mode: AddrMode::AM_R_MR,
        reg_1: RegType::RT_L,
        reg_2: RegType::RT_HL,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0x6F] = Instruction {
        i_type: InType::IN_LD,
        mode: AddrMode::AM_R_R,
        reg_1: RegType::RT_L,
        reg_2: RegType::RT_A,
        cond: CondType::CT_NONE,
        param: 0,
    };

    //0x7X
    INSTRUCTIONS[0x70] = Instruction {
        i_type: InType::IN_LD,
        mode: AddrMode::AM_MR_R,
        reg_1: RegType::RT_HL,
        reg_2: RegType::RT_B,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0x71] = Instruction {
        i_type: InType::IN_LD,
        mode: AddrMode::AM_MR_R,
        reg_1: RegType::RT_HL,
        reg_2: RegType::RT_C,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0x72] = Instruction {
        i_type: InType::IN_LD,
        mode: AddrMode::AM_MR_R,
        reg_1: RegType::RT_HL,
        reg_2: RegType::RT_D,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0x73] = Instruction {
        i_type: InType::IN_LD,
        mode: AddrMode::AM_MR_R,
        reg_1: RegType::RT_HL,
        reg_2: RegType::RT_E,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0x74] = Instruction {
        i_type: InType::IN_LD,
        mode: AddrMode::AM_MR_R,
        reg_1: RegType::RT_HL,
        reg_2: RegType::RT_H,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0x75] = Instruction {
        i_type: InType::IN_LD,
        mode: AddrMode::AM_MR_R,
        reg_1: RegType::RT_HL,
        reg_2: RegType::RT_L,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0x76] = Instruction {
        i_type: InType::IN_HALT,
        mode: AddrMode::AM_IMP,
        reg_1: RegType::RT_NONE,
        reg_2: RegType::RT_NONE,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0x77] = Instruction {
        i_type: InType::IN_LD,
        mode: AddrMode::AM_MR_R,
        reg_1: RegType::RT_HL,
        reg_2: RegType::RT_A,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0x78] = Instruction {
        i_type: InType::IN_LD,
        mode: AddrMode::AM_R_R,
        reg_1: RegType::RT_A,
        reg_2: RegType::RT_B,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0x79] = Instruction {
        i_type: InType::IN_LD,
        mode: AddrMode::AM_R_R,
        reg_1: RegType::RT_A,
        reg_2: RegType::RT_C,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0x7A] = Instruction {
        i_type: InType::IN_LD,
        mode: AddrMode::AM_R_R,
        reg_1: RegType::RT_A,
        reg_2: RegType::RT_D,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0x7B] = Instruction {
        i_type: InType::IN_LD,
        mode: AddrMode::AM_R_R,
        reg_1: RegType::RT_A,
        reg_2: RegType::RT_E,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0x7C] = Instruction {
        i_type: InType::IN_LD,
        mode: AddrMode::AM_R_R,
        reg_1: RegType::RT_A,
        reg_2: RegType::RT_H,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0x7D] = Instruction {
        i_type: InType::IN_LD,
        mode: AddrMode::AM_R_R,
        reg_1: RegType::RT_A,
        reg_2: RegType::RT_L,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0x7E] = Instruction {
        i_type: InType::IN_LD,
        mode: AddrMode::AM_R_MR,
        reg_1: RegType::RT_A,
        reg_2: RegType::RT_HL,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0x7F] = Instruction {
        i_type: InType::IN_LD,
        mode: AddrMode::AM_R_R,
        reg_1: RegType::RT_A,
        reg_2: RegType::RT_A,
        cond: CondType::CT_NONE,
        param: 0,
    };

    INSTRUCTIONS[0xAF] = Instruction {
        i_type: InType::IN_XOR,
        mode: AddrMode::AM_R,
        reg_1: RegType::RT_A,
        reg_2: RegType::RT_NONE,
        cond: CondType::CT_NONE,
        param: 0,
    };

    // 0xCX
    INSTRUCTIONS[0xC1] = Instruction {
        i_type: InType::IN_POP,
        mode: AddrMode::AM_R,
        reg_1: RegType::RT_BC,
        reg_2: RegType::RT_NONE,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0xC3] = Instruction {
        i_type: InType::IN_JP,
        mode: AddrMode::AM_D16,
        reg_1: RegType::RT_NONE,
        reg_2: RegType::RT_NONE,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0xC4] = Instruction {
        i_type: InType::IN_CALL,
        mode: AddrMode::AM_D16,
        reg_1: RegType::RT_NONE,
        reg_2: RegType::RT_NONE,
        cond: CondType::CT_NZ,
        param: 0,
    };
    INSTRUCTIONS[0xC5] = Instruction {
        i_type: InType::IN_PUSH,
        mode: AddrMode::AM_R,
        reg_1: RegType::RT_BC,
        reg_2: RegType::RT_NONE,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0xCC] = Instruction {
        i_type: InType::IN_CALL,
        mode: AddrMode::AM_D16,
        reg_1: RegType::RT_NONE,
        reg_2: RegType::RT_NONE,
        cond: CondType::CT_Z,
        param: 0,
    };
    INSTRUCTIONS[0xCD] = Instruction {
        i_type: InType::IN_CALL,
        mode: AddrMode::AM_D16,
        reg_1: RegType::RT_NONE,
        reg_2: RegType::RT_NONE,
        cond: CondType::CT_NONE,
        param: 0,
    };

    // 0xDX
    INSTRUCTIONS[0xD1] = Instruction {
        i_type: InType::IN_POP,
        mode: AddrMode::AM_R,
        reg_1: RegType::RT_DE,
        reg_2: RegType::RT_NONE,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0xD4] = Instruction {
        i_type: InType::IN_CALL,
        mode: AddrMode::AM_D16,
        reg_1: RegType::RT_NONE,
        reg_2: RegType::RT_NONE,
        cond: CondType::CT_NC,
        param: 0,
    };
    INSTRUCTIONS[0xD5] = Instruction {
        i_type: InType::IN_PUSH,
        mode: AddrMode::AM_R,
        reg_1: RegType::RT_DE,
        reg_2: RegType::RT_NONE,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0xDC] = Instruction {
        i_type: InType::IN_CALL,
        mode: AddrMode::AM_D16,
        reg_1: RegType::RT_NONE,
        reg_2: RegType::RT_NONE,
        cond: CondType::CT_C,
        param: 0,
    };

    // 0xEX
    INSTRUCTIONS[0xE0] = Instruction {
        i_type: InType::IN_LDH,
        mode: AddrMode::AM_A8_R,
        reg_1: RegType::RT_NONE,
        reg_2: RegType::RT_A,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0xE1] = Instruction {
        i_type: InType::IN_POP,
        mode: AddrMode::AM_R,
        reg_1: RegType::RT_HL,
        reg_2: RegType::RT_NONE,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0xE2] = Instruction {
        i_type: InType::IN_LD,
        mode: AddrMode::AM_R_MR,
        reg_1: RegType::RT_C,
        reg_2: RegType::RT_A,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0xE5] = Instruction {
        i_type: InType::IN_PUSH,
        mode: AddrMode::AM_R,
        reg_1: RegType::RT_HL,
        reg_2: RegType::RT_NONE,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0xEA] = Instruction {
        i_type: InType::IN_LD,
        mode: AddrMode::AM_A16_R,
        reg_1: RegType::RT_NONE,
        reg_2: RegType::RT_A,
        cond: CondType::CT_NONE,
        param: 0,
    };

    //0xFX
    INSTRUCTIONS[0xF0] = Instruction {
        i_type: InType::IN_LDH,
        mode: AddrMode::AM_R_A8,
        reg_1: RegType::RT_A,
        reg_2: RegType::RT_NONE,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0xF1] = Instruction {
        i_type: InType::IN_POP,
        mode: AddrMode::AM_R,
        reg_1: RegType::RT_AF,
        reg_2: RegType::RT_NONE,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0xF2] = Instruction {
        i_type: InType::IN_LD,
        mode: AddrMode::AM_R_MR,
        reg_1: RegType::RT_A,
        reg_2: RegType::RT_C,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0xF3] = Instruction {
        i_type: InType::IN_DI,
        mode: AddrMode::AM_IMP,
        reg_1: RegType::RT_NONE,
        reg_2: RegType::RT_NONE,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0xE5] = Instruction {
        i_type: InType::IN_PUSH,
        mode: AddrMode::AM_R,
        reg_1: RegType::RT_AF,
        reg_2: RegType::RT_NONE,
        cond: CondType::CT_NONE,
        param: 0,
    };
    INSTRUCTIONS[0xFA] = Instruction {
        i_type: InType::IN_LD,
        mode: AddrMode::AM_R_A16,
        reg_1: RegType::RT_A,
        reg_2: RegType::RT_NONE,
        cond: CondType::CT_NONE,
        param: 0,
    };
}

pub unsafe fn instruction_by_opcode(code: u8) -> Option<Instruction> {
    let code = code as usize;
    if INSTRUCTIONS[code].i_type == InType::IN_NONE {
        return None;
    }

    Some(INSTRUCTIONS[code])
}
