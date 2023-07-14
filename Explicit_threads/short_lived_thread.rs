fn main() {
    let arr = &[1, 25, -4, 10];
    // 배열에서 가장 큰 값을 찾는 로직
    let max = find_max(arr);
    assert_eq!(max, Some(25));
}

fn find_max(arr: &[i32]) -> Option<i32> {
    const THRESHOLD: usize = 2;
    // 재귀적인 실행을 위한 준비
    if arr.len() <= THRESHOLD {
        return arr.iter().cloned().max();
    }

    // 절반 위치 인덱스를 구해서
    let mid = arr.len() / 2;
    // 해당 인덱스에서 반반으로 배열을 나눈다   
    let (left, right) = arr.split_at(mid);
    // golang의 channel과 유사하게
    crossbeam::scope(|s| {
        // 반으로 나눈 배열에서 재귀적으로 실행
        let thread_l = s.spawn(|_| find_max(left));
        let thread_r = s.spawn(|_| find_max(right));
        // thread_l, r의 실행이 완료될 때까지 대기
        let max_l = thread_l.join().unwrap()?;
        let max_r = thread_r.join().unwrap()?;
        // l, r의 최대값을 비교해 더 큰 값을 반환(Option<T>) 
        Some(max_l.max(max_r))
    // Scope<'a> 벗겨내기 -> Option<T>
    // 'a는 crossbeam::scope내의 컨텍스트의 수명.
    // 참조 유효성 보장하기위해서 이용
    }).unwrap();

    // Option타입은 값이 없는경우(null)를 위해 안정성을 위해 사용한다
    // 값이 없을 때 unwrap을 하면 panic
}