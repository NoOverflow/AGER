use std::collections::HashMap;

pub struct Debugger {
    pub translation_table: HashMap<i8, String>,
    pub translation_table_extended: HashMap<i8, String>,
}

impl Debugger {
    pub fn new() -> Debugger {
        Debugger {
            translation_table: HashMap::from([
                (0x0, "NOP".to_string()),
                (0x1, "LD BC, d16".to_string()),
                (0x2, "LD (BC), A".to_string()),
                (0x3, "INC BC".to_string()),
                (0x4, "INC B".to_string()),
                (0x5, "DEC B".to_string()),
                (0x6, "LD B, d8".to_string()),
                (0x7, "RLCA".to_string()),
                (0x8, "LD (a16), SP".to_string()),
                (0x9, "ADD HL, BC".to_string()),
                (0xA, "LD A, (BC)".to_string()),
                (0xB, "DEC BC".to_string()),
                (0xC, "INC C".to_string()),
                (0xD, "DEC C".to_string()),
                (0xE, "LD C, d8".to_string()),
                (0xF, "RRCA".to_string()),
                (0x10, "STOP 0".to_string()),
            ]),
            translation_table_extended: HashMap::from([(0x0, "RLC B".to_string())]),
        }
    }
}
