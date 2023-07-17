use rayon::prelude::*;

fn main() {
    let mut arr = [0, 7, 9, 11];
    // par_iter_mut로 mutable한 배열 병렬 반복자 생성
    // for_each로 클로저 병렬 실행
    arr.par_iter_mut().for_each(|p| *p -= 1);
    println!("{:?}", arr);
}