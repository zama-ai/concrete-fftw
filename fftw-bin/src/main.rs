use f128::f128;
use fftw::array::AlignedVec;
use fftw::plan::*;
use fftw::types::*;


fn test_f32(n:usize) {
    let mut plan: C2CPlan32 = C2CPlan::aligned(&[n], Sign::Forward, Flag::MEASURE).unwrap();
    let mut a = AlignedVec::new(n);
    let mut b = AlignedVec::new(n);
    let k0 = 2.0 * std::f32::consts::PI / n as f32;
    for i in 0..n {
        a[i] = c32::new((k0 * i as f32), 0.0);
    }

    let now = std::time::Instant::now();
    for _ in 0..1000{
        plan.c2c(&mut a, &mut b).unwrap();
    }
    println!("Execution duration of f32 test for size {}: {} s", n, now.elapsed().as_secs_f32());
}

fn test_f64(n:usize) {
    let mut plan: C2CPlan64 = C2CPlan::aligned(&[n], Sign::Forward, Flag::MEASURE).unwrap();
    let mut a = AlignedVec::new(n);
    let mut b = AlignedVec::new(n);
    let k0 = 2.0 * std::f64::consts::PI / n as f64;
    for i in 0..n {
        a[i] = c64::new((k0 * i as f64), 0.0);
    }

    let now = std::time::Instant::now();
    for _ in 0..1000{
        plan.c2c(&mut a, &mut b).unwrap();
    }
    println!("Execution duration of f64 test for size {}: {} s", n, now.elapsed().as_secs_f32());
}

fn test_f128(n:usize) {
    let mut plan: C2CPlan128 = C2CPlan::aligned(&[n], Sign::Forward, Flag::MEASURE).unwrap();
    let mut a = AlignedVec::new(n);
    let mut b = AlignedVec::new(n);
    let k0 = f128::new(2.0_f64) * f128::PI / f128::from(n);
    for i in 0..n {
        a[i] = c128::new((k0 * f128::from(i)), f128::ZERO);
    }

    let now = std::time::Instant::now();
    for _ in 0..1000{
        plan.c2c(&mut a, &mut b).unwrap();
    }
    println!("Execution duration of f128 test for size {}: {} s", n, now.elapsed().as_secs_f32());
}

fn main() {
    test_f32(256);
    test_f32(512);
    test_f32(1024);
    test_f32(2048);
    test_f32(4096);

    test_f64(256);
    test_f64(512);
    test_f64(1024);
    test_f64(2048);
    test_f64(4096);

    test_f128(256);
    test_f128(512);
    test_f128(1024);
    test_f128(2048);
    test_f128(4096);
}
