#![feature(core_intrinsics)]

use std::{hint::black_box, mem::MaybeUninit};

use criterion::{Criterion, Throughput};
use fast_collections::{const_transmute_unchecked, GetUnchecked, Vec};
use generic_array::typenum::U4;

pub struct NewTypeOfU8(pub u8);

impl Default for NewTypeOfU8 {
    fn default() -> Self {
        NewTypeOfU8(1)
    }
}

const SIZED_SLICE: Vec<NewTypeOfU8, U4> =
    Vec::from_array(unsafe { const_transmute_unchecked::<[u8; 4], [NewTypeOfU8; 4]>(*b"ABCD") });
const UNSIZED_SLICE: &[NewTypeOfU8] =
    unsafe { const_transmute_unchecked::<&[u8], &[NewTypeOfU8]>(b"ABCD") };

fn benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("benches");
    group.throughput(Throughput::Bytes(1000));

    group.bench_function("bench boxed unsized slice", |b| {
        b.iter(|| {
            for _ in 0..1000 {
                let value: Box<[MaybeUninit<u8>]> = Box::new([MaybeUninit::<u8>::uninit(); 4]);
                black_box(value);
            }
        })
    });

    let mut vec = std::vec::Vec::<NewTypeOfU8>::with_capacity(4);
    vec.push(NewTypeOfU8(1));
    vec.push(NewTypeOfU8(2));
    vec.push(NewTypeOfU8(3));
    vec.push(NewTypeOfU8(4));

    group.bench_function("bench alloc vec", |b| {
        b.iter(|| {
            for _ in 0..1000 {
                black_box(std::vec::Vec::<u8>::with_capacity(4));
            }
        })
    });
    group.bench_function("bench sized slice", |b| {
        b.iter(|| {
            for _ in 0..1000 {
                black_box(Vec::<u8, U4>::uninit());
            }
        })
    });
    group.bench_function("bench unsized slice", |b| {
        b.iter(|| {
            for _ in 0..1000 {
                let value: &[MaybeUninit<u8>] = &[MaybeUninit::<u8>::uninit(); 4];
                black_box(value);
            }
        })
    });
}

criterion::criterion_main!(benches);
criterion::criterion_group!(benches, benchmark);
