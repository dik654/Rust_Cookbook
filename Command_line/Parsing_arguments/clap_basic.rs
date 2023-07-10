// 인수를 사용할 때 사용하는 라이브러리
// Arg로 인수의 이름, 설명 등을 선언하고, App을 통해 인수를 받는다 
use clap::{Arg, App};

fn main() {
    // new 아래 인수는 첫 번째 줄에 print할 text내용
    let matches = App::new("My Test Program")
    // text 옆에 붙을 버전
        .version("0.1.0")
        // 아래에 붙을 제작자명
        .author("kimmalee <kimmalee1577@gmail.com>")
        // 그 아래 붙을 프로그램 설명
        .about("studying parsing arguments")
        // 인수에 대한 선언 시작 1
        .arg(Arg::with_name("file")
            .short("f")
            .long("file")
            .take_value(true)
            .help("goody file"))
        // 인수에 대한 선언 시작 2
        .arg(Arg::with_name("num")
            .short("n")
            .long("number")
            .take_value(true)
            .help("five less than your favorite number"))
            // 사용자가 제공한 인수들 파싱하여 matches에 저장
            .get_matches();
    // file인수로 제공된 값 myfile로 저장
    let myfile = matches.value_of("file").unwrap_or("input.txt");
    // 콘솔에 print
    println!("The file passed is: {}", myfile);
    // num인수로 제공된 값 num_str으로 저장
    let num_str = matches.value_of("num"); 
    match num_str {
        // 값이 없다면 모르겠다는 내용 콘솔에 print
        None => println!("No idea what your favorite number is."),
        // 값이 있다면
        Some(s) => {
            // string을 i32로 변환하고
            match s.parse::<i32>() {
                // 숫자가 맞다면 숫자 + 5로 좋아하는 숫자 리턴
                Ok(n) => println!("Your favorite number must be {}.", n + 5),
                // 에러가 난다면 받은 값과 숫자가 아니라는 메세지를 콘솔에 print
                Err(_) => println!("That's not a number! {}", s),
            }
        }
    }
}