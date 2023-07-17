use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;
use rayon::prelude::*;

fn main() {
    let mut vec = vec![String::new(); 100_000];
    vec.par_iter_mut().for_each(|p| {
        let mut rng = thread_rng();
        // 5의 길이를 가진 "0 ~ 9", "A ~ Z", "a ~ z" 사이의 랜덤값 생성 
        // ex. 00nJb
        *p = (0..5).map(|_| rng.sample(&Alphanumeric) as char).collect()
    });
    // 생성한 vec에 대해 unstable하게 동시성 정렬 실행
    // unstable한 것은 같은 값의 경우 index가 달라질 수 있는 것
    vec.par_sort_unstable();
    println!("{:?}", vec);
}