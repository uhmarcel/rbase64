const ENCODE_MAP: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
const DECODE_MAP: &[u8; 256] = &construct_decode_map();

const INVALID_BYTE: u8 = 0x40;

pub fn encode(value: &[u8]) -> String {
    let mut encoded = String::with_capacity((value.len() * 4 / 3) + 8);
    let mut bytes = 0u32;
    let mut size = 0u8;

    for byte in value {
        bytes = (bytes << 8) + *byte as u32;
        size += 8;

        while size >= 6 {
            size -= 6;

            let mask = 0x3f << size;
            encoded.push(to_base64_char(((bytes & mask) >> size) as u8));
            bytes &= !mask;
        }
    }

    if size > 0 {
        bytes <<= 6 - size;
        encoded.push(to_base64_char(bytes as u8));
    }

    while encoded.len() % 4 > 0 {
        encoded.push('=');
    }
    encoded
}

pub fn decode(encoded: &str) -> Vec<u8> {
    let mut decoded: Vec<u8> = Vec::with_capacity(encoded.len() * 3 / 4);
    let mut bytes = 0u32;
    let mut size = 0u8;

    for char in encoded.as_bytes() {
        if char == &b'=' {
            break;
        }
        bytes = (bytes << 6) + to_byte(*char) as u32;
        size += 6;

        while size >= 8 {
            size -= 8;

            let mask = 0xff << size;
            decoded.push(((bytes & mask) >> size) as u8);
            bytes &= !mask;
        }
    }
    decoded
}

fn to_base64_char(value: u8) -> char {
    ENCODE_MAP[value as usize] as char
}

fn to_byte(encoded_byte: u8) -> u8 {
    let decoded = DECODE_MAP[encoded_byte as usize];

    if decoded == INVALID_BYTE {
        panic!(
            "Unable to decode non-base64 character '{}'",
            encoded_byte as char
        )
    }
    decoded
}

const fn construct_decode_map() -> [u8; 256] {
    let mut map = [INVALID_BYTE; 256];
    let mut index = 0;

    while index < 64 {
        map[ENCODE_MAP[index] as usize] = index as u8;
        index += 1;
    }
    map
}

#[cfg(test)]
mod tests {
    use super::*;
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
    #[should_panic(expected = "Unable to decode non-base64 character '^'")]
    fn should_panic_when_decode_non_base64_input() {
        decode("AAA^AAA==");
    }

    #[test]
    fn should_preserve_original_input() {
        for size in 0..512 {
            let bytes = random_bytes(size);
            assert_eq!(decode(&encode(&bytes)), bytes);
        }
    }

    #[test]
    fn should_construct_matching_encode_decode_tables() {
        for byte in 0..64 {
            assert_eq!(
                construct_decode_map()[ENCODE_MAP[byte] as usize],
                byte as u8
            );
        }
    }

    fn random_bytes(size: usize) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(size);
        let mut r = rand::rngs::SmallRng::from_entropy();
        while bytes.len() < size {
            bytes.push(r.gen::<u8>());
        }
        bytes
    }
}
