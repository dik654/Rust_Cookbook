//랜덤 모듈 가져오기
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();

    //rand::Rng trait에서 구현을 제공해야 사용가능
    //uint8 "0" ~ "2^8 - 1" 범위에서 랜덤 숫자 리턴
    let n1: u8 = rng.gen();
    //uint16 범위의 랜덤 숫자 리턴
    let n2: u16 = rng.gen();
    //"!"는 매크로로 코드의 약자같은 역할
    //{}는 C언어의 %d, %f같은 역할
    println!("Random u8: {}", n1);
    
    // #[macro_export]
    // #[stable(feature = "rust1", since = "1.0.0")]
    // #[cfg_attr(not(test), rustc_diagnostic_item = "println_macro")]
    // #[allow_internal_unstable(print_internals, format_args_nl)]
    // macro_rules! println {
    //     () => {
    //         $crate::print!("\n")
    //     };
    //     ($($arg:tt)*) => {{
    //         $crate::io::_print($crate::format_args_nl!($($arg)*));
    //     }};
    // }
    println!("Random u16: {}", n2);
    //이렇게 <타입>을 붙여 따로 변수를 선언하지 않고도 바로 사용할 수 있다.
    println!("Random u32: {}", rng.gen::<u32>());
    println!("Random i32: {}", rng.gen::<i32>());
}