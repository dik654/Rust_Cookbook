use vec;

// trait 기본구현 생성
// Debug: 디버깅 형식 관련 trait
// Eq, PartialEq: "==", "!=" 관련 tarit으로, PartialEq는 부동소수점 등에서 사용
// Ord, PartialOrd: 크기 비교 관련 trait, PartialEq는 부동소수점 등에서 사용
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Person {
    name: String,
    age: u32
}

impl Person {
    pub fn new(name: String, age: u32) -> Self {
        Person {
            name,
            age
        }
    }
}

fn main() {
    let mut people = vec![
        Person::new("Zoe".to_string(), 25),
        Person::new("Al".to_string(), 60),
        Person::new("John".to_string(), 1),
    ];
    // Ord trait에 의해 첫 번째 필드인 name끼리 비교 후 서로 같다면,
    // age 필드를 비교하여 정렬한다 
    people.sort();

    assert_eq!(
        people,
        vec![
            Person::new("Al".to_string, 60),
            Person::new("John".to_string, 1),
            Person::new("Zoe".to_string, 25),
        ];
    )
    // sort_by는 정렬 방식을 인수로 선언
    // 아래는 나이로만 비교하는 경우를 테스트한다
    people.sort_by(|a, b| b.age.cmp(&a.age));

    assert_eq!(
        people,
        vec![
            Person::new("Al".to_string, 60),
            Person::new("Zoe".to_string, 25),
            Person::new("John".to_string, 1),
        ];
    )
}