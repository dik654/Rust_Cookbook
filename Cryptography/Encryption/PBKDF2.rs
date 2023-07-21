use data_encoding::HEXUPPER;
use ring::error::unspecified;
use ring::rand::SecureRandom;
use ring::{digest, pbkdf2, rand};
use std::num::NonZeroU32;

fn main() -> Result<(), Unspecified> {
    // 키의 길이를 512로 지정
    const CREDENTIAL_LEN: usize = digest::SHA512_OUTPUT_LEN;
    // pbkdf2의 반복횟수를 100000번으로 설정
    let n_iter = NonZeroU32::new(100_000).unwrap();
    // 해싱에서 사용할 salt값을 만들기 위해 512길이의 0으로 채워진 배열 생성
    let mut salt = [0u8; CREDENTIAL_LEN];
    // 랜덤값으로 slat배열 채우기
    rng.fill(&mut salt)?;
    // 해싱할 값
    let password = "Guess Me If You Can!";
    // 해시값을 담을 배열
    let mut pbkdf2_hash = [0u8; CREDENTIAL_LEN];
    // 해싱 시작
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA512,
        n_iter,
        &salt,
        password.as_bytes(),
        &mut pbkdf2_hash,
    );
    println!("Salt: {}", HEXUPPER.encode(&salt));
    println!("PBKDF2 hash: {}", HEXUPPER.encode(&pbkdf2_hash));

    // 해싱된 비밀번호 검증
    let should_succeed = pbkdf2::verify(
        pbkdf2::PBKDF2_HMAC_SHA512,
        n_iter,
        &salt,
        password.as_bytes(),
        &pbkdf2_hash,
    );
    // 틀린 비밀번호로 검증
    let wrong_password = "Definitely not the correct password";
    let should_fail = pbkdf2::verify(
        pbkdf2::PBKDF2_HMAC_SHA512,
        n_iter,
        &salt,
        // salt등 다른 값은 동일하고 비밀번호만 다르게 넣어 확인한다
        wrong_password.as_bytes(),
        &pbkdf2_hash,
    );
    // 실제로 검증이 제대로 이루어지는지 확인
    assert!(should_succeed.is_ok());
    assert!(!should_fail.is_ok());

    Ok(())
}