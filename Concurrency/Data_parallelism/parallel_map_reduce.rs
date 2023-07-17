use rayon::prelude::*;

struct Person {
    age: u32,
}

fn main() {
    let v: Vec<Person> = vec! {
        Person { age: 23 },
        Person { age: 31 },
        Person { age: 19 },
        Person { age: 42 },
    }

    let num_over_30 = v.par_iter().filter(|&x| x.age > 30).count() as f32;
    // map과 reduce를 이용해 30살이 넘는 나이를 모두 합한다
    let sum_over_30 = v.par_iter()
        .map(|x| x.age)
        .filter(|&x| x > 30)
        .reduce(|| 0, |x, y| x + y);
    // reduce대신 sum을 사용한 경우
    let alt_sum_30: u32 = v.par_iter()
        .map(|x| x.age)
        .filter(|&x| x > 30)
        .sum();

    let avg_over_30 = sum_over_30 as f32 / num_over_30;
    let alt_avg_over_30 = alt_sum_30 as f32 / num_over_30;
    // std::f32::EPSILON는 f32에서 매우 작은 값을 나타낸다
    // 즉 두 값이 거의 동일해야 해당 assert를 통과 가능
    assert!((avg_over_30 - alt_avg_over_30).abs() < std::f32::EPSILON);
    println!("The average age of people older than 30 is {}", avg_over_30);
}