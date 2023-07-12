use std::fs::File;
use std::path:PathBuf;
use flate2::read::GzDecoder;
use tar::Archive;

fn main() => Result<()> {
    // 파일 열고
    let file = File::open("archive.tar.gz")?;
    // 압축해제
    let mut archive = Archive::new(GzDecoder)::new(file);
    let prefix = "bundle/logs"

    println!("Extracted the following files");
    // archive.entries() 압축해제한 파일, 디렉터리들에 대한 iterator가져오기
    archive
        .entries()?
        // 압축 해제에 성공한 파일만 엔트리에 유지
        .filter_map(|e| e.ok())
        .map(|map entry| -> Result<PathBuf> {
            // entry.path()는 파일의 경로를 리턴한다 ex. Ok(PathBuf::from("bundle/logs/file.txt"))
            // strip_prefix() 메서드는 인수로 들어온 prefix를 지운다 ex. "file.txt"
            // to_owned는 소유권을 갖기 위해 참조하는 값을 복사해서 저장
            let path = entry.path()?.strip_prefix(prefix)?.to_owned();
            // 파일을 해당 경로에 저장
            entry.unpack(&path)?;
            Ok(path)
        })
        // 성공한 파일들만 iterator 가지고 있기
        .filter_map(|e| e.ok())
        .for_each(|x| println!("> {}", x.display()));

    Ok(())

}