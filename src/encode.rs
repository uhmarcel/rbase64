use crate::common::*;
use std::cmp::min;
use std::iter::zip;

#[inline(always)]
#[cfg(not(feature = "parallel"))]
pub(crate) fn encode_u64_chunks(input: &[u8], buffer: &mut [u8]) {
    encode_u64_chunks_sync(input, buffer);
}

#[inline(always)]
#[cfg(feature = "parallel")]
pub(crate) fn encode_u64_chunks(input: &[u8], buffer: &mut [u8]) {
    if input.len() < PARALLEL_THRESHOLD_BYTES {
        encode_u64_chunks_sync(input, buffer);
    } else {
        encode_u64_chunks_parallel(input, buffer);
    };
}

#[inline(always)]
fn encode_u64_chunks_sync(input: &[u8], buffer: &mut [u8]) {
    let in_chunks = input.chunks_exact(ENC_CHUNK_SIZE * 3);
    let out_chunks = buffer.chunks_exact_mut(ENC_CHUNK_SIZE * 4);

    zip(in_chunks, out_chunks).for_each(|(in_chunk, out_chunk)| {
        encode_u64(in_chunk, out_chunk);
    });
}

#[inline(always)]
#[cfg(feature = "parallel")]
fn encode_u64_chunks_parallel(input: &[u8], buffer: &mut [u8]) {
    use rayon::prelude::*;

    let in_chunks = input.par_chunks_exact(ENC_CHUNK_SIZE * 3);
    let out_chunks = buffer.par_chunks_exact_mut(ENC_CHUNK_SIZE * 4);

    in_chunks.zip(out_chunks).for_each(|(in_chunk, out_chunk)| {
        encode_u64(in_chunk, out_chunk);
    });
}

#[inline(always)]
pub(crate) fn encode_u64_remainder(input: &[u8], buffer: &mut [u8]) -> usize {
    let in_u64 = read_u64_partial(input);
    let mut in_bits = 8 * input.len();
    let mut out_index = 0;

    while in_bits >= 6 {
        in_bits -= 6;
        buffer[out_index] = encode_byte(((in_u64 >> in_bits) & SIX_BIT_MASK) as u8);
        out_index += 1;
    }

    if in_bits > 0 {
        buffer[out_index] = encode_byte(((in_u64 << (6 - in_bits)) & SIX_BIT_MASK) as u8);
        out_index += 1;
    }

    while out_index % 4 > 0 {
        buffer[out_index] = b'=';
        out_index += 1;
    }
    out_index
}

#[inline(always)]
fn encode_u64(input: &[u8], buffer: &mut [u8]) {
    let in_u64 = read_u64_partial(input);

    buffer.iter_mut().enumerate().for_each(|(i, out_b)| {
        *out_b = encode_byte(((in_u64 >> (2 + ENC_U128_OFFSET - 6 * i)) & SIX_BIT_MASK) as u8);
    });
}

#[inline(always)]
fn read_u64_partial(bytes: &[u8]) -> u64 {
    let mut buffer = [0u8; 8];
    let size = min(bytes.len(), 8);

    buffer[8 - size..].copy_from_slice(&bytes[..size]);

    u64::from_be_bytes(buffer)
}

#[inline(always)]
fn encode_byte(byte: u8) -> u8 {
    ENCODE_MAP[byte as usize]
}
