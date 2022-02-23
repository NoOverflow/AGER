pub struct BinUtils {}

impl BinUtils {
    pub fn u16_from_u8s(high: u8, low: u8) -> u16 {
        return (low as u16) | ((high as u16) << 8);
    }

    //(High, Low)
    pub fn u8s_from_u16(v: u16) -> (u8, u8) {
        ((v >> 8) as u8, ((v & 0xFF) as u8))
    }
}

#[test]
fn get_u8s_from_u16() {
    let result: (u8, u8) = BinUtils::u8s_from_u16(0xADEF);

    assert_eq!(result.0, 0xAD);
    assert_eq!(result.1, 0xEF);
}

#[test]
fn get_u16_from_u8s() {
    let result: u16 = BinUtils::u16_from_u8s(0xAD, 0xEF);

    assert_eq!(result, 0xADEF);
}
