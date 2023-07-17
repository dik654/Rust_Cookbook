use walkdir::WalkDir;
use std::fs::File;
use std::io::{BufReader, Read, Error};
use std::path::Path;
use threadpool::ThreadPool;
use std::sync::mpsc::channel;
use ring::digest::{Context, Digest, SHA256};

fn compute_digest<P: AsRef<Path>>(filepath: P) -> Result<(Digest, P), Error> {
    // file reader객체 생성
    let mut buf_reader = BufReader::new(File::open(&filepath)?);
    // 어떤 해시 알고리즘으로 msg digest를 읽을지
    let mut context = Context::new(&SHA256);
    // 일반적으로 파일시스템이 4kb, 8kb의 크기를 가져 이에 맞춘 버퍼크기
    let mut buffer = [0; 1024];

    loop {
        // 읽기 시작
        // count는 읽은 바이트의 수
        let count = buf_reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        // update메서드가 중복 데이터는 알아서 처리한다 
        context.update(&buffer[..count])
    }
    Ok((context.finish(), filepath))
}

fn main() -> Result<(), Error> {
    let pool = ThreadPool::new(num_cpus::get());

    let (tx, rx) = channel();
    // WalkDir로 해당 디렉터리에서 파일과 하위 디렉터리 정보 제공
    for entry in WalkDir::new("/home/user/Downloads")
        // symbolic link 따라가기
        .follow_links(true)
        // iterator 설정
        .into_iter()
        // 값이 없는 파일들 iterator에서 제거
        .filter_map(|e| e.ok())
        // 디렉터리가 아닌 ISO파일만 iterator에 유지
        .filter(|e| !e.path().is_dir() && is_iso(e.path()) {
            // 소유권을 포함하여 ISO파일 경로를 가져오기
            let path = entry.path().to_owned();
            // 채널에 쓰기 위해 채널 복사
            // 소유권은 하나밖에 가질 수 없기에 여러 스레드가 동시에 접근하려면 복사를 해야함
            let tx = tx.clone();
            // 쓰레드풀에 작업 제출
            pool.execute(move || {
                // digest 계산 후 리턴
                let digest = compute_digest(path);
                // 채널에 전송
                tx.send(digest).expect("Could not send data!");
            });
        }
    // tx 닫기 
    drop(tx);
    // 작업 완료된 ISO파일들에 대해 iterate
    for t in rx.iter() {
        // digest와 파일 경로 받기
        let (sha, path) = t?;
        println!("{:?} {:?}", sha, path);
    }
    Ok(())
}