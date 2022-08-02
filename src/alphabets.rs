use crate::Base;

/// Decimal
///
/// Characters: 0-9
pub const BASE10: Base<10> = base(b"0123456789");

/// Hexadecimal
///
/// Characters: 0-9 a-f
pub const BASE16: Base<16> = base(b"0123456789abcdef");

// BASE32

/// Characters: 0-9 a-z
pub const BASE36: Base<36> = base(b"0123456789abcdefghijklmnopqrstuvwxyz");

/// Characters: 0-9 A-Z a-z except 0IOl
pub const BASE58: Base<58> = base(b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz");

const BASE64_STD: Base<64> =
    base(b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/");

const BASE64_URL_SAFE: Base<64> =
    base(b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_");

const fn base<const N: usize>(chars: &[u8; N]) -> Base<N> {
    match Base::new(chars) {
        Some(val) => val,
        None => panic!(),
    }
}
