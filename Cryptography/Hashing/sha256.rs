// 16진수로 표기
use data_encoding::HEXUPPER;
use ring::digest::{Context, Digest, SHA256};
use std::fs::File;
use std::io::{BufReader, Read, Write};

// <R: Read>는 Read 트레이트 구현했다는 뜻 (trait bound)
// Read 트레이트를 가진 reader를 인수로 받는다
fn sha256_digest<R: Read>(mut reader: R) -> Result<Digest> {
    // 암호화에 사용되는 Context를 이용하여 SHA256으로 digest(해시값)를 계산할 준비를 한다
    let mut context = Context::new(&SHA256);
    // 1KB 버퍼    
    let mut buffer = [0; 1024];

    loop {
        // reader에서 bytes를 읽어서 buffer에 저장
        let count = reader.read(&mut buffer)?;
        // 모두 다 읽었다면 종료
        if count == 0 {
            break;
        }
        // context에 데이터를 차례대로 담아 해시를 계산할 준비
        context.update(&buffer[..count]);
    }
    // SHA256해시를 계산하고 반환
    Ok(context.finish())
}

fn main() -> Result<()> {
    let path = "file.txt";

    let mut output = File::create(path)?;
    write!(output, "We will generate a digest of this text")?;

    let input = File::open(path)?;
    // Reader로 버퍼를 이용해 파일 읽기
    let reader = BufReader::new(input);
    // 해시화 작업
    let digest = sha256_digest(reader)?;
    // 16진수로 표기
    println!("SHA-256 digest is {}", HEXUPPER.encode(digest.as_ref()));
    
    Ok(())
}