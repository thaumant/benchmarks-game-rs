use criterion::{criterion_group, criterion_main, Criterion};
use benchmarks_game::{rehnberger, biffle, thaumant};

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

    c.bench_function("rehnberger", |b| {
        let mut rehnberger_bodies = rehnberger::STARTING_STATE;
        let mut rehnberger_sim = rehnberger::BodiesAdvance::new();
        rehnberger::offset_momentum(&mut rehnberger_bodies);
        b.iter(|| {
            for _ in 0..1000 {
                rehnberger_sim.advance(&mut rehnberger_bodies, 0.01);
            }
        })
    });

    c.bench_function("thaumant", |b| {
        let mut thaumant2_bodies = thaumant::STARTING_STATE;
        thaumant::offset_momentum(&mut thaumant2_bodies);
        b.iter(|| {
            thaumant::advance(&mut thaumant2_bodies, 1000);
        })
    });
}

criterion_group!(benches, nbody_benchmark);
criterion_main!(benches);
