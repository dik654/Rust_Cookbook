use std::{thread, time};
use crossbeam_channel::unbounded;

fn main() {
    // 용량제한없는 MPSC 채널 생성 
    let (snd, rcv) = unbounded();
    let n_msgs = 5;
    crossbeam::scope(|s| {
        s.spawn(|_| {
            for i in 0..n_msgs {
                // 데이터를 전송하고 스레드 0.1초간 대기
                snd.send(i).unwrap();
                thread::sleep(time::Duration::from_millis(100));
            }
        });
    }).unwrap();
    for _ in 0..n_msgs {
        // 채널에서 데이터 읽기
        let msg = rcv.recv().unwrap();
        println!("Received {}", msg);
    }
}