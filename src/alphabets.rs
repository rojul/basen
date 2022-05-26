use crate::Base;

pub const BASE10: Base<10> = base(b"0123456789");
pub const BASE16: Base<16> = base(b"0123456789abcdef");
// BASE32
pub const BASE36: Base<36> = base(b"0123456789abcdefghijklmnopqrstuvwxyz");
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
