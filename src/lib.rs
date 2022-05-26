mod alphabets;
#[cfg(test)]
mod tests;

pub use alphabets::*;
use std::mem::size_of;

#[derive(Debug)]
pub struct Base<const N: usize> {
    encode: [u8; N],
    decode: [u8; 128],
}

impl<const N: usize> Base<N> {
    pub const fn new(chars: &[u8; N]) -> Option<Self> {
        let encode = *chars;
        if encode.len() < 2 {
            return None;
        }
        let mut decode = [255; 128];
        let mut index = 0;
        while index < N {
            let val = encode[index] as usize;
            if val >= decode.len() || decode[val] != 255 {
                return None;
            }
            decode[val] = index as u8;
            index += 1;
        }
        Some(Self { encode, decode })
    }

    pub fn encode_const_len<const LEN: usize, T: ConstLen<LEN>>(&self, t: &T) -> String {
        let const_len = Self::const_len::<LEN>();
        let encoded = vec![0; const_len];
        self.encode_into(&t.to_bytes(), encoded)
    }

    pub fn decode_const_len<const LEN: usize, T: ConstLen<LEN>>(&self, s: &str) -> Option<T> {
        if s.len() != Self::const_len::<LEN>() {
            return None;
        }
        let mut bytes = [0; LEN];
        self.decode_into(s, &mut bytes)?;
        Some(T::from_bytes(bytes))
    }

    pub fn const_len<const LEN: usize>() -> usize {
        let len_upper_bound = Self::len_upper_bound(LEN);
        let len = Self::encode_into_raw(&[255; LEN], Vec::with_capacity(len_upper_bound)).len();
        debug_assert!(len <= len_upper_bound);
        len
    }

    pub fn encode_var_len<const LEN: usize, T: ConstLen<LEN>>(&self, t: &T) -> String {
        todo!()
    }

    pub fn decode_var_len<const LEN: usize, T: ConstLen<LEN>>(&self, s: &str) -> Option<T> {
        todo!()
    }

    fn encode_into_raw(input: &[u8], mut output: Vec<u8>) -> Vec<u8> {
        for &val in input {
            let mut carry = val as usize;
            for byte in &mut output {
                carry += (*byte as usize) << 8;
                *byte = (carry % N) as u8;
                carry /= N;
            }
            while carry > 0 {
                output.push(0);
                let last = output.len() - 1;
                output[last] = (carry % N) as u8;
                carry /= N;
            }
        }

        output.reverse();
        output
    }

    fn encode_into(&self, input: &[u8], output: Vec<u8>) -> String {
        let mut output = Self::encode_into_raw(input, output);

        for val in &mut output {
            *val = self.encode[*val as usize];
        }

        String::from_utf8(output).unwrap()
    }

    fn decode_into(&self, input: &str, output: &mut [u8]) -> Option<()> {
        let input = input.as_bytes();

        for &char in input {
            let mut val = *self.decode.get(char as usize)? as usize;
            if val == 255 {
                return None;
            }

            for byte in &mut *output {
                val += (*byte as usize) * N;
                *byte = (val & 0xFF) as u8;
                val >>= 8;
            }

            if val != 0 {
                return None;
            }
        }

        output.reverse();
        Some(())
    }

    const fn len_upper_bound(len: usize) -> usize {
        // div_ceil(len * 8, log2(N))
        let bits = 63 - N.leading_zeros() as usize;
        (len * 8 + (bits - 1)) / bits
    }
}

pub trait ConstLen<const LEN: usize> {
    fn to_bytes(&self) -> [u8; LEN];
    fn from_bytes(bytes: [u8; LEN]) -> Self;
}

impl<const LEN: usize> ConstLen<LEN> for [u8; LEN] {
    fn to_bytes(&self) -> [u8; LEN] {
        *self
    }

    fn from_bytes(bytes: [u8; LEN]) -> Self {
        bytes
    }
}

macro_rules! const_len_int_impl {
    ($($T:ty)*) => {$(
        impl ConstLen<{ size_of::<$T>() }> for $T {
            fn to_bytes(&self) -> [u8; size_of::<$T>()] {
                self.to_be_bytes()
            }

            fn from_bytes(bytes: [u8; size_of::<$T>()]) -> Self {
                Self::from_be_bytes(bytes)
            }
        }
    )*}
}

const_len_int_impl! { u8 u16 u32 u64 u128 }
