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
