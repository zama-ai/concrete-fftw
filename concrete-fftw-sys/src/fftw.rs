/* automatically generated by rust-bindgen 0.59.1 */

pub const FFTW_FORWARD: i32 = -1;
pub const FFTW_BACKWARD: u32 = 1;
pub const FFTW_NO_TIMELIMIT: f64 = -1.0;
pub const FFTW_MEASURE: u32 = 0;
pub const FFTW_DESTROY_INPUT: u32 = 1;
pub const FFTW_UNALIGNED: u32 = 2;
pub const FFTW_CONSERVE_MEMORY: u32 = 4;
pub const FFTW_EXHAUSTIVE: u32 = 8;
pub const FFTW_PRESERVE_INPUT: u32 = 16;
pub const FFTW_PATIENT: u32 = 32;
pub const FFTW_ESTIMATE: u32 = 64;
pub const FFTW_WISDOM_ONLY: u32 = 2097152;
pub const FFTW_ESTIMATE_PATIENT: u32 = 128;
pub const FFTW_BELIEVE_PCOST: u32 = 256;
pub const FFTW_NO_DFT_R2HC: u32 = 512;
pub const FFTW_NO_NONTHREADED: u32 = 1024;
pub const FFTW_NO_BUFFERING: u32 = 2048;
pub const FFTW_NO_INDIRECT_OP: u32 = 4096;
pub const FFTW_ALLOW_LARGE_GENERIC: u32 = 8192;
pub const FFTW_NO_RANK_SPLITS: u32 = 16384;
pub const FFTW_NO_VRANK_SPLITS: u32 = 32768;
pub const FFTW_NO_VRECURSE: u32 = 65536;
pub const FFTW_NO_SIMD: u32 = 131072;
pub const FFTW_NO_SLOW: u32 = 262144;
pub const FFTW_NO_FIXED_RADIX_LARGE_N: u32 = 524288;
pub const FFTW_ALLOW_PRUNING: u32 = 1048576;
pub type size_t = ::std::os::raw::c_ulong;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum fftw_r2r_kind_do_not_use_me {
    FFTW_R2HC = 0,
    FFTW_HC2R = 1,
    FFTW_DHT = 2,
    FFTW_REDFT00 = 3,
    FFTW_REDFT01 = 4,
    FFTW_REDFT10 = 5,
    FFTW_REDFT11 = 6,
    FFTW_RODFT00 = 7,
    FFTW_RODFT01 = 8,
    FFTW_RODFT10 = 9,
    FFTW_RODFT11 = 10,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct fftw_iodim_do_not_use_me {
    pub n: ::std::os::raw::c_int,
    pub is: ::std::os::raw::c_int,
    pub os: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_fftw_iodim_do_not_use_me() {
    assert_eq!(
        ::core::mem::size_of::<fftw_iodim_do_not_use_me>(),
        12usize,
        concat!("Size of: ", stringify!(fftw_iodim_do_not_use_me))
    );
    assert_eq!(
        ::core::mem::align_of::<fftw_iodim_do_not_use_me>(),
        4usize,
        concat!("Alignment of ", stringify!(fftw_iodim_do_not_use_me))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<fftw_iodim_do_not_use_me>())).n as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(fftw_iodim_do_not_use_me),
            "::",
            stringify!(n)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<fftw_iodim_do_not_use_me>())).is as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(fftw_iodim_do_not_use_me),
            "::",
            stringify!(is)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<fftw_iodim_do_not_use_me>())).os as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(fftw_iodim_do_not_use_me),
            "::",
            stringify!(os)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct fftw_iodim64_do_not_use_me {
    pub n: isize,
    pub is: isize,
    pub os: isize,
}
#[test]
fn bindgen_test_layout_fftw_iodim64_do_not_use_me() {
    assert_eq!(
        ::core::mem::size_of::<fftw_iodim64_do_not_use_me>(),
        24usize,
        concat!("Size of: ", stringify!(fftw_iodim64_do_not_use_me))
    );
    assert_eq!(
        ::core::mem::align_of::<fftw_iodim64_do_not_use_me>(),
        8usize,
        concat!("Alignment of ", stringify!(fftw_iodim64_do_not_use_me))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<fftw_iodim64_do_not_use_me>())).n as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(fftw_iodim64_do_not_use_me),
            "::",
            stringify!(n)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<fftw_iodim64_do_not_use_me>())).is as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(fftw_iodim64_do_not_use_me),
            "::",
            stringify!(is)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<fftw_iodim64_do_not_use_me>())).os as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(fftw_iodim64_do_not_use_me),
            "::",
            stringify!(os)
        )
    );
}
pub type fftw_write_char_func_do_not_use_me = ::core::option::Option<
    unsafe extern "C" fn(c: ::std::os::raw::c_char, arg1: *mut ::core::ffi::c_void),
