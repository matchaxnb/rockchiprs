pub fn default_vid(vendor_id: Option<u16>) -> u16 {
    match vendor_id {
        Some(0) => 0x2207,
        Some(v) => v,
        None => 0x2207,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn without_value() {
        let nothing = None;
        match default_vid(nothing) {
            0x2207 => println!("test 1 ok!"),
            _ => panic!("test 1 failed!"),
        }
    }
    #[test]
    fn with_zero() {
        let zero = Some(0x0000);
        match default_vid(zero) {
            0x2207 => println!("test zero ok!"),
            _ => panic!("test zero failed!"),
        }
    }
    #[test]
    fn with_rockchip() {
        let rockchip = Some(0x2207);
        match default_vid(rockchip) {
            0x2207 => println!("test 2 ok!"),
            _ => panic!("test 2 failed!"),
        }
    }

    #[test]
    fn with_other() {
        let arty = Some(0x1c75);
        match default_vid(arty) {
            0x1c75 => println!("test 3 ok!"),
            _ => panic!("test 3 failed!"),
        }
    }
}
