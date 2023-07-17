use rayon::prelude::*;

fn main() {
    let v = vec![6, 2, 1, 9, 3, 8, 11];
    
    // &x가 iterator 참조
    // &&x가 값에 대한 참조
    // find_any는 조건에 만족하는 첫 번째 요소를 바로 리턴하는 함수
    let f1 = v.par_iter().find_any(|&&x| x == 9);
    let f2 = v.par_iter().find_any(|&&x| x % 2 == 0 && x > 6);
}