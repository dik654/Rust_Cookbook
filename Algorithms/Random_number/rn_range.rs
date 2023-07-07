use rand::Rng;
use rand::distributions::{Distribution, Uniform};

fn just_random() {
    //랜덤 숫자 생성 인스턴스 반환
    let mut rng = rand::thread_rng();
    //0 ~ 10 사이의 정수값 프린트
    println!("Integer: {}", rng.gen_range(0..10));
    //0 ~ 10 사이의 실수값 프린트
    println!("Float: {}", rng.gen_range(0.0..10.0));
}
// 균일한 확률 분포를 이용한 랜덤 숫자 생성
// 조금 더 빠르다
fn random_with_uniform() {
    let mut rng = rand::thread_rng();
    //1 ~ 7 값에 균일한 확률 분포를 생성
    let die = Uniform::from(1..7);
    //while 루프와 동일
    loop {
        //1 ~ 6의 동일한 확률 분포에서 값을 샘플링
        let throw = die.sample(&mut rng);
        println!("Roll the die: {}", throw);
        //6이 나오면 종료
        if throw == 6 {
            break;
        }
    }
}

fn main() {
    just_random();
    random_with_uniform();
}