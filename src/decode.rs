use crate::common::*;
use std::iter::zip;

#[inline(always)]
#[cfg(not(feature = "parallel"))]
pub(crate) fn decode_u64_chunks(input: &[u8], buffer: &mut [u8]) {
    decode_u64_chunks_sync(input, buffer);
}

#[inline(always)]
#[cfg(feature = "parallel")]
pub(crate) fn decode_u64_chunks(input: &[u8], buffer: &mut [u8]) {
    if input.len() < PARALLEL_THRESHOLD_BYTES {
        decode_u64_chunks_sync(input, buffer);
    } else {
        decode_u64_chunks_parallel(input, buffer);
    };
}

#[inline(always)]
fn decode_u64_chunks_sync(input: &[u8], buffer: &mut [u8]) {
    let in_chunks = input.chunks_exact(DEC_CHUNK_SIZE * 4);
    let out_chunks = buffer.chunks_exact_mut(DEC_CHUNK_SIZE * 3);

    for (in_chunk, out_chunk) in zip(in_chunks, out_chunks) {
        decode_u64(in_chunk, out_chunk);
    }
}

#[inline(always)]
#[cfg(feature = "parallel")]
fn decode_u64_chunks_parallel(input: &[u8], buffer: &mut [u8]) {
    use rayon::prelude::*;

    let batch_size = PARALLEL_BATCH_SIZE * DEC_CHUNK_SIZE;
    let in_batch = input.par_chunks(batch_size * 4);
    let out_batch = buffer.par_chunks_mut(batch_size * 3);

    in_batch.zip(out_batch).for_each(|(in_chunk, out_chunk)| {
        decode_u64_chunks_sync(in_chunk, out_chunk);
    });
}

#[inline(always)]
pub(crate) fn decode_u64_remainder(input: &[u8], buffer: &mut [u8]) -> usize {
    let mut in_u64 = 0u64;
    let mut in_bits = 0;

    for in_byte in input.iter() {
        if *in_byte == b'=' {
            break;
        }
        in_u64 = (in_u64 << 6) | decode_byte(*in_byte) as u64;
        in_bits += 6;
    }

    let byte_count = in_bits / 8;

    for (i, out_byte) in buffer[..byte_count].iter_mut().enumerate() {
        *out_byte = ((in_u64 >> (in_bits - 8 * (i + 1))) & BYTE_MASK) as u8;
    }
    byte_count
}

#[inline(always)]
fn decode_u64(input: &[u8], buffer: &mut [u8]) {
    let mut in_u64 = 0u64;
    let offset = (DEC_CHUNK_SIZE * 4 - 1) * 6;

    input.iter().enumerate().for_each(|(i, in_byte)| {
        in_u64 |= (decode_byte(*in_byte) as u64) << (offset - (6 * i) + 2) as u64;
    });

    buffer.iter_mut().enumerate().for_each(|(i, out_byte)| {
        *out_byte = ((in_u64 >> (offset - (i * 8))) & BYTE_MASK) as u8;
    });
}

#[inline(always)]
fn decode_byte(byte: u8) -> u8 {
    let decoded = DECODE_MAP[byte as usize];

    if decoded == INVALID_BYTE {
        panic!("Unable to decode non-base64 character '{}'", byte as char)
    }
    decoded
}
