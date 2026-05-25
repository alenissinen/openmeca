//! Little-endian decoding shared by the analog devices.
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
