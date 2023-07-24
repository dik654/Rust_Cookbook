use bitflags::bitflags;
use std::fmt;
// bitflags 매크로로 비트 플래그 MyFlags 타입 정의
// bit연산을 위한 여러 메서드와 Debug 트레이트 제공
bitflags! {
    struct MyFlags: u32 {
        const FLAG_A = 0b00000001;
        const FLAG_B = 0b00000010;
        const FLAG_C = 0b00000100;
        const FLAG_ABC = Self::FLAG_A.bits
                       | Self::FLAG_B.bits
                       | Self::FLAG_C.bits;
    }
}
// bitflags 매크로가 따로 제공하지 않는 reset하는 메서드 선언
impl MyFlags {
    pub fn clear(&mut self) -> &mut MyFlags {
        self.bits = 0;
        self
    }
}
// MyFlags 타입의 표준 출력 포맷 구현
impl fmt::Display for MyFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // bits값을 binary로 32비트 길이로 출력
        write!(f, "{:032b}", self.bits)
    }
}

fn main() {
    // 비트연산 
    let e1 = MyFlags::FLAG_A | MyFlags::FLAG_C;
    let e2 = MyFlags::FLAG_B | MyFlags::FLAG_C;
    assert_eq!((e1 | e2), MyFlags::FLAG_ABC);
    assert_eq!((e1 & e2), MyFlags::FLAG_C);
    assert_eq!((e1 - e2), MyFlags::FLAG_A);
    assert_eq!(!e2, MyFlags::FLAG_A);

    let mut flags = MyFlags::FLAG_ABC;
    // 16쌍의 00
    assert_eq!(format!("{}", flags), "00000000000000000000000000000111");
    assert_eq!(format!("{}", flags.clear()), "00000000000000000000000000000000");
    // bitflags 매크로가 자동으로 선언한 Debug 트레이트가 binary bit값이 아닌 FLAG_B 변수명으로 그대로 리턴한다는 것을 보여줌
    assert_eq!(format!("{:?}", MyFlags::FLAG_B), "FLAG_B");
    assert_eq!(format!("{:?}", MyFlags::FLAG_A | MyFlags::FLAG_B), "FLAG_A | FLAG_B");
}