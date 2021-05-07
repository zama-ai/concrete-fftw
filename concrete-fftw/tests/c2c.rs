use concrete_fftw::plan::*;
use concrete_fftw::types::*;
use num_traits::Zero;

/// Check successive forward and backward transform equals to the identity
#[test]
fn c2c2c_identity_64() {
    let n = 32;
    let mut a = vec![c64::zero(); n];
    let mut b = vec![c64::zero(); n];

    let plan_ab: C2CPlan64 =
        C2CPlan::new(&[n], &mut a, &mut b, Sign::Forward, Flag::MEASURE).unwrap();
    let plan_ba: C2CPlan64 =
        C2CPlan::new(&[n], &mut b, &mut a, Sign::Forward, Flag::MEASURE).unwrap();

    for i in 0..n {
        a[i] = c64::new(1.0, 0.0);
    }

    plan_ab.c2c(&mut a, &mut b).unwrap();
    plan_ba.c2c(&mut b, &mut a).unwrap();
    for v in a.iter() {
        let ans = c64::new(n as f64, 0.0);
        let dif = (v - ans).norm();
        if dif > 1e-7 {
            panic!("Large difference: v={}, dif={}", v, dif);
        }
    }
}

#[test]
fn c2c2c_identity_32() {
    let n = 32;
    let mut a = vec![c32::zero(); n];
    let mut b = vec![c32::zero(); n];

    let plan_ab: C2CPlan32 =
        C2CPlan::new(&[n], &mut a, &mut b, Sign::Forward, Flag::MEASURE).unwrap();
    let plan_ba: C2CPlan32 =
        C2CPlan::new(&[n], &mut b, &mut a, Sign::Forward, Flag::MEASURE).unwrap();

    for i in 0..n {
        a[i] = c32::new(1.0, 0.0);
    }

    plan_ab.c2c(&mut a, &mut b).unwrap();
    plan_ba.c2c(&mut b, &mut a).unwrap();
    for v in a.iter() {
        let ans = c32::new(n as f32, 0.0);
        let dif = (v - ans).norm();
        if dif > 1e-7 {
            panic!("Large difference: v={}, dif={}", v, dif);
        }
    }
}

/// Check cos transform
#[test]
fn c2c_cos_64() {
    let n = 32;
    let mut a = vec![c64::zero(); n];
    let mut b = vec![c64::zero(); n];
    let plan: C2CPlan64 = C2CPlan::new(&[n], &mut a, &mut b, Sign::Forward, Flag::MEASURE).unwrap();
    let pi = ::std::f64::consts::PI;
    for i in 0..n {
        a[i] = c64::new((2.0 * pi * i as f64 / n as f64).cos(), 0.0);
    }
    plan.c2c(&mut a, &mut b).unwrap();
    for (i, v) in b.iter().enumerate() {
        let ans = if i == 1 || i == n - 1 {
            0.5 * n as f64
        } else {
            0.0
        };
        let dif = (v - ans).norm();
        if dif > 1e-7 {
            panic!(
                "Large difference: v={}, ans={}, dif={}, i={}",
                v, ans, dif, i
            );
        }
    }
}

#[test]
fn c2c_cos_32() {
    let n = 32;
    let mut a = vec![c32::zero(); n];
    let mut b = vec![c32::zero(); n];
    let plan: C2CPlan32 = C2CPlan::new(&[n], &mut a, &mut b, Sign::Forward, Flag::MEASURE).unwrap();
    let pi = ::std::f32::consts::PI;
    for i in 0..n {
        a[i] = c32::new((2.0 * pi * i as f32 / n as f32).cos(), 0.0);
    }
    plan.c2c(&mut a, &mut b).unwrap();
    for (i, v) in b.iter().enumerate() {
        let ans = if i == 1 || i == n - 1 {
            0.5 * n as f32
        } else {
            0.0
        };
        let dif = (v - ans).norm();
        if dif > 1e-5 {
            panic!(
                "Large difference: v={}, ans={}, dif={}, i={}",
                v, ans, dif, i
            );
        }
    }
}
