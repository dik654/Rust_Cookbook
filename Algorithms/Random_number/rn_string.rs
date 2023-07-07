use rand::{thread_rng, Rng};
//알파벳 분포 사용(u8 사용)
use rand::distributions::Alphanumeric;

fn main() {
    let rand_string: String = thread_rng()
        //랜덤 'a ~ z' 'A ~ Z' '0 ~ 9' iterator 생성
        .sample_iter(&Alphanumeric)
        //첫 30개 문자만 가져오기
        .take(30)
        //각 byte를 char로 변환
        .map(char::from)
        //생성된 char들을 String으로 수집
        .collect();

    println!("{}", rand_string)
}