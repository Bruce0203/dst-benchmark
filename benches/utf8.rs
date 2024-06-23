use std::hint::black_box;

use criterion::Criterion;

const value: &[u8] =
    "qwerjasndfkashdiuzjksdbnfqjwehprqijk2bn4iq274htfquivhjka2ikb87uq3jhgetbkau2j4hwgbgiu"
        .as_bytes();

fn bench(c: &mut Criterion) {
    c.bench_function("bench function", |b| {
        b.iter(|| {
            let result = core::str::from_utf8(value);
            black_box(result);
        });
    });
}

criterion::criterion_main!(benches);
criterion::criterion_group!(benches, bench);
