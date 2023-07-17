// rust 구버전에서 라이브러리 dependency를 추가하는 방법
extern crate crossbeam;
extern crate crossbeam_channel;

use std::thread;
use std::time::Duration;
use crossbeam_channel::bounded;

fn main() {
    // 1개의 요소를 담을 수 있는 채널의 sender, receiver 생성
    let (snd1, rcv1) = bounded(1);
    let (snd2, rcv2) = bounded(1);
    let n_msgs = 4;
    let n_workers = 2;

    crossbeam::scope(|s| {
        s.spawn(|_| {
            for i in 0..n_msgs {
                // 1부터 4까지 차례대로 sender에 실어서 보내기
                // send시 이미 채널이 가득 찼다면 채널이 빌 때까지 대기한다  
                snd1.send(i).expect("Failed to send data");
                println!("Source sent {}", i);
            }
            // drop은 인수 값의 수명을 강제종료
            // 즉 sender를 닫는 역할
            drop(snd1);
        });

        for _ in 0..n_workers {
            let (sendr, recvr) = (snd2.clone(), rcv1.clone());
            // move로 소유권이 이동되어서 sendr이 snd2채널을 사용할 수 있게되었다.
            s.spawn(move |_| {
                // 0.5초 대기
                thread::sleep(Duration::from_millis(500));
                // receiver에서 iter()를 불러오면 값을 꺼낸다
                // 만약 1번 채널에서 채널에 값이 없다면 값이 들어올 때까지 대기한다
                for msg in recvr.iter() {
                    println!("Worker {:?} received {}.", thread::current().id(), msg);
                }
                
                sendr.send(msg * 2).expect("Failed to send data");
            });
        }
        // snd2 닫기
        drop(snd2);

        // msg * 2으로 전송한 데이터 읽기
        for msg in rcv2.iter() {
            println!("Sink received {}", msg);
        }
    }).unwrap();
}