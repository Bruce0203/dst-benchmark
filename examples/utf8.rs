#![feature(portable_simd)]
#![feature(array_chunks)]

use std::simd::{u8x16, u8x32, u8x4, u8x8};

fn main() {
    let data = "가".as_bytes();
    let data = "abcd".as_bytes();
    let data2 = is_plain_ascii(data);
    println!("{:?}", data);
    println!("{:?}", data2);
}

#[inline]
fn is_plain_ascii(slice: &[u8]) -> bool {
    let mut is_plain_ascii = true;
    let chunks_32_exact = slice.array_chunks::<32>();
    let mut remainder = chunks_32_exact.remainder();
    if remainder.len() > 16 {
        let chunk;
        (chunk, remainder) = remainder.split_first_chunk::<16>().unwrap();
        let mask = u8x16::splat(0b10000000);
        let zero = u8x16::splat(0);
        let simd = u8x16::from_array(*chunk);
        let and = simd & mask;
        if and != zero {
            is_plain_ascii = false;
        }
    }
    if remainder.len() > 8 {
        let chunk;
        (chunk, remainder) = remainder.split_first_chunk::<8>().unwrap();
        let mask = u8x8::splat(0b10000000);
        let zero = u8x8::splat(0);
        let simd = u8x8::from_array(*chunk);
        let and = simd & mask;
        if and != zero {
            is_plain_ascii = false;
        }
    }
    if remainder.len() > 4 {
        let chunk;
        (chunk, remainder) = remainder.split_first_chunk::<4>().unwrap();
        let mask = u8x4::splat(0b10000000);
        let zero = u8x4::splat(0);
        let simd = u8x4::from_array(*chunk);
        let and = simd & mask;
        if and != zero {
            is_plain_ascii = false;
        }
    }
    for &byte in remainder {
        if byte & 0b10000000 != 0 {
            is_plain_ascii = false;
        }
    }

    for &chunk in chunks_32_exact {
        let mask = u8x32::splat(0b10000000);
        let zero = u8x32::splat(0);
        let simd = u8x32::from_array(chunk);
        let and = simd & mask;
        if and != zero {
            is_plain_ascii = false;
        }
    }

    is_plain_ascii
}
