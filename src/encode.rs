use crate::common::*;
use rayon::prelude::*;
use std::cmp::min;
use std::iter::zip;

#[inline(always)]
pub(crate) fn encode_u128_chunks(input: &[u8], buffer: &mut [u8]) {
    let in_chunks = input.chunks_exact(ENC_CHUNK_SIZE * 3);
    let out_chunks = buffer.chunks_exact_mut(ENC_CHUNK_SIZE * 4);

    zip(in_chunks, out_chunks).for_each(|(in_chunk, out_chunk)| {
        encode_u128(in_chunk, out_chunk);
    });
}

#[inline(always)]
pub(crate) fn encode_u128_chunks_parallel(input: &[u8], buffer: &mut [u8]) {
    let in_chunks = input.par_chunks_exact(ENC_CHUNK_SIZE * 3);
    let out_chunks = buffer.par_chunks_exact_mut(ENC_CHUNK_SIZE * 4);

    in_chunks.zip(out_chunks).for_each(|(in_chunk, out_chunk)| {
        encode_u128(in_chunk, out_chunk);
    });
}

#[inline(always)]
pub(crate) fn encode_u128_remainder(input: &[u8], buffer: &mut [u8]) -> usize {
    let in_u128 = read_u128_partial(input);
    let mut in_bits = 8 * input.len();
    let mut out_index = 0;

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
    out_index
}

#[inline(always)]
fn encode_u128(input: &[u8], buffer: &mut [u8]) {
    let in_u128 = read_u128_partial(input);

    buffer.iter_mut().enumerate().for_each(|(i, out_b)| {
        *out_b = encode_byte(((in_u128 >> (2 + ENC_U128_OFFSET - 6 * i)) & SIX_BIT_MASK) as u8);
    });
}

#[inline(always)]
fn read_u128_partial(bytes: &[u8]) -> u128 {
    let size = min(bytes.len(), 16);
    let mut buffer = [0u8; 16];

    buffer[16 - size..].copy_from_slice(&bytes[..size]);

    u128::from_be_bytes(buffer)
}

#[inline(always)]
fn encode_byte(byte: u8) -> u8 {
    ENCODE_MAP[byte as usize]
}
