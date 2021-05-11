use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fftw::plan::{Plan, C2CPlan, C2CPlan64};
use fftw::types::{Sign, Flag};
use fftw::array::AlignedVec;

const SIZE:usize = 1024;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("forward", |b| {
        let mut plan: C2CPlan64 = C2CPlan::aligned(&[SIZE], Sign::Forward, Flag::MEASURE).unwrap();
        let mut input = AlignedVec::new(SIZE);
        let mut output = AlignedVec::new(SIZE);
        b.iter(|| (black_box(plan.c2c(input.as_slice_mut(), output.as_slice_mut()))))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);