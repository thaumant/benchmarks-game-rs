use criterion::{criterion_group, criterion_main, Criterion};
use dangerust_6::{original, modified1, modified2};

pub fn dangerust_benchmark(c: &mut Criterion) {

    c.bench_function("original", |b| {
        let mut original_bodies = original::STARTING_STATE;
        original::offset_momentum(&mut original_bodies);
        b.iter(|| {
            for _ in 0..1000 {
                original::advance(&mut original_bodies);
            }
        })
    });

    c.bench_function("modified1", |b| {
        let mut modified2_bodies = modified1::STARTING_STATE;
        modified1::offset_momentum(&mut modified2_bodies);
        b.iter(|| {
            for _ in 0..1000 {
                modified1::advance(&mut modified2_bodies);
            }
        })
    });

    c.bench_function("modified2", |b| {
        let mut modified2_bodies = modified2::STARTING_STATE;
        modified2::offset_momentum(&mut modified2_bodies);
        b.iter(|| {
            for _ in 0..1000 {
                modified2::advance(&mut modified2_bodies);
            }
        })
    });
}

criterion_group!(benches, dangerust_benchmark);
criterion_main!(benches);
