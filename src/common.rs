pub const ENCODE_MAP: &[u8; 64] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
pub const DECODE_MAP: &[u8; 256] = &construct_decode_map();

pub const SIX_BIT_MASK: u128 = 0x3f;
pub const BYTE_MASK: u64 = 0xff;
pub const INVALID_BYTE: u8 = 0x40;

pub const ENC_CHUNK_SIZE: usize = 4;
pub const DEC_CHUNK_SIZE: usize = 2;
pub const PARALLEL_THRESHOLD_BYTES: usize = 2 << 17; // 256 KiB

pub const ENC_U128_OFFSET: usize = (ENC_CHUNK_SIZE * 3 - 1) * 8;
pub const DEC_U64_OFFSET: usize = (DEC_CHUNK_SIZE * 4 - 1) * 6;

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
