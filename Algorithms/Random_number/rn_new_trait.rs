use rand::Rng;
use rand::distributions::{Distribution, Standard};

//컴파일러에게 Debug trait를 자동으로 구현하도록 지시
//{:?}, {:#?} 사용가능
#[derive(Debug)]
struct Point{
    x: i32,
    y: i32,
}
//Point타입의 sample trait를 구현
impl Distribution<Point> for Standard {
    //?Sized는 컴파일 시점에 그 크기를 알아야한다는 제약을 해제
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point {
        let (rand_x, rand_y) = rng.gen();
        Point {
            x: rand_x,
            y: rand_y,
        }
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    //튜플로 한번에 동시에 여러 타입의 값의 랜덤 값을 얻을 수 있음
    let rand_tuple = rng.gen::<(i32, bool, f64)>();
    //gen()메서드는 rand::distributions::Standard trait의 sample 메서드를 사용하므로
    //sample만 구현해도 gen() 메서드를 사용할 수 있다
    let rand_point: Point = rng.gen();
    //Random tuple: (1697525552, false, 0.2700106437406633)
    println!("Random tuple: {:?}", rand_tuple);
    println!("Random point: {:?}", rand_point);
}