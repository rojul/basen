use super::*;
use std::fmt::Debug;

fn test_ende_const_len<const N: usize, const LEN: usize, T>(base: Base<N>, bytes: T, str: &str)
where
    T: ConstLen<LEN> + Debug + PartialEq,
{
    assert_eq!(base.encode_const_len(&bytes), str);
    assert_eq!(base.decode_const_len(&str), Some(bytes));
}

fn hex<const LEN: usize>(s: &str) -> [u8; LEN] {
    BASE16.decode_const_len(s).unwrap()
}

#[test]
fn bounds() {
    assert_eq!(Base::<10>::const_len::<8>(), 20);
    test_ende_const_len(BASE10, 0u64, "00000000000000000000");
    test_ende_const_len(BASE10, 1u64, "00000000000000000001");
    test_ende_const_len(BASE10, u64::MAX - 1, &(u64::MAX - 1).to_string());
    test_ende_const_len(BASE10, u64::MAX, &u64::MAX.to_string());

    assert_eq!(Base::<36>::const_len::<8>(), 13);
    test_ende_const_len(BASE36, 0u64, "0000000000000");
    test_ende_const_len(BASE36, 1u64, "0000000000001");
    test_ende_const_len(BASE36, u64::MAX - 1, "3w5e11264sgse");
    test_ende_const_len(BASE36, u64::MAX, "3w5e11264sgsf");

    assert_eq!(Base::<58>::const_len::<8>(), 11);
    test_ende_const_len(BASE58, 0u64, "11111111111");
    test_ende_const_len(BASE58, 1u64, "11111111112");
    test_ende_const_len(BASE58, u64::MAX - 1, "jpXCZedGfVP");
    test_ende_const_len(BASE58, u64::MAX, "jpXCZedGfVQ");
}

#[test]
fn decode_overflow() {
    assert_eq!(BASE58.decode_const_len("zzzzzzzzzzz"), None::<u64>);
    assert_eq!(BASE58.decode_var_len("zzzzzzzzzzz"), None::<u64>);
}

#[test]
fn decode_invalid_chars() {
    assert_eq!(BASE58.decode_const_len("-----------"), None::<u64>);
}

#[test]
fn decode_invalid_length() {
    assert_eq!(BASE58.decode_const_len(""), None::<u64>);
    assert_eq!(BASE58.decode_const_len("1111111111"), None::<u64>);
    assert_eq!(BASE58.decode_const_len("111111111111"), None::<u64>);
}

#[test]
fn base58_512bit() {
    test_ende_const_len(
        BASE58,
        hex::<64>("1f78a149865616cdab285690e687cd9facdd1393a70faa0f6bf8c726c1f037b3bbc261c16182a4d62550af4c596cf44a658a64b8f1acc5dbafddb8d3dd7109e7"),
        "1dVfUxKg2Py5dJQSDgnJrZ7xiALQ5XgB3vww5Vibnqwf2MneQnALM5H8uqZUWwSywWAuHtU2Mx5J8LqwHAMiju8a",
    );
}

#[test]
fn var_len() {
    assert_eq!(BASE10.encode_var_len(&0u64), "");
    assert_eq!(BASE10.decode_var_len(""), Some(0u64));
    assert_eq!(BASE10.decode_var_len("0000"), Some(0u8));

    assert_eq!(BASE10.encode_var_len(&1u64), "1");
    assert_eq!(BASE10.decode_var_len("1"), Some(1u64));
    assert_eq!(BASE10.decode_var_len("0001"), Some(1u8));

    assert_eq!(BASE10.encode_var_len(&u64::MAX), u64::MAX.to_string());
}
