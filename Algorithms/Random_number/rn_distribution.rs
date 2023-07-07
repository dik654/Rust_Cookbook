use rand_distr::{Distribution, Normal, NormalError};
use rand::thread_rng;

fn main() -> Result<(), NormalError> {
    let mut rng = thread_rng();
    //2 ~ 3 사이의 정규분포 생성
    //?는 오류 발생시 함수 종료 후 오류 리턴
    let normal = Normal::new(2.0, 3.0)?;
    //정규 분포에서 샘플링
    let v = normal.sample(&mut rng);
    println!("{} is from a N(2, 9) distribution", v);
    //main함수가 성공했다는 Result값
    //내부 ()는 리턴 값이 없다는 뜻
    Ok(())
}