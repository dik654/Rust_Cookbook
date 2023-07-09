use vec;

fn main() {
    let mut vec = vec![1.1, 1.15, 5.5, 1.123, 2.0];
    // 인수의 조건 하에서 차례대로 정렬 실행
    // partial_cmp는 a < b라면 a, b
    // a > b 라면 b, a로 정렬
    vec.sort_by(|a, b| a.partial_cmp(b).unwrap());
    
    assert_eq!(vec, vec![1.1, 1.123, 1.15, 2.0, 5.5]);
}