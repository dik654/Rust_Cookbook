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
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())

}