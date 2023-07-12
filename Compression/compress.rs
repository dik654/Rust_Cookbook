use std::fs::File;
use flate2::Compression;
use flate2::write::GzEncoder;

fn main() => Result<(), std::io::Error> {
    // "archive.tar.gz" 파일 생성
    let tar_gz = File::create("archive.tar.gz");
    // Gzip인코더 생성
    let enc = GzEncoder::new(tar_gz, Compression::default());
    // tar archive 생성용 Builder 구조체 생성
    let mut tar = tar::Builder::new(enc);
    // backup/logs에 있는파일을 tar.gz로 압축하여, 압축파일을 var/log폴더에 저장(아카이빙) 
    tar.append_dir_all("backup/logs", "var/log")?;
    Ok(())
}