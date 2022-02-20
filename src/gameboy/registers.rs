pub struct FRegister {
    zero: bool,
    substract: bool,
    half_carry: bool,
    carry: bool,
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
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: FRegister,
    h: u8,
    l: u8,
    sp: u16,
    pc: u16,
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
