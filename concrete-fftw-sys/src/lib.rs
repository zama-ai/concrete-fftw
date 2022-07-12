#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]

#[cfg(feature = "quad")]
use f128::f128;
use num_complex::Complex;

#[link(name = "fftw3")]
extern "C" {}

#[link(name = "fftw3f")]
extern "C" {}

#[cfg(target_os = "macos")]
type __darwin_size_t = std::os::raw::c_ulong;
#[cfg(target_os = "macos")]
type __darwin_off_t = std::os::raw::c_longlong;

pub type fftw_complex = Complex<f64>;
pub type fftwf_complex = Complex<f32>;

include!("fftw.rs");
