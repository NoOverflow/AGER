pub struct FRegister {
    pub zero: bool,
    pub substract: bool,
    pub half_carry: bool,
    pub carry: bool,
}

impl FRegister {
    pub fn new() -> Self {
        FRegister {
            zero: false,
            substract: false,
            half_carry: false,
            carry: false,
        }
    }
}

pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: FRegister,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub pc: u16,
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: FRegister::new(),
            h: 0,
            l: 0,
            sp: 0,
            pc: 0,
        }
    }
}
