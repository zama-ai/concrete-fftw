use f128::f128;
use fftw::array::AlignedVec;
use fftw::plan::*;
use fftw::types::*;
use crossbeam::scope;

static N_TRANSFORMS:usize = 10_000;

fn test_f32(n:usize) {
    println!("Executing f32 transforms in series for size {}", n);
    let mut plan: C2CPlan32 = C2CPlan::aligned(&[n], Sign::Forward, Flag::MEASURE).unwrap();
    let now = std::time::Instant::now();
    let mut a = AlignedVec::new(n);
    let mut b = AlignedVec::new(n);
    let k0 = 2.0 * std::f32::consts::PI / n as f32;
    for i in 0..n {
        a[i] = c32::new((k0 * i as f32), 0.0);
    }
    for _ in 0..N_TRANSFORMS{
        plan.c2c(&mut a, &mut b).unwrap();
    }
    println!("   Duration: {} s",now.elapsed().as_secs_f32());
}

fn test_threaded_f32(n:usize) {
    println!("Executing f32 transforms in parallel for size {}", n);
    let mut plan: C2CPlan32 = C2CPlan::aligned(&[n], Sign::Forward, Flag::MEASURE).unwrap();
    let now = std::time::Instant::now();
    scope(|s|{
        let handles = (0..4).map(|_|{
            s.spawn(|_|{
                let mut a = AlignedVec::new(n);
                let mut b = AlignedVec::new(n);
                let k0 = 2.0 * std::f32::consts::PI / n as f32;
                for i in 0..n {
                    a[i] = c32::new((k0 * i as f32), 0.0);
                }
                for _ in 0..N_TRANSFORMS/4{
                    plan.c2c(&mut a, &mut b).unwrap();
                }
            })
        }).collect::<Vec<_>>();
        handles.into_iter().for_each(|h| h.join().unwrap());
    });
    println!("   Duration: {} s",now.elapsed().as_secs_f32());
}

fn test_f64(n:usize) {
    println!("Executing f64 transforms in series for size {}", n);
    let mut plan: C2CPlan64 = C2CPlan::aligned(&[n], Sign::Forward, Flag::MEASURE).unwrap();
    let now = std::time::Instant::now();
    let mut a = AlignedVec::new(n);
    let mut b = AlignedVec::new(n);
    let k0 = 2.0 * std::f64::consts::PI / n as f64;
    for i in 0..n {
        a[i] = c64::new((k0 * i as f64), 0.0);
    }
    for _ in 0..N_TRANSFORMS{
        plan.c2c(&mut a, &mut b).unwrap();
    }
    println!("   Duration: {} s",now.elapsed().as_secs_f32());
}

fn test_threaded_f64(n:usize) {
    println!("Executing f64 transforms in parallel for size {}", n);
    let mut plan: C2CPlan64 = C2CPlan::aligned(&[n], Sign::Forward, Flag::MEASURE).unwrap();
    let now = std::time::Instant::now();
    scope(|s|{
        let handles = (0..4).map(|_|{
            s.spawn(|_|{
                let mut a = AlignedVec::new(n);
                let mut b = AlignedVec::new(n);
                let k0 = 2.0 * std::f64::consts::PI / n as f64;
                for i in 0..n {
                    a[i] = c64::new((k0 * i as f64), 0.0);
                }
                for _ in 0..N_TRANSFORMS/4{
                    plan.c2c(&mut a, &mut b).unwrap();
                }
            })
        }).collect::<Vec<_>>();
        handles.into_iter().for_each(|h| h.join().unwrap());
    });
    println!("   Duration: {} s",now.elapsed().as_secs_f32());
}

fn test_f128(n:usize) {
    println!("Executing f128 transforms in serie for size {}", n);
    let mut plan: C2CPlan128 = C2CPlan::aligned(&[n], Sign::Forward, Flag::MEASURE).unwrap();
    let now = std::time::Instant::now();
    let mut a = AlignedVec::new(n);
    let mut b = AlignedVec::new(n);
    let k0 = f128::new(2.0_f64) * f128::PI / f128::from(n);
    for i in 0..n {
        a[i] = c128::new((k0 * f128::from(i)), f128::ZERO);
    }
    for _ in 0..N_TRANSFORMS{
        plan.c2c(&mut a, &mut b).unwrap();
    }
    println!("   Duration: {} s",now.elapsed().as_secs_f32());
}

fn test_threaded_f128(n:usize) {
    println!("Executing f128 transforms in parallel for size {}", n);
    let mut plan: C2CPlan128 = C2CPlan::aligned(&[n], Sign::Forward, Flag::MEASURE).unwrap();
    let now = std::time::Instant::now();
    scope(|s|{
        let handles = (0..4).map(|_|{
            s.spawn(|_|{
                let mut a = AlignedVec::new(n);
                let mut b = AlignedVec::new(n);
                let k0 = f128::new(2.0_f64) * f128::PI / f128::from(n);
                for i in 0..n {
                    a[i] = c128::new((k0 * f128::from(i)), f128::ZERO);
                }
                for _ in 0..N_TRANSFORMS/4{
                    plan.c2c(&mut a, &mut b).unwrap();
                }
            })
        }).collect::<Vec<_>>();
        handles.into_iter().for_each(|h| h.join().unwrap());
    });
    println!("   Duration: {} s",now.elapsed().as_secs_f32());
}

fn main() {

    println!("==================================================> F32");
    test_f32(256);
    test_threaded_f32(256);
    test_f32(512);
    test_threaded_f32(512);
    test_f32(1024);
    test_threaded_f32(1024);
    test_f32(2048);
    test_threaded_f32(2048);
    test_f32(4096);
    test_threaded_f32(4096);

    println!("==================================================> F64");
    test_f64(256);
    test_threaded_f64(256);
    test_f64(512);
    test_threaded_f64(512);
    test_f64(1024);
    test_threaded_f64(1024);
    test_f64(2048);
    test_threaded_f64(2048);
    test_f64(4096);
    test_threaded_f64(4096);

    println!("==================================================> F128");
    test_f128(256);
    test_threaded_f128(256);
    test_f128(512);
    test_threaded_f128(512);
    test_f128(1024);
    test_threaded_f128(1024);
    test_f128(2048);
    test_threaded_f128(2048);
    test_f128(4096);
    test_threaded_f128(4096);

}
