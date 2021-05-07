use concrete_fftw::plan::*;
use concrete_fftw::types::*;
use num_traits::Zero;

/// Check successive forward and backward transform equals to the identity
#[test]
fn c2r2c_identity_64() {
    let n = 32;
    let mut a = vec![c64::zero(); n / 2 + 1];
    let mut b = vec![0.0; n];
    let c2r: C2RPlan64 = C2RPlan::new(&[n], &mut a, &mut b, Flag::MEASURE).unwrap();
    let r2c: R2CPlan64 = R2CPlan::new(&[n], &mut b, &mut a, Flag::MEASURE).unwrap();
    for i in 0..(n / 2 + 1) {
        a[i] = c64::new(1.0, 0.0);
    }
    c2r.c2r(&mut a, &mut b).unwrap();
    r2c.r2c(&mut b, &mut a).unwrap();
    for v in a.iter() {
        let ans = c64::new(n as f64, 0.0);
        let dif = (v - ans).norm();
        if dif > 1e-7 {
            panic!("Large difference: v={}, dif={}", v, dif);
        }
    }
}

#[test]
fn c2r2c_identity_32() {
    let n = 32;
    let mut a = vec![c32::zero(); n / 2 + 1];
    let mut b = vec![0.0; n];
    let c2r: C2RPlan32 = C2RPlan::new(&[n], &mut a, &mut b, Flag::MEASURE).unwrap();
    let r2c: R2CPlan32 = R2CPlan::new(&[n], &mut b, &mut a, Flag::MEASURE).unwrap();
    for i in 0..(n / 2 + 1) {
        a[i] = c32::new(1.0, 0.0);
    }
    c2r.c2r(&mut a, &mut b).unwrap();
    r2c.r2c(&mut b, &mut a).unwrap();
    for v in a.iter() {
        let ans = c32::new(n as f32, 0.0);
        let dif = (v - ans).norm();
        if dif > 1e-7 {
            panic!("Large difference: v={}, dif={}", v, dif);
        }
    }
}
