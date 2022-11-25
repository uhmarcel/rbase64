use crate::common::*;
use crate::decode::DecodeError;
use mimalloc::MiMalloc;

mod common;
mod decode;
mod encode;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

pub fn encode(input: &[u8]) -> String {
    let mut buffer = vec![0; ((input.len() / 3) + 1) * 4];
    let total_chunks = input.len() / (ENC_CHUNK_SIZE * 3);

    encode::encode_u128_chunks(input, &mut buffer);

    let bytes_rem = encode::encode_u128_remainder(
        &input[ENC_CHUNK_SIZE * total_chunks * 3..],
        &mut buffer[ENC_CHUNK_SIZE * total_chunks * 4..],
    );

    buffer.truncate(ENC_CHUNK_SIZE * total_chunks * 4 + bytes_rem);

    // Buffer built from UTF8 chars only. Safe to use and improves performance.
    unsafe { String::from_utf8_unchecked(buffer) }
}

pub fn decode(encoded: &str) -> Result<Vec<u8>, DecodeError> {
    let input = encoded.as_bytes();
    let mut buffer = vec![0; ((input.len() + 3) / 4) * 3];

    let total_chunks = input.len().saturating_sub(2) / (DEC_CHUNK_SIZE * 4);
    let in_limit = total_chunks * DEC_CHUNK_SIZE * 4;
    let out_limit = total_chunks * DEC_CHUNK_SIZE * 3;

    decode::decode_u64_chunks(&input[..in_limit], &mut buffer)?;

    let bytes_rem = decode::decode_u64_remainder(&input[in_limit..], &mut buffer[out_limit..])?;

    buffer.truncate(out_limit + bytes_rem);
    Ok(buffer)
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
        assert_eq!(decode("SGVsbG8h").unwrap(), b"Hello!");
        assert_eq!(decode("MDEyMzQ1Njc4OQ==").unwrap(), b"0123456789");
        assert_eq!(
            decode("aHR0cHM6Ly9mb28uYmFyL3E/YT0yJmI9MyNmcg==").unwrap(),
            b"https://foo.bar/q?a=2&b=3#fr"
        );
        assert_eq!(decode("ICA=").unwrap(), b"  ");
        assert_eq!(decode("").unwrap(), b"");
        assert_eq!(decode("AAAAAA==").unwrap(), 0u32.to_ne_bytes())
    }

    #[test]
    fn should_preserve_original_input() {
        for size in 0..512 {
            let bytes = random_bytes(size);
            assert_eq!(decode(&encode(&bytes)).unwrap(), bytes);
        }
        let large = random_bytes(3 * 1024 * 1024);
        assert_eq!(decode(&encode(&large)).unwrap(), large);
    }

    #[test]
    fn should_error_when_decode_non_base64_input() {
        assert_eq!(
            decode("AAA^AAA==").unwrap_err(),
            DecodeError::InvalidByte(b'^')
        );
        assert_eq!(decode("!").unwrap_err(), DecodeError::InvalidByte(b'!'));
        assert_eq!(
            decode("\nNjc4OQ==").unwrap_err(),
            DecodeError::InvalidByte(b'\n')
        );

        assert_eq!(
            "Invalid non-base64 byte '#'",
            format!("{}", DecodeError::InvalidByte(b'#'))
        );
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
