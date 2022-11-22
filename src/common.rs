pub const ENCODE_MAP: &[u8; 64] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
pub const DECODE_MAP: &[u8; 256] = &construct_decode_map();

pub const ENC_CHUNK_SIZE: usize = 2;
pub const DEC_CHUNK_SIZE: usize = 2;

pub const SIX_BIT_MASK: u64 = 0x3f;
pub const BYTE_MASK: u64 = 0xff;
pub const INVALID_BYTE: u8 = 0x40;

#[cfg(feature = "parallel")]
pub const PARALLEL_THRESHOLD_BYTES: usize = 2 << 16; // 128 KiB
#[cfg(feature = "parallel")]
pub const PARALLEL_BATCH_SIZE: usize = 256;

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
    use crate::common::construct_decode_map;
    use crate::ENCODE_MAP;

    #[test]
    fn should_construct_matching_encode_decode_tables() {
        for byte in 0..64 {
            assert_eq!(
                construct_decode_map()[ENCODE_MAP[byte] as usize],
                byte as u8
            );
        }
    }
}
