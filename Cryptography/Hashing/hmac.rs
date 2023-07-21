use ring::{hmac, rand};
use ring::rand::SecureRandom;
use ring::error::Unspecified;

fn main() -> Result<(), Unspecified> {
    // 길이 48, uint8 타입의 0으로 가득 찬 배열 생성(비밀키용 초기값)
    let mut key_value = [0u8; 48];
    // 난수 생성기 생성
    let rng = rand::SystemRandom::new();
    
    // 위에 생성한 key_value배열에 난수 생성해서 채우기
    rng.fill(&mut key_value)?;
    // key_Value배열을 사용하여 비밀키 생성
    let key = hmac::Key::new(hmac::HMAC_SHA256, &key_value);

    let message = "Legitimage and important message.";
    // HMAC 서명
    let signature = hmac::sign(&key, message.as_bytes());
    // 비밀키와 메세지를 이용해 서명이 올바른지 확인
    hmac::verify(&key, message.as_bytes(), signature.as_ref())?;

    Ok(())
}