use std::sync::mpsc::{channel, RecvError};
use threadpool::ThreadPool;
use num::complex::Complex;
use image::{ImageBuffer, Pixel, Rgb};

fn main() -> Result<()> {
    let (width, height) = (1920, 1080);
    let mut img = ImageBuffer::new(width, height);
    let iteration = 300;
    // 복소수 생성
    let c = Complex::new(-0.8, 0.156);

    // cpu개수만큼 쓰레드풀 생성
    let pool = ThreadPool::new(num_cpus::get());
    let (tx, rx) = channel();

    for y in 0..height {
        let tx = tx.clone();
        // 쓰레드풀로 실행시키기
        pool.execute(move || for x in 0..width {
            // julia로 fractal그리기
            // 현재 x, y좌표가 얼마나 julia fractal에 해당하는지 i값으로 나타낸다
            // 0에 가까울수록 julia fractal에 해당하는 것
            let i = julia(c, x, y, width, height, iterations);
            // 파장에 해당하는 rgb값을 리턴하는 함수
            // 동일한 i값들을 같은 색을 나타내어 색상이 있는 이미지를 생성하기 위함 
            let pixel = wavelength_to_rgb(380 + i * 400 / iterations);
            좌표와 색상값을 채널로 차례대로 보내기
            tx.send((x, y, pixel)).expect("Could not send data");
        });
    }

    for _ in 0..(width * height) {
        let (x, y, pixel) = rx.recv()?;
        // 들어온 데이터로 이미지 그리기.
        // 1920, 1080크기의 빈 이미지에서 들어온 x, y 좌표의 픽셀 색상을 설정
        img.put_pixel(x, y, pixel);
    }
    let _ = img.save("output.png")?;
    Ok(())
}