use base32;
use sha2::{Digest, Sha256};

/**
 * 주어진 문자열로부터 sha256 다이제스트 생성. 256bit, 그러니까 32바이트 벡터 반환됨.
 */
pub fn digest(url: &str) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(url.as_bytes());
    hasher.finalize().to_vec()
}

/**
 * 주어진 바이트 벡터의 앞부분 일부를 취해서 Crockford's base32로 인코딩
 */
pub fn truncated_base32(digest: &Vec<u8>, truncate_len: usize) -> String {
    let truncated: &[u8] = &digest.as_slice()[..truncate_len];
    base32::encode(base32::Alphabet::Crockford, truncated)
}

/**
 * Crockford의 Base32 인코딩에서는 사람이 보기에 혼동하기 쉬운 글자들을 하나로 인식한다.
 * 예를들어, 인코딩된 문자열에서 1을 i나 l등으로 인식해서 입력했다고 하더라도,
 * 디코딩시에 정상적으로 1로 처리해주는 것. 사람이 잘못 읽어서 기재한 인코딩 문자열을 디코딩했다가
 * 다시 인코딩하면, 표준적인 base32문자열을 만들 수 있다.
 */
pub fn base32_normalize(encoded: &str) -> Option<String> {
    let decoded = base32::decode(base32::Alphabet::Crockford, encoded)?;
    Some(base32::encode(base32::Alphabet::Crockford, &decoded))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digest() {
        let a_long_url = "https://hatemogi.com/long-url-to-be-shorten";
        let b_long_url = "https://hatemogi.com/other-url-to-be-shorten";
        let a_digest = digest(a_long_url);
        assert_eq!(32, a_digest.len());
        assert_eq!(a_digest, digest(a_long_url));
        assert_ne!(a_digest, digest(b_long_url));
    }

    #[test]
    fn test_base32() {
        let data = digest("https://long-url");
        let base32 = truncated_base32(&data, 2);
        assert_eq!(base32.len(), 4);
        assert_eq!("CGF0", base32);
    }

    #[test]
    fn test_base32_normalize() {
        assert_eq!("CGF0", base32_normalize("CGFO").unwrap());
        assert_eq!("1111A", base32_normalize("1ilIA").unwrap());
    }
}
