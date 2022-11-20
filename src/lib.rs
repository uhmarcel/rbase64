use crate::common::*;

mod common;
mod decode;
mod encode;

pub fn encode(input: &[u8]) -> String {
    let mut buffer = vec![0; ((input.len() / 3) + 1) * 4];
    let total_chunks = input.len() / (ENC_CHUNK_SIZE * 3);

    encode::encode_u64_chunks(input, &mut buffer);

    let bytes_rem = encode::encode_u64_remainder(
        &input[ENC_CHUNK_SIZE * total_chunks * 3..],
        &mut buffer[ENC_CHUNK_SIZE * total_chunks * 4..],
    );

    buffer.truncate(ENC_CHUNK_SIZE * total_chunks * 4 + bytes_rem);

    // Buffer is built from UTF8 chars only. Safe to use and improves performance.
    unsafe { String::from_utf8_unchecked(buffer) }
}

pub fn decode(encoded: &str) -> Vec<u8> {
    let input = encoded.as_bytes();
    let mut buffer = vec![0; ((encoded.len() + 3) / 4) * 3];

    let total_chunks = input
        .len()
        .saturating_sub(DEC_CHUNK_SIZE)
        .saturating_div(DEC_CHUNK_SIZE * 4);

    decode::decode_u64_chunks(input, &mut buffer, total_chunks);

    let bytes_rem = decode::decode_u64_remainder(
        &input[DEC_CHUNK_SIZE * total_chunks * 4..],
        &mut buffer[DEC_CHUNK_SIZE * total_chunks * 3..],
    );

    buffer.truncate(3 * DEC_CHUNK_SIZE * total_chunks + bytes_rem);
    buffer
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::SmallRng;
    use rand::{Rng, SeedableRng};

    #[test]
    fn should_encode_following_base64_spec() {
        assert_eq!(encode(b"Hello!"), "SGVsbG8h");
        assert_eq!(encode(b"0123456789"), "MDEyMzQ1Njc4OQ==");
        assert_eq!(
            encode(b"https://foo.bar/q?a=2&b=3#fr"),
            "aHR0cHM6Ly9mb28uYmFyL3E/YT0yJmI9MyNmcg=="
        );
        assert_eq!(encode(b"  "), "ICA=");
        assert_eq!(encode(b""), "");
        assert_eq!(encode(&0u32.to_ne_bytes()), "AAAAAA==");
    }

    #[test]
    fn should_decode_following_base64_spec() {
        assert_eq!(decode("SGVsbG8h"), b"Hello!");
        assert_eq!(decode("MDEyMzQ1Njc4OQ=="), b"0123456789");
        assert_eq!(
            decode("aHR0cHM6Ly9mb28uYmFyL3E/YT0yJmI9MyNmcg=="),
            b"https://foo.bar/q?a=2&b=3#fr"
        );
        assert_eq!(decode("ICA="), b"  ");
        assert_eq!(decode(""), b"");
        assert_eq!(decode("AAAAAA=="), 0u32.to_ne_bytes())
    }

    #[test]
    fn should_preserve_original_input() {
        for size in 0..512 {
            let bytes = random_bytes(size);
            assert_eq!(decode(&encode(&bytes)), bytes);
        }
        let large = random_bytes(3 * 1024 * 1024);
        assert_eq!(decode(&encode(&large)), large);
    }

    #[test]
    #[should_panic(expected = "Unable to decode non-base64 character '^'")]
    fn should_panic_when_decode_non_base64_input() {
        decode("AAA^AAA==");
    }

    fn random_bytes(size: usize) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(size);
        let mut r = SmallRng::from_entropy();
        while bytes.len() < size {
            bytes.push(r.gen::<u8>());
        }
        bytes
    }
}
