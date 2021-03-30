//! Rusty types for manipulating FFTW

use bitflags::bitflags;
use ffi::fftw_complex;
use ffi::fftwf_complex;
use ffi::fftwl_complex;
use f128::f128;

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Complex32(fftwf_complex);

impl Add<Complex32> for Complex32{
    type Output = Complex32;
    fn add(self, rhs: Complex32) -> Self::Output {
        Complex32([self.0[0] + rhs.0[0], self.0[1] + rhs.0[1]])
    }
}

impl Zero for Complex32{
    fn zero() -> Self {
        Complex32([ 0f32, 0f32])
    }
    fn is_zero(&self) -> bool {
        self.0[0] == 0f32 && self.0[1] == 0f32
    }
}

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Complex64(fftw_complex);

impl Add<Complex64> for Complex64{
    type Output = Complex64;
    fn add(self, rhs: Complex64) -> Self::Output {
        Complex64([self.0[0] + rhs.0[0], self.0[1] + rhs.0[1]])
    }
}

impl Zero for Complex64{
    fn zero() -> Self {
        Complex64([ 0f64, 0f64])
    }
    fn is_zero(&self) -> bool {
        self.0[0] == 0f64 && self.0[1] == 0f64
    }
}

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Complex128(fftwl_complex);

impl Add<Complex128> for Complex128{
    type Output = Complex128;
    fn add(self, rhs: Complex128) -> Self::Output {
        Complex128([self.0[0] + rhs.0[0], self.0[1] + rhs.0[1]])
    }
}

impl Zero for Complex128{
    fn zero() -> Self {
        Complex128([f128::ZERO, f128::ZERO])
    }
    fn is_zero(&self) -> bool {
        self.0[0] == f128::ZERO && self.0[1] == f128::ZERO
    }
}

/// Expose the kinds of real-to-real transformations
pub use ffi::fftw_r2r_kind as R2RKind;
use num_traits::Zero;
use std::ops::Add;

/// Direction of Complex-to-Complex transformation
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Sign {
    Forward = -1,
    Backward = 1,
}

impl ::std::ops::Neg for Sign {
    type Output = Sign;
    fn neg(self) -> Self::Output {
        match self {
            Sign::Forward => Sign::Backward,
            Sign::Backward => Sign::Forward,
        }
    }
}

bitflags! {
    /// Flags for creating plans and wisdom
    ///
    /// This will be the most important part for fast FFT.
    /// You should see the [Words of Wisdom] in the original document
    ///
    /// [Words of Wisdom]: http://www.fftw.org/fftw3_doc/Words-of-Wisdom_002dSaving-Plans.html
    #[derive(Default)]
    pub struct Flag: u32 {
        const MEASURE = 0;
        const DESTROYINPUT = 1 ;
        const UNALIGNED = 1 << 1;
        const CONSERVEMEMORY = 1 << 2;
        const EXHAUSIVE = 1 << 3;
        const PRESERVEINPUT = 1 << 4;
        const PATIENT = 1 << 5;
        const ESTIMATE = 1 << 6;
        const WISDOWMONLY = 1 << 21;
    }
}
