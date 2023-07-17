use std::path::Path;
use std::fs::create_dir_all;

use glob::{glob_with, MatchOptions};
use image::{FilterType, ImageError};
use rayon::prelude::*;

fn main() -> Result<()> {
    let options: MatchOptions = Default::default();
    // options의 디렉터리에서 jpg파일을 검색
    let files: Vec<_> = glob_with("*.jpg", options)?
        // 값이 있는 경우만 모아서
        .filter_map(|x| x.ok())
        // vector로 연결
        .collect();
    
    if files.len() == 0 {
        error_chain::bail!("No .jpg files found in current directory");
    }

    let thumb_dir = "thumbnails";
    create_dir_all(thumb_dir)?;

    let image_failures: Vec<_> = files
        .par_iter()
        .map(|path| {
            make_thumbnail(path, thumb_dir, 300)
                .map_err(|e| e.chain_err(|| path.display().to_string()))
        })
        .filter_map(|x| x.err())
        .collect();
    
    image_failures.
}