#[cfg_attr(feature = "system", link(name = "fftw3q"))]
extern "C" {}

pub type fftwl_complex = [f128::f128; 2usize];

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));



