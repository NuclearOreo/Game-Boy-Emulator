#[derive(Debug, Clone, Copy)]
enum AddrMode {
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

#[derive(Debug, Clone, Copy)]
enum RegType {
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

#[derive(Debug, Clone, Copy)]
enum InType {
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

#[derive(Debug, Clone, Copy)]
enum CondType {
    CT_NONE,
    CT_NZ,
    CT_Z,
    CT_NC,
    CT_C,
}

#[derive(Debug, Clone, Copy)]
pub struct Instruction {
    i_type: InType,
    mode: AddrMode,
    reg_1: RegType,
    reg_2: RegType,
    cond: CondType,
    param: u8,
}

static mut instructions: [Instruction; 0x100] = [Instruction {
    i_type: InType::IN_NONE,
    mode: AddrMode::AM_IMP,
    reg_1: RegType::RT_NONE,
    reg_2: RegType::RT_NONE,
    cond: CondType::CT_NONE,
    param: 0,
}; 0x100];

fn get_instructions() -> &'static mut [Instruction; 0x100] {
    unsafe { &mut instructions }
}

pub fn set_instuctions() {
    unsafe {
        instructions[0x00] = Instruction {
            i_type: InType::IN_NOP,
            mode: AddrMode::AM_IMP,
            reg_1: RegType::RT_NONE,
            reg_2: RegType::RT_NONE,
            cond: CondType::CT_NONE,
            param: 0,
        };
        instructions[0x05] = Instruction {
            i_type: InType::IN_DEC,
            mode: AddrMode::AM_R,
            reg_1: RegType::RT_B,
            reg_2: RegType::RT_NONE,
            cond: CondType::CT_NONE,
            param: 0,
        };
        instructions[0xAF] = Instruction {
            i_type: InType::IN_LD,
            mode: AddrMode::AM_R_D8,
            reg_1: RegType::RT_C,
            reg_2: RegType::RT_NONE,
            cond: CondType::CT_NONE,
            param: 0,
        };
        instructions[0xC3] = Instruction {
            i_type: InType::IN_JP,
            mode: AddrMode::AM_D16,
            reg_1: RegType::RT_NONE,
            reg_2: RegType::RT_NONE,
            cond: CondType::CT_NONE,
            param: 0,
        };
        instructions[0xF3] = Instruction {
            i_type: InType::IN_DI,
            mode: AddrMode::AM_IMP,
            reg_1: RegType::RT_NONE,
            reg_2: RegType::RT_NONE,
            cond: CondType::CT_NONE,
            param: 0,
        };
    }
}
