# BaseN

Convert binary data to ASCII with a variety of supported bases.

```rust
assert_eq!(BASE58.encode_const_len(&1557596383284252235u64), "4chjCmhbVFY");
assert_eq!(BASE58.decode_const_len("4chjCmhbVFY"), Some(1557596383284252235u64));
```

Characters required (without padding):

| bits   | 32 | 64 | 128 | 256 | 512 |
|--------|---:|---:|----:|----:|----:|
| Base10 | 10 | 20 |  39 |  78 | 155 |
| Base16 |  8 | 16 |  32 |  64 | 128 |
| Base32 |  7 | 13 |  26 |  52 | 103 |
| Base36 |  7 | 13 |  25 |  50 | 100 |
| Base58 |  6 | 11 |  22 |  44 |  88 |
| Base62 |  6 | 11 |  22 |  43 |  86 |
| Base64 |  6 | 11 |  22 |  43 |  86 |
