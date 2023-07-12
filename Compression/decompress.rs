use std::fs::File;
use flate2::read::GzDecoder;
use tar::Archive;

fn main() -> Result<(), std::io::Error> {
    let path = "archive.tar.gz";

    // 지정한 파일 객체 리턴
    // "?"는 에러 발생시 리턴
    let tar_gz = File::open(path)?;
    // 압축해제, Result<GzDecoder<Read>, Error>을 반환한다
    let tar = GzDecoder::new(tar_gz);
    // tar 압축 파일을 다루는 구조체인 Archive 객체 생성
    let mut archive = Archive::new(tar);
    // 현재 디렉터리(.)에 압축 해제한 파일 저장
    // 성공시 (), 즉 아무 것도 리턴하지 않고 에러가 있다면 Error를 리턴 
    archive.unpack(".")?;

    Ok(())
}