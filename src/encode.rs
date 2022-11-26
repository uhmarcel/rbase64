use crate::common::*;
use std::cmp::min;
use std::iter::zip;

#[inline(always)]
#[cfg(not(feature = "parallel"))]
pub(crate) fn encode_u128_chunks(input: &[u8], buffer: &mut [u8]) {
    encode_u128_chunks_sync(input, buffer);
}

#[inline(always)]
#[cfg(feature = "parallel")]
pub(crate) fn encode_u128_chunks(input: &[u8], buffer: &mut [u8]) {
    if input.len() < PARALLEL_THRESHOLD_BYTES {
        encode_u128_chunks_sync(input, buffer);
    } else {
        encode_u128_chunks_parallel(input, buffer);
    };
}

#[inline(always)]
fn encode_u128_chunks_sync(input: &[u8], buffer: &mut [u8]) {
    let in_chunks = input.chunks_exact(ENC_CHUNK_SIZE * 3);
    let out_chunks = buffer.chunks_exact_mut(ENC_CHUNK_SIZE * 4);

    zip(in_chunks, out_chunks).for_each(|(in_chunk, out_chunk)| {
        encode_u128(in_chunk, out_chunk);
    });
}

#[inline(always)]
#[cfg(feature = "parallel")]
fn encode_u128_chunks_parallel(input: &[u8], buffer: &mut [u8]) {
    use rayon::prelude::*;

    let batch_size = PARALLEL_BATCH_SIZE * ENC_CHUNK_SIZE;
    let in_batches = input.par_chunks(batch_size * 3);
    let out_batches = buffer.par_chunks_mut(batch_size * 4);

    in_batches
        .zip(out_batches)
        .for_each(|(in_batch, out_batch)| {
            encode_u128_chunks_sync(in_batch, out_batch);
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
    let offset = (ENC_CHUNK_SIZE * 3 - 1) * 8;

    buffer.iter_mut().enumerate().for_each(|(i, out_b)| {
        *out_b = encode_byte(((in_u128 >> (2 + offset - (i * 6))) & SIX_BIT_MASK) as u8);
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
