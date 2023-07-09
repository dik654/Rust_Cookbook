// vector관련 라이브러리 가져오기
use vec;

fn main() {
    let mut vec = vec! [1, 5, 10, 2, 15];
    // 정렬 메서드 실행하여 오름차순 정렬    
    vec.sort();
    // 정렬이 제대로 되었는지 확인
    assert_eq!(vec, vec![1, 2, 5, 10, 15]);
}