use rayon::prelude::*;
use std::cmp::min;
use std::iter::zip;

const ENCODE_MAP: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
const DECODE_MAP: &[u8; 256] = &construct_decode_map();

const SIX_BIT_MASK: u128 = 0x3f;
const BYTE_MASK: u64 = 0xff;
const INVALID_BYTE: u8 = 0x40;

const ENCODE_CHUNK_SIZE: usize = 4;
const DECODE_CHUNK_SIZE: usize = 2;
const PARALLEL_THRESHOLD_CHUNKS: usize = 1024;

const ENCODE_U128_OFFSET: usize = (ENCODE_CHUNK_SIZE * 3 - 1) * 8;
const DECODE_U64_OFFSET: usize = (DECODE_CHUNK_SIZE * 4 - 1) * 6;

pub fn encode(bytes: &[u8]) -> String {
    let mut buffer = vec![0; ((bytes.len() / 3) + 1) * 4];
    let total_chunks = bytes.len() / (ENCODE_CHUNK_SIZE * 3);

    if total_chunks < PARALLEL_THRESHOLD_CHUNKS {
        encode_u128_chunks(bytes, &mut buffer);
    } else {
        encode_u128_chunks_parallel(bytes, &mut buffer);
    }

    let in_index = ENCODE_CHUNK_SIZE * total_chunks * 3;
    let mut out_index = ENCODE_CHUNK_SIZE * total_chunks * 4;
    let in_u128 = read_u128_partial(&bytes[in_index..]);
    let mut in_bits = 8 * (bytes.len() - in_index);

    while in_bits >= 6 {
        in_bits -= 6;
        buffer[out_index] = encode_byte(((in_u128 >> in_bits) & SIX_BIT_MASK) as u8);
        out_index += 1;
    }

    if in_bits > 0 {
        buffer[out_index] = encode_byte(((in_u128 << (6 - in_bits)) & SIX_BIT_MASK) as u8);
        out_index += 1;
    }

    while out_index % 4 > 0 {
        buffer[out_index] = b'=';
        out_index += 1;
    }

    buffer.truncate(out_index);

    // Buffer is built from UTF8 chars only. Safe to use and improves performance.
    unsafe { String::from_utf8_unchecked(buffer) }
}

pub fn decode(encoded: &str) -> Vec<u8> {
    let input = encoded.as_bytes();
    let mut buffer = vec![0; ((encoded.len() + 3) / 4) * 3];

    let total_chunks = input
        .len()
        .saturating_sub(DECODE_CHUNK_SIZE)
        .saturating_div(DECODE_CHUNK_SIZE * 4);

    if total_chunks < PARALLEL_THRESHOLD_CHUNKS {
        decode_u64_chunks(input, &mut buffer, total_chunks);
    } else {
        decode_u64_chunks_parallel(input, &mut buffer, total_chunks);
    }

    let mut in_u64 = 0u64;
    let mut in_bits = 0;

    for in_byte in input[DECODE_CHUNK_SIZE * total_chunks * 4..].iter() {
        if *in_byte == b'=' {
            break;
        }
        in_u64 = (in_u64 << 6) | decode_byte(*in_byte) as u64;
        in_bits += 6;
    }

    let bytes_remaining = in_bits / 8;
    let start_pos = DECODE_CHUNK_SIZE * total_chunks * 3;

    for (i, out_byte) in buffer[start_pos..start_pos + bytes_remaining]
        .iter_mut()
        .enumerate()
    {
        *out_byte = ((in_u64 >> (in_bits - 8 * (i + 1))) & BYTE_MASK) as u8;
    }

    buffer.truncate(3 * DECODE_CHUNK_SIZE * total_chunks + bytes_remaining);
    buffer
}

#[inline(always)]
fn encode_u128_chunks(input: &[u8], buffer: &mut [u8]) {
    let in_chunks = input.chunks_exact(ENCODE_CHUNK_SIZE * 3);
    let out_chunks = buffer.chunks_exact_mut(ENCODE_CHUNK_SIZE * 4);

    zip(in_chunks, out_chunks).for_each(|(in_chunk, out_chunk)| {
        encode_u128(in_chunk, out_chunk);
    });
}

#[inline(always)]
fn encode_u128_chunks_parallel(input: &[u8], buffer: &mut [u8]) {
    let in_chunks = input.par_chunks_exact(ENCODE_CHUNK_SIZE * 3);
    let out_chunks = buffer.par_chunks_exact_mut(ENCODE_CHUNK_SIZE * 4);

    in_chunks.zip(out_chunks).for_each(|(in_chunk, out_chunk)| {
        encode_u128(in_chunk, out_chunk);
    });
}

#[inline(always)]
fn encode_u128(input: &[u8], buffer: &mut [u8]) {
    let in_u128 = read_u128_partial(input);

    buffer.iter_mut().enumerate().for_each(|(i, out_b)| {
        *out_b = encode_byte(((in_u128 >> (2 + ENCODE_U128_OFFSET - 6 * i)) & SIX_BIT_MASK) as u8);
    });
}

#[inline(always)]
fn decode_u64_chunks(input: &[u8], buffer: &mut [u8], total_chunks: usize) {
    let in_chunks = input.chunks_exact(DECODE_CHUNK_SIZE * 4);
    let out_chunks = buffer.chunks_exact_mut(DECODE_CHUNK_SIZE * 3);

    for (in_chunk, out_chunk) in zip(in_chunks, out_chunks).take(total_chunks) {
        decode_u64(in_chunk, out_chunk);
    }
}

#[inline(always)]
fn decode_u64_chunks_parallel(input: &[u8], buffer: &mut [u8], total_chunks: usize) {
    let in_chunks = input.par_chunks_exact(DECODE_CHUNK_SIZE * 4);
    let out_chunks = buffer.par_chunks_exact_mut(DECODE_CHUNK_SIZE * 3);

    in_chunks
        .zip(out_chunks)
        .take(total_chunks)
        .for_each(|(in_chunk, out_chunk)| {
            decode_u64(in_chunk, out_chunk);
        });
}

#[inline(always)]
fn decode_u64(input: &[u8], buffer: &mut [u8]) {
    let mut in_u64 = 0u64;

    input.iter().enumerate().for_each(|(i, in_byte)| {
        in_u64 |= (decode_byte(*in_byte) as u64) << (DECODE_U64_OFFSET - 6 * i + 2) as u64;
    });

    buffer.iter_mut().enumerate().for_each(|(i, out_byte)| {
        *out_byte = ((in_u64 >> (DECODE_U64_OFFSET - (8 * i))) & BYTE_MASK) as u8;
    });
}

#[inline(always)]
fn encode_byte(byte: u8) -> u8 {
    ENCODE_MAP[byte as usize]
}

#[inline(always)]
fn decode_byte(byte: u8) -> u8 {
    let decoded = DECODE_MAP[byte as usize];

    if decoded == INVALID_BYTE {
        panic!("Unable to decode non-base64 character '{}'", byte as char)
    }
    decoded
}

#[inline(always)]
fn read_u128_partial(bytes: &[u8]) -> u128 {
    let size = min(bytes.len(), 16);
    let mut buffer = [0u8; 16];

    buffer[16 - size..].copy_from_slice(&bytes[..size]);

    u128::from_be_bytes(buffer)
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
    fn should_construct_matching_encode_decode_tables() {
        for byte in 0..64 {
            assert_eq!(
                construct_decode_map()[ENCODE_MAP[byte] as usize],
                byte as u8
            );
        }
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
