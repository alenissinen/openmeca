//! Little-endian encoding and decoding shared by the analog devices.
//! All EVO device reports store values as 16-bit little-endian, see `docs/`.

/// Reads one le `u16` from the start of `bytes`.
/// Returns `None` if fewer thon two bytes are available, doesn't panic.
pub(crate) fn read_u16_le(bytes: &[u8]) -> Option<u16> {
    let pair: [u8; 2] = bytes.get(..2)?.try_into().ok()?;
    Some(u16::from_le_bytes(pair))
}

/// Iterates the le `u16` values in `bytes`, two bytes at a time.
/// Borrows `bytes` for the iterators lifetime, doesn't allocate aynthing.
pub(crate) fn iter_u16_le(bytes: &[u8]) -> impl Iterator<Item = u16> + '_ {
    bytes
        .chunks_exact(2)
        .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
}

/// Encodes 32 x u16 values into a 64-byte buffer.
pub(crate) fn encode_u16_le_buffer(values: &[u16; 32]) -> [u8; 64] {
    let mut buf = [0u8; 64];

    for (i, &v) in values.iter().enumerate() {
        let bytes = v.to_le_bytes();
        buf[i * 2] = bytes[0];
        buf[i * 2 + 1] = bytes[1];
    }

    buf
}

/// Decodes a 64-byte buffer into 32 x u16 values.
pub(crate) fn decode_u16_le_buffer(buf: &[u8; 64]) -> [u16; 32] {
    let mut values = [0u16; 32];

    for i in 0..32 {
        values[i] = u16::from_le_bytes([buf[i * 2], buf[i * 2 + 1]]);
    }

    values
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_u16_le_decodes_correctly() {
        assert_eq!(read_u16_le(&[0x00, 0x10]), Some(0x1000));
        assert_eq!(read_u16_le(&[0x8C, 0x00]), Some(0x008C));
        assert_eq!(read_u16_le(&[0xFF, 0xFF]), Some(0xFFFF));
        assert_eq!(read_u16_le(&[0x00, 0x00]), Some(0x0000));
    }

    #[test]
    fn read_u16_le_rejects_short_input() {
        assert_eq!(read_u16_le(&[]), None);
        assert_eq!(read_u16_le(&[0x42]), None);
    }

    #[test]
    fn iter_u16_le_decodes_sequence() {
        let buf = [0x00, 0x10, 0x8C, 0x00, 0xAA, 0x02];
        let values: Vec<u16> = iter_u16_le(&buf).collect();

        assert_eq!(values, vec![0x1000, 0x008C, 0x02AA]);
    }

    #[test]
    fn iter_u16_le_ignores_trailing_byte() {
        let buf = [0x01, 0x00, 0xFF];
        let values: Vec<u16> = iter_u16_le(&buf).collect();

        assert_eq!(values, vec![0x0001]);
    }

    #[test]
    fn encode_decode_roundtrip() {
        let mut original = [0u16; 32];
        original[0] = 0xF102;
        original[1] = 113;
        original[2] = 0;
        original[3] = 140;
        original[4] = 0;
        original[5] = 251;
        original[6] = 682;
        original[18] = 4096;

        let bytes = encode_u16_le_buffer(&original);
        let decoded = decode_u16_le_buffer(&bytes);

        assert_eq!(original, decoded);
    }

    #[test]
    fn encode_decode_all_zeros() {
        let original = [0u16; 32];
        let bytes = encode_u16_le_buffer(&original);
        let decoded = decode_u16_le_buffer(&bytes);

        assert_eq!(original, decoded);
    }

    #[test]
    fn encode_decode_all_max() {
        let original = [0xFFFFu16; 32];
        let bytes = encode_u16_le_buffer(&original);
        let decoded = decode_u16_le_buffer(&bytes);

        assert_eq!(original, decoded);
    }

    #[test]
    fn encode_produces_correct_bytes() {
        let mut values = [0u16; 32];
        values[0] = 0xF102;
        values[1] = 0x1000;

        let bytes = encode_u16_le_buffer(&values);

        assert_eq!(bytes[0], 0x02);
        assert_eq!(bytes[1], 0xF1);
        assert_eq!(bytes[2], 0x00);
        assert_eq!(bytes[3], 0x10);
    }
}
