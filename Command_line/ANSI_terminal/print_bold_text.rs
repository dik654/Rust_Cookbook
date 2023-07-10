// 굵기등 스타일 변경을 위한 라이브러리
use ansi_term::Style;

fn main() {
    println!("{} and this is not",
            // 스타일 적용
            Style::new().bold().paint("This is bold"));
}