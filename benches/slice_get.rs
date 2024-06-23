#![feature(core_intrinsics)]

use std::hint::black_box;

use criterion::Criterion;
use fast_collections::{const_transmute_unchecked, GetUnchecked, Vec};
use generic_array::typenum::U4;

pub struct NewTypeOfU8(pub u8);

const SIZED_SLICE: Vec<NewTypeOfU8, U4> =
    Vec::from_array(unsafe { const_transmute_unchecked::<[u8; 4], [NewTypeOfU8; 4]>(*b"ABCD") });
const UNSIZED_SLICE: &[NewTypeOfU8] =
    unsafe { const_transmute_unchecked::<&[u8], &[NewTypeOfU8]>(b"ABCD") };

fn benchmark(c: &mut Criterion) {
    let mut vec = std::vec::Vec::<NewTypeOfU8>::with_capacity(4);
    vec.push(NewTypeOfU8(1));
    vec.push(NewTypeOfU8(2));
    vec.push(NewTypeOfU8(3));
    vec.push(NewTypeOfU8(4));
    c.bench_function("bench alloc vec", |b| {
        b.iter(|| {
            for _ in 0..1000 {
                unsafe { black_box((&*vec.get_unchecked(0)).0) };
            }
        })
    });
    c.bench_function("bench sized slice", |b| {
        b.iter(|| {
            for _ in 0..1000 {
                unsafe { black_box((&*SIZED_SLICE.get_unchecked_ref(0)).0) };
            }
        })
    });
    c.bench_function("bench unsized slice", |b| {
        b.iter(|| {
            for _ in 0..1000 {
                unsafe { black_box((&*UNSIZED_SLICE.get_unchecked(0)).0) };
            }
        })
    });
}

criterion::criterion_main!(benches);
criterion::criterion_group!(benches, benchmark);
