## CLSAG

This is a pure Rust implementation of the Compact Linkable Spontaneous Anonymous Group construction.

- This is a fork of the [MLSAG](https://github.com/crate-crypto/MLSAG) implementation.

- This implementation has not been reviewed or auditted. Use at your own risk.


## Rust

In order to compile, you must be on Rust nightly.

## Benchmarks

The following table gives benchmarks for dual-key verification using ristretto255.

| Ring size     | Verification (μs) |
|:-------------:|:-----------------:|
| 2             | 997               |
| 4             | 1,385             |
| 6             | 2,026             |
| 8             | 2,855             |
| 11            | 3,695             |
| 16            | 5,399             |
| 32            | 11,939            |
| 64            | 24,824            |
| 128           | 48,453            |
| 256           | 107,200           |
| 512           | 267,760           |

*CPU : 2.2 GHz Intel Core i7*



## Paper

https://eprint.iacr.org/2019/654

## License

Licensed under MIT: 

- MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)

