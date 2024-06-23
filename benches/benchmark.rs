use std::{hint::black_box, mem::MaybeUninit};

use criterion::{Criterion, Throughput};
use fast_collections::{GetUnchecked, Vec};
use generic_array::typenum::U4;

fn benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("benches");
    group.throughput(Throughput::Bytes(1000));

    group.bench_function("bench sized slice", |b| {
        b.iter(|| {
            for _ in 0..1000 {
                let value = Vec::<u8, U4>::uninit();
                let _: u8 = *black_box(unsafe { value.get_unchecked_ref(2) });
            }
        })
    });
    group.bench_function("bench unsized slice", |b| {
        b.iter(|| {
            for _ in 0..1000 {
                let value: &[MaybeUninit<u8>] = &[MaybeUninit::<u8>::uninit(); 4];
                black_box(value);
                let _: MaybeUninit<u8> = *black_box(unsafe { value.get_unchecked(2) });
            }
        })
    });
}

criterion::criterion_main!(benches);
criterion::criterion_group!(benches, benchmark);
