pub fn encode(value: &[u8]) -> String {
    let mut encoded = String::with_capacity(value.len() * 4 / 3);
    let mut bytes = 0u32;
    let mut size = 0u8;

    for byte in value {
        bytes = (bytes << 8) + *byte as u32;
        size += 8;

        while size >= 6 {
            size -= 6;

            let mask = 0x3f << size;
            encoded.push(to_base64_char(((bytes & mask) >> size) as u8));
            bytes = bytes & !mask;
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
        bytes = (bytes << 6) + to_byte(char) as u32;
        size += 6;

        while size >= 8 {
            size -= 8;

            let mask = 0xff << size;
            decoded.push(((bytes & mask) >> size) as u8);
            bytes = bytes & !mask;
        }
    }
    decoded
}

fn to_base64_char(value: u8) -> char {
    if value < 26 {
        char::from(b'A' + value)
    } else if value < 52 {
        char::from(b'a' + (value - 26))
    } else if value < 62 {
        char::from(b'0' + (value - 52))
    } else if value < 63 {
        '+'
    } else if value < 64 {
        '/'
    } else {
        panic!("Input byte is not in base 64 ({})", value)
    }
}

fn to_byte(base64: &u8) -> u8 {
    if (b'A'..=b'Z').contains(&base64) {
        base64 - b'A'
    } else if (b'a'..=b'z').contains(&base64) {
        base64 - b'a' + 26
    } else if (b'0'..=b'9').contains(&base64) {
        base64 - b'0' + 52
    } else if base64 == &b'+' {
        62
    } else if base64 == &b'/' {
        63
    } else {
        panic!(
            "Character '{}' is not part of the base64 spec ([a-z][A-Z][0-9]+/=)",
            *base64 as char
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn should_preserve_original_value() {
        assert_eq!(decode(&encode(b"Hello!")), b"Hello!");
        assert_eq!(decode(&encode(b"Large msg...")), b"Large msg...");
        assert_eq!(decode(&encode(b"!@#$%^&*()_+")), b"!@#$%^&*()_+")
    }
}
