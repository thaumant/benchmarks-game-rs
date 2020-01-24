use criterion::{criterion_group, criterion_main, Criterion};
use benchmarks_game::{biffle, thaumant1, thaumant2};

pub fn nbody_benchmark(c: &mut Criterion) {

    c.bench_function("biffle", |b| {
        let mut biffle_bodies = biffle::STARTING_STATE;
        biffle::offset_momentum(&mut biffle_bodies);
        b.iter(|| {
            for _ in 0..1000 {
                biffle::advance(&mut biffle_bodies);
            }
        })
    });

    c.bench_function("thaumant1", |b| {
        let mut thaumant2_bodies = thaumant1::STARTING_STATE;
        thaumant1::offset_momentum(&mut thaumant2_bodies);
        b.iter(|| {
            for _ in 0..1000 {
                thaumant1::advance(&mut thaumant2_bodies);
            }
        })
    });

    c.bench_function("thaumant2", |b| {
        let mut thaumant2_bodies = thaumant2::STARTING_STATE;
        thaumant2::offset_momentum(&mut thaumant2_bodies);
        b.iter(|| {
            for _ in 0..1000 {
                thaumant2::advance(&mut thaumant2_bodies);
            }
        })
    });
}

criterion_group!(benches, nbody_benchmark);
criterion_main!(benches);