>;
pub type fftw_read_char_func_do_not_use_me = ::core::option::Option<
    unsafe extern "C" fn(arg1: *mut ::core::ffi::c_void) -> ::std::os::raw::c_int,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fftw_plan_s {
    _unused: [u8; 0],
}
pub type fftw_plan = *mut fftw_plan_s;
pub type fftw_iodim = fftw_iodim_do_not_use_me;
pub type fftw_iodim64 = fftw_iodim64_do_not_use_me;
pub use self::fftw_r2r_kind_do_not_use_me as fftw_r2r_kind;
pub type fftw_write_char_func = fftw_write_char_func_do_not_use_me;
pub type fftw_read_char_func = fftw_read_char_func_do_not_use_me;
extern "C" {
    pub fn fftw_execute(p: fftw_plan);
}
extern "C" {
    pub fn fftw_plan_dft(
        rank: ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        in_: *mut fftw_complex,
        out: *mut fftw_complex,
        sign: ::std::os::raw::c_int,
        flags: ::std::os::raw::c_uint,
    ) -> fftw_plan;
}
extern "C" {
    pub fn fftw_plan_dft_1d(
        n: ::std::os::raw::c_int,
        in_: *mut fftw_complex,
        out: *mut fftw_complex,
        sign: ::std::os::raw::c_int,
        flags: ::std::os::raw::c_uint,
    ) -> fftw_plan;
}
extern "C" {
    pub fn fftw_plan_dft_2d(
        n0: ::std::os::raw::c_int,
        n1: ::std::os::raw::c_int,
        in_: *mut fftw_complex,
        out: *mut fftw_complex,
        sign: ::std::os::raw::c_int,
        flags: ::std::os::raw::c_uint,
    ) -> fftw_plan;
}
extern "C" {
    pub fn fftw_plan_dft_3d(
        n0: ::std::os::raw::c_int,
        n1: ::std::os::raw::c_int,
        n2: ::std::os::raw::c_int,
        in_: *mut fftw_complex,
        out: *mut fftw_complex,
        sign: ::std::os::raw::c_int,
        flags: ::std::os::raw::c_uint,
    ) -> fftw_plan;
}
extern "C" {
    pub fn fftw_plan_many_dft(
        rank: ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        howmany: ::std::os::raw::c_int,
        in_: *mut fftw_complex,
        inembed: *const ::std::os::raw::c_int,
        istride: ::std::os::raw::c_int,
        idist: ::std::os::raw::c_int,
        out: *mut fftw_complex,
        onembed: *const ::std::os::raw::c_int,
        ostride: ::std::os::raw::c_int,
        odist: ::std::os::raw::c_int,
        sign: ::std::os::raw::c_int,
        flags: ::std::os::raw::c_uint,
    ) -> fftw_plan;
}
extern "C" {
    pub fn fftw_plan_guru_dft(
        rank: ::std::os::raw::c_int,
        dims: *const fftw_iodim,
        howmany_rank: ::std::os::raw::c_int,
        howmany_dims: *const fftw_iodim,
        in_: *mut fftw_complex,
        out: *mut fftw_complex,
        sign: ::std::os::raw::c_int,
        flags: ::std::os::raw::c_uint,
    ) -> fftw_plan;
}
extern "C" {
    pub fn fftw_plan_guru_split_dft(
        rank: ::std::os::raw::c_int,
        dims: *const fftw_iodim,
        howmany_rank: ::std::os::raw::c_int,
        howmany_dims: *const fftw_iodim,
        ri: *mut f64,
        ii: *mut f64,
        ro: *mut f64,
        io: *mut f64,
        flags: ::std::os::raw::c_uint,
    ) -> fftw_plan;
}
extern "C" {
    pub fn fftw_plan_guru64_dft(
        rank: ::std::os::raw::c_int,
        dims: *const fftw_iodim64,
        howmany_rank: ::std::os::raw::c_int,
        howmany_dims: *const fftw_iodim64,
        in_: *mut fftw_complex,
        out: *mut fftw_complex,
        sign: ::std::os::raw::c_int,
        flags: ::std::os::raw::c_uint,
    ) -> fftw_plan;
}
extern "C" {
    pub fn fftw_plan_guru64_split_dft(
        rank: ::std::os::raw::c_int,
        dims: *const fftw_iodim64,
        howmany_rank: ::std::os::raw::c_int,
        howmany_dims: *const fftw_iodim64,
        ri: *mut f64,
        ii: *mut f64,
        ro: *mut f64,
        io: *mut f64,
        flags: ::std::os::raw::c_uint,
    ) -> fftw_plan;
}
extern "C" {
    pub fn fftw_execute_dft(p: fftw_plan, in_: *mut fftw_complex, out: *mut fftw_complex);
}
extern "C" {
    pub fn fftw_execute_split_dft(
        p: fftw_plan,
        ri: *mut f64,
        ii: *mut f64,
        ro: *mut f64,
        io: *mut f64,
    );
}
extern "C" {
    pub fn fftw_plan_many_dft_r2c(
        rank: ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        howmany: ::std::os::raw::c_int,
        in_: *mut f64,
        inembed: *const ::std::os::raw::c_int,
        istride: ::std::os::raw::c_int,
        idist: ::std::os::raw::c_int,
        out: *mut fftw_complex,
        onembed: *const ::std::os::raw::c_int,
        ostride: ::std::os::raw::c_int,
        odist: ::std::os::raw::c_int,
        flags: ::std::os::raw::c_uint,
    ) -> fftw_plan;
}
extern "C" {
    pub fn fftw_plan_dft_r2c(
        rank: ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        in_: *mut f64,
        out: *mut fftw_complex,
        flags: ::std::os::raw::c_uint,
    ) -> fftw_plan;
}
extern "C" {
    pub fn fftw_plan_dft_r2c_1d(
        n: ::std::os::raw::c_int,
        in_: *mut f64,
        out: *mut fftw_complex,
        flags: ::std::os::raw::c_uint,
    ) -> fftw_plan;
}
extern "C" {
    pub fn fftw_plan_dft_r2c_2d(
        n0: ::std::os::raw::c_int,
        n1: ::std::os::raw::c_int,
        in_: *mut f64,
        out: *mut fftw_complex,
        flags: ::std::os::raw::c_uint,
    ) -> fftw_plan;
}
extern "C" {
    pub fn fftw_plan_dft_r2c_3d(
        n0: ::std::os::raw::c_int,
        n1: ::std::os::raw::c_int,
        n2: ::std::os::raw::c_int,
        in_: *mut f64,
        out: *mut fftw_complex,
        flags: ::std::os::raw::c_uint,
    ) -> fftw_plan;
}
extern "C" {
    pub fn fftw_plan_many_dft_c2r(
        rank: ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        howmany: ::std::os::raw::c_int,
        in_: *mut fftw_complex,
        inembed: *const ::std::os::raw::c_int,
        istride: ::std::os::raw::c_int,
        idist: ::std::os::raw::c_int,
        out: *mut f64,
        onembed: *const ::std::os::raw::c_int,
        ostride: ::std::os::raw::c_int,
        odist: ::std::os::raw::c_int,
        flags: ::std::os::raw::c_uint,
    ) -> fftw_plan;
}
extern "C" {
    pub fn fftw_plan_dft_c2r(
        rank: ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        in_: *mut fftw_complex,
        out: *mut f64,
        flags: ::std::os::raw::c_uint,
    ) -> fftw_plan;
}
extern "C" {
    pub fn fftw_plan_dft_c2r_1d(
        n: ::std::os::raw::c_int,
        in_: *mut fftw_complex,
        out: *mut f64,
        flags: ::std::os::raw::c_uint,
    ) -> fftw_plan;
}
extern "C" {
    pub fn fftw_plan_dft_c2r_2d(
        n0: ::std::os::raw::c_int,
        n1: ::std::os::raw::c_int,
        in_: *mut fftw_complex,
        out: *mut f64,
        flags: ::std::os::raw::c_uint,
    ) -> fftw_plan;
}
extern "C" {
    pub fn fftw_plan_dft_c2r_3d(
        n0: ::std::os::raw::c_int,
        n1: ::std::os::raw::c_int,
        n2: ::std::os::raw::c_int,
        in_: *mut fftw_complex,
        out: *mut f64,
        flags: ::std::os::raw::c_uint,
    ) -> fftw_plan;
}
extern "C" {
    pub fn fftw_plan_guru_dft_r2c(
        rank: ::std::os::raw::c_int,
        dims: *const fftw_iodim,
        howmany_rank: ::std::os::raw::c_int,
        howmany_dims: *const fftw_iodim,
        in_: *mut f64,
        out: *mut fftw_complex,
        flags: ::std::os::raw::c_uint,
    ) -> fftw_plan;
}
extern "C" {
    pub fn fftw_plan_guru_dft_c2r(
        rank: ::std::os::raw::c_int,
        dims: *const fftw_iodim,
        howmany_rank: ::std::os::raw::c_int,
        howmany_dims: *const fftw_iodim,
        in_: *mut fftw_complex,
        out: *mut f64,
        flags: ::std::os::raw::c_uint,
    ) -> fftw_plan;
}
extern "C" {
    pub fn fftw_plan_guru_split_dft_r2c(
        rank: ::std::os::raw::c_int,
        dims: *const fftw_iodim,
        howmany_rank: ::std::os::raw::c_int,
        howmany_dims: *const fftw_iodim,
        in_: *mut f64,
        ro: *mut f64,
        io: *mut f64,
        flags: ::std::os::raw::c_uint,
    ) -> fftw_plan;
}
extern "C" {
    pub fn fftw_plan_guru_split_dft_c2r(
        rank: ::std::os::raw::c_int,
        dims: *const fftw_iodim,
        howmany_rank: ::std::os::raw::c_int,
        howmany_dims: *const fftw_iodim,
        ri: *mut f64,
        ii: *mut f64,
        out: *mut f64,
        flags: ::std::os::raw::c_uint,
    ) -> fftw_plan;
}
extern "C" {
    pub fn fftw_plan_guru64_dft_r2c(
        rank: ::std::os::raw::c_int,
        dims: *const fftw_iodim64,
        howmany_rank: ::std::os::raw::c_int,
        howmany_dims: *const fftw_iodim64,
        in_: *mut f64,
        out: *mut fftw_complex,
        flags: ::std::os::raw::c_uint,
    ) -> fftw_plan;
}
extern "C" {
    pub fn fftw_plan_guru64_dft_c2r(
        rank: ::std::os::raw::c_int,
        dims: *const fftw_iodim64,
        howmany_rank: ::std::os::raw::c_int,
        howmany_dims: *const fftw_iodim64,
        in_: *mut fftw_complex,
        out: *mut f64,
        flags: ::std::os::raw::c_uint,
    ) -> fftw_plan;
}
extern "C" {
    pub fn fftw_plan_guru64_split_dft_r2c(
        rank: ::std::os::raw::c_int,
        dims: *const fftw_iodim64,
        howmany_rank: ::std::os::raw::c_int,
        howmany_dims: *const fftw_iodim64,
        in_: *mut f64,
        ro: *mut f64,
        io: *mut f64,
        flags: ::std::os::raw::c_uint,
    ) -> fftw_plan;
}
extern "C" {
    pub fn fftw_plan_guru64_split_dft_c2r(
        rank: ::std::os::raw::c_int,
        dims: *const fftw_iodim64,
        howmany_rank: ::std::os::raw::c_int,
        howmany_dims: *const fftw_iodim64,
        ri: *mut f64,
        ii: *mut f64,
        out: *mut f64,
        flags: ::std::os::raw::c_uint,
    ) -> fftw_plan;
}
extern "C" {
    pub fn fftw_execute_dft_r2c(p: fftw_plan, in_: *mut f64, out: *mut fftw_complex);
}
extern "C" {
    pub fn fftw_execute_dft_c2r(p: fftw_plan, in_: *mut fftw_complex, out: *mut f64);
}
extern "C" {
    pub fn fftw_execute_split_dft_r2c(p: fftw_plan, in_: *mut f64, ro: *mut f64, io: *mut f64);
}
extern "C" {
    pub fn fftw_execute_split_dft_c2r(p: fftw_plan, ri: *mut f64, ii: *mut f64, out: *mut f64);
}
extern "C" {
    pub fn fftw_plan_many_r2r(
        rank: ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        howmany: ::std::os::raw::c_int,
        in_: *mut f64,
        inembed: *const ::std::os::raw::c_int,
        istride: ::std::os::raw::c_int,
        idist: ::std::os::raw::c_int,
        out: *mut f64,
        onembed: *const ::std::os::raw::c_int,
        ostride: ::std::os::raw::c_int,
        odist: ::std::os::raw::c_int,
        kind: *const fftw_r2r_kind,
        flags: ::std::os::raw::c_uint,
    ) -> fftw_plan;
}
extern "C" {
    pub fn fftw_plan_r2r(
        rank: ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        in_: *mut f64,
        out: *mut f64,
        kind: *const fftw_r2r_kind,
        flags: ::std::os::raw::c_uint,
    ) -> fftw_plan;
}
extern "C" {
    pub fn fftw_plan_r2r_1d(
        n: ::std::os::raw::c_int,
        in_: *mut f64,
        out: *mut f64,
        kind: fftw_r2r_kind,
        flags: ::std::os::raw::c_uint,
    ) -> fftw_plan;
}
extern "C" {
    pub fn fftw_plan_r2r_2d(
        n0: ::std::os::raw::c_int,
        n1: ::std::os::raw::c_int,
        in_: *mut f64,
        out: *mut f64,
        kind0: fftw_r2r_kind,
        kind1: fftw_r2r_kind,
        flags: ::std::os::raw::c_uint,
    ) -> fftw_plan;
}
extern "C" {
    pub fn fftw_plan_r2r_3d(
        n0: ::std::os::raw::c_int,
        n1: ::std::os::raw::c_int,
        n2: ::std::os::raw::c_int,
        in_: *mut f64,
        out: *mut f64,
        kind0: fftw_r2r_kind,
        kind1: fftw_r2r_kind,
        kind2: fftw_r2r_kind,
        flags: ::std::os::raw::c_uint,
    ) -> fftw_plan;
}
extern "C" {
    pub fn fftw_plan_guru_r2r(
        rank: ::std::os::raw::c_int,
        dims: *const fftw_iodim,
        howmany_rank: ::std::os::raw::c_int,
        howmany_dims: *const fftw_iodim,
        in_: *mut f64,
        out: *mut f64,
        kind: *const fftw_r2r_kind,
        flags: ::std::os::raw::c_uint,
    ) -> fftw_plan;
}
extern "C" {
    pub fn fftw_plan_guru64_r2r(
        rank: ::std::os::raw::c_int,
        dims: *const fftw_iodim64,
        howmany_rank: ::std::os::raw::c_int,
        howmany_dims: *const fftw_iodim64,
        in_: *mut f64,
        out: *mut f64,
        kind: *const fftw_r2r_kind,
        flags: ::std::os::raw::c_uint,
    ) -> fftw_plan;
}
extern "C" {
    pub fn fftw_execute_r2r(p: fftw_plan, in_: *mut f64, out: *mut f64);
}
extern "C" {
    pub fn fftw_destroy_plan(p: fftw_plan);
}
extern "C" {
    pub fn fftw_cleanup();
}
extern "C" {
    pub fn fftw_set_timelimit(t: f64);
}
extern "C" {
    pub fn fftw_plan_with_nthreads(nthreads: ::std::os::raw::c_int);
}
extern "C" {
    pub fn fftw_init_threads() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fftw_cleanup_threads();
}
extern "C" {
    pub fn fftw_make_planner_thread_safe();
}
extern "C" {
    pub fn fftw_print_plan(p: fftw_plan);
}
extern "C" {
    pub fn fftw_sprint_plan(p: fftw_plan) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn fftw_malloc(n: size_t) -> *mut ::core::ffi::c_void;
}
extern "C" {
    pub fn fftw_alloc_real(n: size_t) -> *mut f64;
}
extern "C" {
    pub fn fftw_alloc_complex(n: size_t) -> *mut fftw_complex;
}
extern "C" {
    pub fn fftw_free(p: *mut ::core::ffi::c_void);
}
extern "C" {
    pub fn fftw_flops(p: fftw_plan, add: *mut f64, mul: *mut f64, fmas: *mut f64);
}
extern "C" {
    pub fn fftw_estimate_cost(p: fftw_plan) -> f64;
}
extern "C" {
    pub fn fftw_cost(p: fftw_plan) -> f64;
}
extern "C" {
    pub fn fftw_alignment_of(p: *mut f64) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fftwf_plan_s {
    _unused: [u8; 0],
}
pub type fftwf_plan = *mut fftwf_plan_s;
pub type fftwf_iodim = fftw_iodim_do_not_use_me;
pub type fftwf_iodim64 = fftw_iodim64_do_not_use_me;
pub use self::fftw_r2r_kind_do_not_use_me as fftwf_r2r_kind;
pub type fftwf_write_char_func = fftw_write_char_func_do_not_use_me;
pub type fftwf_read_char_func = fftw_read_char_func_do_not_use_me;
extern "C" {
    pub fn fftwf_execute(p: fftwf_plan);
}
extern "C" {
    pub fn fftwf_plan_dft(
        rank: ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        in_: *mut fftwf_complex,
        out: *mut fftwf_complex,
        sign: ::std::os::raw::c_int,
        flags: ::std::os::raw::c_uint,
    ) -> fftwf_plan;
}
extern "C" {
    pub fn fftwf_plan_dft_1d(
        n: ::std::os::raw::c_int,
        in_: *mut fftwf_complex,
        out: *mut fftwf_complex,
        sign: ::std::os::raw::c_int,
        flags: ::std::os::raw::c_uint,
    ) -> fftwf_plan;
}
extern "C" {
    pub fn fftwf_plan_dft_2d(
        n0: ::std::os::raw::c_int,
        n1: ::std::os::raw::c_int,
        in_: *mut fftwf_complex,
        out: *mut fftwf_complex,
        sign: ::std::os::raw::c_int,
        flags: ::std::os::raw::c_uint,
    ) -> fftwf_plan;
}
extern "C" {
    pub fn fftwf_plan_dft_3d(
        n0: ::std::os::raw::c_int,
        n1: ::std::os::raw::c_int,
        n2: ::std::os::raw::c_int,
        in_: *mut fftwf_complex,
        out: *mut fftwf_complex,
        sign: ::std::os::raw::c_int,
        flags: ::std::os::raw::c_uint,
    ) -> fftwf_plan;
}
extern "C" {
    pub fn fftwf_plan_many_dft(
        rank: ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        howmany: ::std::os::raw::c_int,
        in_: *mut fftwf_complex,
        inembed: *const ::std::os::raw::c_int,
        istride: ::std::os::raw::c_int,
        idist: ::std::os::raw::c_int,
        out: *mut fftwf_complex,
        onembed: *const ::std::os::raw::c_int,
        ostride: ::std::os::raw::c_int,
        odist: ::std::os::raw::c_int,
        sign: ::std::os::raw::c_int,
        flags: ::std::os::raw::c_uint,
    ) -> fftwf_plan;
}
extern "C" {
    pub fn fftwf_plan_guru_dft(
        rank: ::std::os::raw::c_int,
        dims: *const fftwf_iodim,
        howmany_rank: ::std::os::raw::c_int,
        howmany_dims: *const fftwf_iodim,
        in_: *mut fftwf_complex,
        out: *mut fftwf_complex,
        sign: ::std::os::raw::c_int,
        flags: ::std::os::raw::c_uint,
    ) -> fftwf_plan;
}
extern "C" {
    pub fn fftwf_plan_guru_split_dft(
        rank: ::std::os::raw::c_int,
        dims: *const fftwf_iodim,
        howmany_rank: ::std::os::raw::c_int,
        howmany_dims: *const fftwf_iodim,
        ri: *mut f32,
        ii: *mut f32,
        ro: *mut f32,
        io: *mut f32,
        flags: ::std::os::raw::c_uint,
    ) -> fftwf_plan;
}
extern "C" {
    pub fn fftwf_plan_guru64_dft(
        rank: ::std::os::raw::c_int,
        dims: *const fftwf_iodim64,
        howmany_rank: ::std::os::raw::c_int,
        howmany_dims: *const fftwf_iodim64,
        in_: *mut fftwf_complex,
        out: *mut fftwf_complex,
        sign: ::std::os::raw::c_int,
        flags: ::std::os::raw::c_uint,
    ) -> fftwf_plan;
}
extern "C" {
    pub fn fftwf_plan_guru64_split_dft(
        rank: ::std::os::raw::c_int,
        dims: *const fftwf_iodim64,
        howmany_rank: ::std::os::raw::c_int,
        howmany_dims: *const fftwf_iodim64,
        ri: *mut f32,
        ii: *mut f32,
        ro: *mut f32,
        io: *mut f32,
        flags: ::std::os::raw::c_uint,
    ) -> fftwf_plan;
}
extern "C" {
    pub fn fftwf_execute_dft(p: fftwf_plan, in_: *mut fftwf_complex, out: *mut fftwf_complex);
}
extern "C" {
    pub fn fftwf_execute_split_dft(
        p: fftwf_plan,
        ri: *mut f32,
        ii: *mut f32,
        ro: *mut f32,
        io: *mut f32,
    );
}
extern "C" {
    pub fn fftwf_plan_many_dft_r2c(
        rank: ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        howmany: ::std::os::raw::c_int,
        in_: *mut f32,
        inembed: *const ::std::os::raw::c_int,
        istride: ::std::os::raw::c_int,
        idist: ::std::os::raw::c_int,
        out: *mut fftwf_complex,
        onembed: *const ::std::os::raw::c_int,
        ostride: ::std::os::raw::c_int,
        odist: ::std::os::raw::c_int,
        flags: ::std::os::raw::c_uint,
    ) -> fftwf_plan;
}
extern "C" {
    pub fn fftwf_plan_dft_r2c(
        rank: ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        in_: *mut f32,
        out: *mut fftwf_complex,
        flags: ::std::os::raw::c_uint,
    ) -> fftwf_plan;
}
extern "C" {
    pub fn fftwf_plan_dft_r2c_1d(
        n: ::std::os::raw::c_int,
        in_: *mut f32,
        out: *mut fftwf_complex,
        flags: ::std::os::raw::c_uint,
    ) -> fftwf_plan;
}
extern "C" {
    pub fn fftwf_plan_dft_r2c_2d(
        n0: ::std::os::raw::c_int,
        n1: ::std::os::raw::c_int,
        in_: *mut f32,
        out: *mut fftwf_complex,
        flags: ::std::os::raw::c_uint,
    ) -> fftwf_plan;
}
extern "C" {
    pub fn fftwf_plan_dft_r2c_3d(
        n0: ::std::os::raw::c_int,
        n1: ::std::os::raw::c_int,
        n2: ::std::os::raw::c_int,
        in_: *mut f32,
        out: *mut fftwf_complex,
        flags: ::std::os::raw::c_uint,
    ) -> fftwf_plan;
}
extern "C" {
    pub fn fftwf_plan_many_dft_c2r(
        rank: ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        howmany: ::std::os::raw::c_int,
        in_: *mut fftwf_complex,
        inembed: *const ::std::os::raw::c_int,
        istride: ::std::os::raw::c_int,
        idist: ::std::os::raw::c_int,
        out: *mut f32,
        onembed: *const ::std::os::raw::c_int,
        ostride: ::std::os::raw::c_int,
        odist: ::std::os::raw::c_int,
        flags: ::std::os::raw::c_uint,
    ) -> fftwf_plan;
}
extern "C" {
    pub fn fftwf_plan_dft_c2r(
        rank: ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        in_: *mut fftwf_complex,
        out: *mut f32,
        flags: ::std::os::raw::c_uint,
    ) -> fftwf_plan;
}
extern "C" {
    pub fn fftwf_plan_dft_c2r_1d(
        n: ::std::os::raw::c_int,
        in_: *mut fftwf_complex,
        out: *mut f32,
        flags: ::std::os::raw::c_uint,
    ) -> fftwf_plan;
}
extern "C" {
    pub fn fftwf_plan_dft_c2r_2d(
        n0: ::std::os::raw::c_int,
        n1: ::std::os::raw::c_int,
        in_: *mut fftwf_complex,
        out: *mut f32,
        flags: ::std::os::raw::c_uint,
    ) -> fftwf_plan;
}
extern "C" {
    pub fn fftwf_plan_dft_c2r_3d(
        n0: ::std::os::raw::c_int,
        n1: ::std::os::raw::c_int,
        n2: ::std::os::raw::c_int,
        in_: *mut fftwf_complex,
        out: *mut f32,
        flags: ::std::os::raw::c_uint,
    ) -> fftwf_plan;
}
extern "C" {
    pub fn fftwf_plan_guru_dft_r2c(
        rank: ::std::os::raw::c_int,
        dims: *const fftwf_iodim,
        howmany_rank: ::std::os::raw::c_int,
        howmany_dims: *const fftwf_iodim,
        in_: *mut f32,
        out: *mut fftwf_complex,
        flags: ::std::os::raw::c_uint,
    ) -> fftwf_plan;
}
extern "C" {
    pub fn fftwf_plan_guru_dft_c2r(
        rank: ::std::os::raw::c_int,
        dims: *const fftwf_iodim,
        howmany_rank: ::std::os::raw::c_int,
        howmany_dims: *const fftwf_iodim,
        in_: *mut fftwf_complex,
        out: *mut f32,
        flags: ::std::os::raw::c_uint,
    ) -> fftwf_plan;
}
extern "C" {
    pub fn fftwf_plan_guru_split_dft_r2c(
        rank: ::std::os::raw::c_int,
        dims: *const fftwf_iodim,
        howmany_rank: ::std::os::raw::c_int,
        howmany_dims: *const fftwf_iodim,
        in_: *mut f32,
        ro: *mut f32,
        io: *mut f32,
        flags: ::std::os::raw::c_uint,
    ) -> fftwf_plan;
}
extern "C" {
    pub fn fftwf_plan_guru_split_dft_c2r(
        rank: ::std::os::raw::c_int,
        dims: *const fftwf_iodim,
        howmany_rank: ::std::os::raw::c_int,
        howmany_dims: *const fftwf_iodim,
        ri: *mut f32,
        ii: *mut f32,
        out: *mut f32,
        flags: ::std::os::raw::c_uint,
    ) -> fftwf_plan;
}
extern "C" {
    pub fn fftwf_plan_guru64_dft_r2c(
        rank: ::std::os::raw::c_int,
        dims: *const fftwf_iodim64,
        howmany_rank: ::std::os::raw::c_int,
        howmany_dims: *const fftwf_iodim64,
        in_: *mut f32,
        out: *mut fftwf_complex,
        flags: ::std::os::raw::c_uint,
    ) -> fftwf_plan;
}
extern "C" {
    pub fn fftwf_plan_guru64_dft_c2r(
        rank: ::std::os::raw::c_int,
        dims: *const fftwf_iodim64,
        howmany_rank: ::std::os::raw::c_int,
        howmany_dims: *const fftwf_iodim64,
        in_: *mut fftwf_complex,
        out: *mut f32,
        flags: ::std::os::raw::c_uint,
    ) -> fftwf_plan;
}
extern "C" {
    pub fn fftwf_plan_guru64_split_dft_r2c(
        rank: ::std::os::raw::c_int,
        dims: *const fftwf_iodim64,
        howmany_rank: ::std::os::raw::c_int,
        howmany_dims: *const fftwf_iodim64,
        in_: *mut f32,
        ro: *mut f32,
        io: *mut f32,
        flags: ::std::os::raw::c_uint,
    ) -> fftwf_plan;
}
extern "C" {
    pub fn fftwf_plan_guru64_split_dft_c2r(
        rank: ::std::os::raw::c_int,
        dims: *const fftwf_iodim64,
        howmany_rank: ::std::os::raw::c_int,
        howmany_dims: *const fftwf_iodim64,
        ri: *mut f32,
        ii: *mut f32,
        out: *mut f32,
        flags: ::std::os::raw::c_uint,
    ) -> fftwf_plan;
}
extern "C" {
    pub fn fftwf_execute_dft_r2c(p: fftwf_plan, in_: *mut f32, out: *mut fftwf_complex);
}
extern "C" {
    pub fn fftwf_execute_dft_c2r(p: fftwf_plan, in_: *mut fftwf_complex, out: *mut f32);
}
extern "C" {
    pub fn fftwf_execute_split_dft_r2c(p: fftwf_plan, in_: *mut f32, ro: *mut f32, io: *mut f32);
}
extern "C" {
    pub fn fftwf_execute_split_dft_c2r(p: fftwf_plan, ri: *mut f32, ii: *mut f32, out: *mut f32);
}
extern "C" {
    pub fn fftwf_plan_many_r2r(
        rank: ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        howmany: ::std::os::raw::c_int,
        in_: *mut f32,
        inembed: *const ::std::os::raw::c_int,
        istride: ::std::os::raw::c_int,
        idist: ::std::os::raw::c_int,
        out: *mut f32,
        onembed: *const ::std::os::raw::c_int,
        ostride: ::std::os::raw::c_int,
        odist: ::std::os::raw::c_int,
        kind: *const fftwf_r2r_kind,
        flags: ::std::os::raw::c_uint,
    ) -> fftwf_plan;
}
extern "C" {
    pub fn fftwf_plan_r2r(
        rank: ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        in_: *mut f32,
        out: *mut f32,
        kind: *const fftwf_r2r_kind,
        flags: ::std::os::raw::c_uint,
    ) -> fftwf_plan;
}
extern "C" {
    pub fn fftwf_plan_r2r_1d(
        n: ::std::os::raw::c_int,
        in_: *mut f32,
        out: *mut f32,
        kind: fftwf_r2r_kind,
        flags: ::std::os::raw::c_uint,
    ) -> fftwf_plan;
}
extern "C" {
    pub fn fftwf_plan_r2r_2d(
        n0: ::std::os::raw::c_int,
        n1: ::std::os::raw::c_int,
        in_: *mut f32,
        out: *mut f32,
        kind0: fftwf_r2r_kind,
        kind1: fftwf_r2r_kind,
        flags: ::std::os::raw::c_uint,
    ) -> fftwf_plan;
}
extern "C" {
    pub fn fftwf_plan_r2r_3d(
        n0: ::std::os::raw::c_int,
        n1: ::std::os::raw::c_int,
        n2: ::std::os::raw::c_int,
        in_: *mut f32,
        out: *mut f32,
        kind0: fftwf_r2r_kind,
        kind1: fftwf_r2r_kind,
        kind2: fftwf_r2r_kind,
        flags: ::std::os::raw::c_uint,
    ) -> fftwf_plan;
}
extern "C" {
    pub fn fftwf_plan_guru_r2r(
        rank: ::std::os::raw::c_int,
        dims: *const fftwf_iodim,
        howmany_rank: ::std::os::raw::c_int,
        howmany_dims: *const fftwf_iodim,
        in_: *mut f32,
        out: *mut f32,
        kind: *const fftwf_r2r_kind,
        flags: ::std::os::raw::c_uint,
    ) -> fftwf_plan;
}
extern "C" {
    pub fn fftwf_plan_guru64_r2r(
        rank: ::std::os::raw::c_int,
        dims: *const fftwf_iodim64,
        howmany_rank: ::std::os::raw::c_int,
        howmany_dims: *const fftwf_iodim64,
        in_: *mut f32,
        out: *mut f32,
        kind: *const fftwf_r2r_kind,
        flags: ::std::os::raw::c_uint,
    ) -> fftwf_plan;
}
extern "C" {
    pub fn fftwf_execute_r2r(p: fftwf_plan, in_: *mut f32, out: *mut f32);
}
extern "C" {
    pub fn fftwf_destroy_plan(p: fftwf_plan);
}
extern "C" {
    pub fn fftwf_cleanup();
}
extern "C" {
    pub fn fftwf_set_timelimit(t: f64);
}
extern "C" {
    pub fn fftwf_plan_with_nthreads(nthreads: ::std::os::raw::c_int);
}
extern "C" {
    pub fn fftwf_init_threads() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fftwf_cleanup_threads();
}
extern "C" {
    pub fn fftwf_make_planner_thread_safe();
}
extern "C" {
    pub fn fftwf_print_plan(p: fftwf_plan);
}
extern "C" {
    pub fn fftwf_sprint_plan(p: fftwf_plan) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn fftwf_malloc(n: size_t) -> *mut ::core::ffi::c_void;
}
extern "C" {
    pub fn fftwf_alloc_real(n: size_t) -> *mut f32;
}
extern "C" {
    pub fn fftwf_alloc_complex(n: size_t) -> *mut fftwf_complex;
}
extern "C" {
    pub fn fftwf_free(p: *mut ::core::ffi::c_void);
}
extern "C" {
    pub fn fftwf_flops(p: fftwf_plan, add: *mut f64, mul: *mut f64, fmas: *mut f64);
}
extern "C" {
    pub fn fftwf_estimate_cost(p: fftwf_plan) -> f64;
}
extern "C" {
    pub fn fftwf_cost(p: fftwf_plan) -> f64;
}
extern "C" {
    pub fn fftwf_alignment_of(p: *mut f32) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fftwl_plan_s {
    _unused: [u8; 0],
}
pub type fftwl_plan = *mut fftwl_plan_s;
pub type fftwl_iodim = fftw_iodim_do_not_use_me;
pub type fftwl_iodim64 = fftw_iodim64_do_not_use_me;
pub use self::fftw_r2r_kind_do_not_use_me as fftwl_r2r_kind;
pub type fftwl_write_char_func = fftw_write_char_func_do_not_use_me;
pub type fftwl_read_char_func = fftw_read_char_func_do_not_use_me;