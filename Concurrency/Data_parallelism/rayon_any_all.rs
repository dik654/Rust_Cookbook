use rayon::prelude::*;

fn main() {
    let mut vec = vec![2, 4, 6, 8];
    // any는 조건에 해당하는 값이 하나라도 있는지 체크
    assert!(!vec.par_iter().any(|n| (*n % 2) != 0));
    // iterate한 모든 값들이 조건에 해당하는지 체크 
    assert!(vec.par_iter().all(|n| (*n % 2) == 0));
}