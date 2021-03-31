use num_complex::Complex;
use f128::f128;

#[link(name = "fftw3")]
extern "C" {}

pub type fftw_complex = Complex<f64>;
pub type fftwf_complex = Complex<f32>;
pub type fftwl_complex = Complex<f128>;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));