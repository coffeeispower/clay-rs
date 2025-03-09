/* automatically generated by rust-bindgen 0.71.1 */

#[derive(PartialEq, Copy, Clone, Hash, Debug, Default)]
#[repr(C)]
pub struct __BindgenComplex<T> {
    pub re: T,
    pub im: T,
}
#[derive(PartialEq, Copy, Clone, Hash, Debug, Default)]
#[repr(transparent)]
pub struct __BindgenFloat16(pub u16);
pub const _STDINT_H: u32 = 1;
pub const _FEATURES_H: u32 = 1;
pub const _DEFAULT_SOURCE: u32 = 1;
pub const __GLIBC_USE_ISOC2Y: u32 = 0;
pub const __GLIBC_USE_ISOC23: u32 = 0;
pub const __USE_ISOC11: u32 = 1;
pub const __USE_ISOC99: u32 = 1;
pub const __USE_ISOC95: u32 = 1;
pub const __USE_POSIX_IMPLICITLY: u32 = 1;
pub const _POSIX_SOURCE: u32 = 1;
pub const _POSIX_C_SOURCE: u32 = 200809;
pub const __USE_POSIX: u32 = 1;
pub const __USE_POSIX2: u32 = 1;
pub const __USE_POSIX199309: u32 = 1;
pub const __USE_POSIX199506: u32 = 1;
pub const __USE_XOPEN2K: u32 = 1;
pub const __USE_XOPEN2K8: u32 = 1;
pub const _ATFILE_SOURCE: u32 = 1;
pub const __WORDSIZE: u32 = 64;
pub const __WORDSIZE_TIME64_COMPAT32: u32 = 1;
pub const __SYSCALL_WORDSIZE: u32 = 64;
pub const __TIMESIZE: u32 = 64;
pub const __USE_TIME_BITS64: u32 = 1;
pub const __USE_MISC: u32 = 1;
pub const __USE_ATFILE: u32 = 1;
pub const __USE_FORTIFY_LEVEL: u32 = 0;
pub const __GLIBC_USE_DEPRECATED_GETS: u32 = 0;
pub const __GLIBC_USE_DEPRECATED_SCANF: u32 = 0;
pub const __GLIBC_USE_C23_STRTOL: u32 = 0;
pub const _STDC_PREDEF_H: u32 = 1;
pub const __STDC_IEC_559__: u32 = 1;
pub const __STDC_IEC_60559_BFP__: u32 = 201404;
pub const __STDC_IEC_559_COMPLEX__: u32 = 1;
pub const __STDC_IEC_60559_COMPLEX__: u32 = 201404;
pub const __STDC_ISO_10646__: u32 = 201706;
pub const __GNU_LIBRARY__: u32 = 6;
pub const __GLIBC__: u32 = 2;
pub const __GLIBC_MINOR__: u32 = 41;
pub const _SYS_CDEFS_H: u32 = 1;
pub const __glibc_c99_flexarr_available: u32 = 1;
pub const __LDOUBLE_REDIRECTS_TO_FLOAT128_ABI: u32 = 0;
pub const __HAVE_GENERIC_SELECTION: u32 = 1;
pub const __GLIBC_USE_LIB_EXT2: u32 = 0;
pub const __GLIBC_USE_IEC_60559_BFP_EXT: u32 = 0;
pub const __GLIBC_USE_IEC_60559_BFP_EXT_C23: u32 = 0;
pub const __GLIBC_USE_IEC_60559_EXT: u32 = 0;
pub const __GLIBC_USE_IEC_60559_FUNCS_EXT: u32 = 0;
pub const __GLIBC_USE_IEC_60559_FUNCS_EXT_C23: u32 = 0;
pub const __GLIBC_USE_IEC_60559_TYPES_EXT: u32 = 0;
pub const _BITS_TYPES_H: u32 = 1;
pub const _BITS_TYPESIZES_H: u32 = 1;
pub const __OFF_T_MATCHES_OFF64_T: u32 = 1;
pub const __INO_T_MATCHES_INO64_T: u32 = 1;
pub const __RLIM_T_MATCHES_RLIM64_T: u32 = 1;
pub const __STATFS_MATCHES_STATFS64: u32 = 1;
pub const __KERNEL_OLD_TIMEVAL_MATCHES_TIMEVAL64: u32 = 1;
pub const __FD_SETSIZE: u32 = 1024;
pub const _BITS_TIME64_H: u32 = 1;
pub const _BITS_WCHAR_H: u32 = 1;
pub const _BITS_STDINT_INTN_H: u32 = 1;
pub const _BITS_STDINT_UINTN_H: u32 = 1;
pub const _BITS_STDINT_LEAST_H: u32 = 1;
pub const INT8_MIN: i32 = -128;
pub const INT16_MIN: i32 = -32768;
pub const INT32_MIN: i32 = -2147483648;
pub const INT8_MAX: u32 = 127;
pub const INT16_MAX: u32 = 32767;
pub const INT32_MAX: u32 = 2147483647;
pub const UINT8_MAX: u32 = 255;
pub const UINT16_MAX: u32 = 65535;
pub const UINT32_MAX: u32 = 4294967295;
pub const INT_LEAST8_MIN: i32 = -128;
pub const INT_LEAST16_MIN: i32 = -32768;
pub const INT_LEAST32_MIN: i32 = -2147483648;
pub const INT_LEAST8_MAX: u32 = 127;
pub const INT_LEAST16_MAX: u32 = 32767;
pub const INT_LEAST32_MAX: u32 = 2147483647;
pub const UINT_LEAST8_MAX: u32 = 255;
pub const UINT_LEAST16_MAX: u32 = 65535;
pub const UINT_LEAST32_MAX: u32 = 4294967295;
pub const INT_FAST8_MIN: i32 = -128;
pub const INT_FAST16_MIN: i64 = -9223372036854775808;
pub const INT_FAST32_MIN: i64 = -9223372036854775808;
pub const INT_FAST8_MAX: u32 = 127;
pub const INT_FAST16_MAX: u64 = 9223372036854775807;
pub const INT_FAST32_MAX: u64 = 9223372036854775807;
pub const UINT_FAST8_MAX: u32 = 255;
pub const UINT_FAST16_MAX: i32 = -1;
pub const UINT_FAST32_MAX: i32 = -1;
pub const INTPTR_MIN: i64 = -9223372036854775808;
pub const INTPTR_MAX: u64 = 9223372036854775807;
pub const UINTPTR_MAX: i32 = -1;
pub const PTRDIFF_MIN: i64 = -9223372036854775808;
pub const PTRDIFF_MAX: u64 = 9223372036854775807;
pub const SIG_ATOMIC_MIN: i32 = -2147483648;
pub const SIG_ATOMIC_MAX: u32 = 2147483647;
pub const SIZE_MAX: i32 = -1;
pub const WINT_MIN: u32 = 0;
pub const WINT_MAX: u32 = 4294967295;
pub const __bool_true_false_are_defined: u32 = 1;
pub const true_: u32 = 1;
pub const false_: u32 = 0;
pub const WNOHANG: u32 = 1;
pub const WUNTRACED: u32 = 2;
pub const WSTOPPED: u32 = 2;
pub const WEXITED: u32 = 4;
pub const WCONTINUED: u32 = 8;
pub const WNOWAIT: u32 = 16777216;
pub const __WNOTHREAD: u32 = 536870912;
pub const __WALL: u32 = 1073741824;
pub const __WCLONE: u32 = 2147483648;
pub const __W_CONTINUED: u32 = 65535;
pub const __WCOREFLAG: u32 = 128;
pub const __HAVE_FLOAT128: u32 = 1;
pub const __HAVE_DISTINCT_FLOAT128: u32 = 1;
pub const __HAVE_FLOAT64X: u32 = 1;
pub const __HAVE_FLOAT64X_LONG_DOUBLE: u32 = 1;
pub const __HAVE_FLOAT16: u32 = 0;
pub const __HAVE_FLOAT32: u32 = 1;
pub const __HAVE_FLOAT64: u32 = 1;
pub const __HAVE_FLOAT32X: u32 = 1;
pub const __HAVE_FLOAT128X: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT16: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT32: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT64: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT32X: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT64X: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT128X: u32 = 0;
pub const __HAVE_FLOATN_NOT_TYPEDEF: u32 = 0;
pub const _SYS_TYPES_H: u32 = 1;
pub const __clock_t_defined: u32 = 1;
pub const __clockid_t_defined: u32 = 1;
pub const __time_t_defined: u32 = 1;
pub const __timer_t_defined: u32 = 1;
pub const __BIT_TYPES_DEFINED__: u32 = 1;
pub const _ENDIAN_H: u32 = 1;
pub const _BITS_ENDIAN_H: u32 = 1;
pub const __LITTLE_ENDIAN: u32 = 1234;
pub const __BIG_ENDIAN: u32 = 4321;
pub const __PDP_ENDIAN: u32 = 3412;
pub const _BITS_ENDIANNESS_H: u32 = 1;
pub const __BYTE_ORDER: u32 = 1234;
pub const __FLOAT_WORD_ORDER: u32 = 1234;
pub const LITTLE_ENDIAN: u32 = 1234;
pub const BIG_ENDIAN: u32 = 4321;
pub const PDP_ENDIAN: u32 = 3412;
pub const BYTE_ORDER: u32 = 1234;
pub const _BITS_BYTESWAP_H: u32 = 1;
pub const _BITS_UINTN_IDENTITY_H: u32 = 1;
pub const _SYS_SELECT_H: u32 = 1;
pub const __sigset_t_defined: u32 = 1;
pub const __timeval_defined: u32 = 1;
pub const _STRUCT_TIMESPEC: u32 = 1;
pub const FD_SETSIZE: u32 = 1024;
pub const _THREAD_SHARED_TYPES_H: u32 = 1;
pub const _THREAD_MUTEX_INTERNAL_H: u32 = 1;
pub const __PTHREAD_MUTEX_HAVE_PREV: u32 = 1;
pub const _ALLOCA_H: u32 = 1;
pub const _MM_HINT_ET0: u32 = 7;
pub const _MM_HINT_ET1: u32 = 6;
pub const _MM_HINT_T0: u32 = 3;
pub const _MM_HINT_T1: u32 = 2;
pub const _MM_HINT_T2: u32 = 1;
pub const _MM_HINT_NTA: u32 = 0;
pub const _CMP_EQ_OQ: u32 = 0;
pub const _CMP_LT_OS: u32 = 1;
pub const _CMP_LE_OS: u32 = 2;
pub const _CMP_UNORD_Q: u32 = 3;
pub const _CMP_NEQ_UQ: u32 = 4;
pub const _CMP_NLT_US: u32 = 5;
pub const _CMP_NLE_US: u32 = 6;
pub const _CMP_ORD_Q: u32 = 7;
pub const _MM_EXCEPT_INVALID: u32 = 1;
pub const _MM_EXCEPT_DENORM: u32 = 2;
pub const _MM_EXCEPT_DIV_ZERO: u32 = 4;
pub const _MM_EXCEPT_OVERFLOW: u32 = 8;
pub const _MM_EXCEPT_UNDERFLOW: u32 = 16;
pub const _MM_EXCEPT_INEXACT: u32 = 32;
pub const _MM_EXCEPT_MASK: u32 = 63;
pub const _MM_MASK_INVALID: u32 = 128;
pub const _MM_MASK_DENORM: u32 = 256;
pub const _MM_MASK_DIV_ZERO: u32 = 512;
pub const _MM_MASK_OVERFLOW: u32 = 1024;
pub const _MM_MASK_UNDERFLOW: u32 = 2048;
pub const _MM_MASK_INEXACT: u32 = 4096;
pub const _MM_MASK_MASK: u32 = 8064;
pub const _MM_ROUND_NEAREST: u32 = 0;
pub const _MM_ROUND_DOWN: u32 = 8192;
pub const _MM_ROUND_UP: u32 = 16384;
pub const _MM_ROUND_TOWARD_ZERO: u32 = 24576;
pub const _MM_ROUND_MASK: u32 = 24576;
pub const _MM_FLUSH_ZERO_MASK: u32 = 32768;
pub const _MM_FLUSH_ZERO_ON: u32 = 32768;
pub const _MM_FLUSH_ZERO_OFF: u32 = 0;
pub const _MM_DENORMALS_ZERO_ON: u32 = 64;
pub const _MM_DENORMALS_ZERO_OFF: u32 = 0;
pub const _MM_DENORMALS_ZERO_MASK: u32 = 64;
pub type __u_char = ::core::ffi::c_uchar;
pub type __u_short = ::core::ffi::c_ushort;
pub type __u_int = ::core::ffi::c_uint;
pub type __u_long = ::core::ffi::c_ulong;
pub type __int8_t = ::core::ffi::c_schar;
pub type __uint8_t = ::core::ffi::c_uchar;
pub type __int16_t = ::core::ffi::c_short;
pub type __uint16_t = ::core::ffi::c_ushort;
pub type __int32_t = ::core::ffi::c_int;
pub type __uint32_t = ::core::ffi::c_uint;
pub type __int64_t = ::core::ffi::c_long;
pub type __uint64_t = ::core::ffi::c_ulong;
pub type __int_least8_t = __int8_t;
pub type __uint_least8_t = __uint8_t;
pub type __int_least16_t = __int16_t;
pub type __uint_least16_t = __uint16_t;
pub type __int_least32_t = __int32_t;
pub type __uint_least32_t = __uint32_t;
pub type __int_least64_t = __int64_t;
pub type __uint_least64_t = __uint64_t;
pub type __quad_t = ::core::ffi::c_long;
pub type __u_quad_t = ::core::ffi::c_ulong;
pub type __intmax_t = ::core::ffi::c_long;
pub type __uintmax_t = ::core::ffi::c_ulong;
pub type __dev_t = ::core::ffi::c_ulong;
pub type __uid_t = ::core::ffi::c_uint;
pub type __gid_t = ::core::ffi::c_uint;
pub type __ino_t = ::core::ffi::c_ulong;
pub type __ino64_t = ::core::ffi::c_ulong;
pub type __mode_t = ::core::ffi::c_uint;
pub type __nlink_t = ::core::ffi::c_ulong;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __pid_t = ::core::ffi::c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __fsid_t {
    pub __val: [::core::ffi::c_int; 2usize],
}
pub type __clock_t = ::core::ffi::c_long;
pub type __rlim_t = ::core::ffi::c_ulong;
pub type __rlim64_t = ::core::ffi::c_ulong;
pub type __id_t = ::core::ffi::c_uint;
pub type __time_t = ::core::ffi::c_long;
pub type __useconds_t = ::core::ffi::c_uint;
pub type __suseconds_t = ::core::ffi::c_long;
pub type __suseconds64_t = ::core::ffi::c_long;
pub type __daddr_t = ::core::ffi::c_int;
pub type __key_t = ::core::ffi::c_int;
pub type __clockid_t = ::core::ffi::c_int;
pub type __timer_t = *mut ::core::ffi::c_void;
pub type __blksize_t = ::core::ffi::c_long;
pub type __blkcnt_t = ::core::ffi::c_long;
pub type __blkcnt64_t = ::core::ffi::c_long;
pub type __fsblkcnt_t = ::core::ffi::c_ulong;
pub type __fsblkcnt64_t = ::core::ffi::c_ulong;
pub type __fsfilcnt_t = ::core::ffi::c_ulong;
pub type __fsfilcnt64_t = ::core::ffi::c_ulong;
pub type __fsword_t = ::core::ffi::c_long;
pub type __ssize_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type __syscall_ulong_t = ::core::ffi::c_ulong;
pub type __loff_t = __off64_t;
pub type __caddr_t = *mut ::core::ffi::c_char;
pub type __intptr_t = ::core::ffi::c_long;
pub type __socklen_t = ::core::ffi::c_uint;
pub type __sig_atomic_t = ::core::ffi::c_int;
pub type int_least8_t = __int_least8_t;
pub type int_least16_t = __int_least16_t;
pub type int_least32_t = __int_least32_t;
pub type int_least64_t = __int_least64_t;
pub type uint_least8_t = __uint_least8_t;
pub type uint_least16_t = __uint_least16_t;
pub type uint_least32_t = __uint_least32_t;
pub type uint_least64_t = __uint_least64_t;
pub type int_fast8_t = ::core::ffi::c_schar;
pub type int_fast16_t = ::core::ffi::c_long;
pub type int_fast32_t = ::core::ffi::c_long;
pub type int_fast64_t = ::core::ffi::c_long;
pub type uint_fast8_t = ::core::ffi::c_uchar;
pub type uint_fast16_t = ::core::ffi::c_ulong;
pub type uint_fast32_t = ::core::ffi::c_ulong;
pub type uint_fast64_t = ::core::ffi::c_ulong;
pub type intmax_t = __intmax_t;
pub type uintmax_t = __uintmax_t;
pub type wchar_t = ::core::ffi::c_int;
#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Copy, Clone)]
pub struct max_align_t {
    pub __clang_max_align_nonce1: ::core::ffi::c_longlong,
    pub __bindgen_padding_0: u64,
    pub __clang_max_align_nonce2: u128,
}
pub type __m64 = [::core::ffi::c_longlong; 1usize];
pub type __v1di = [::core::ffi::c_longlong; 1usize];
pub type __v2si = [::core::ffi::c_int; 2usize];
pub type __v4hi = [::core::ffi::c_short; 4usize];
pub type __v8qi = [::core::ffi::c_char; 8usize];
pub type __v4si = [::core::ffi::c_int; 4usize];
pub type __v4sf = [f32; 4usize];
pub type __m128 = [f32; 4usize];
pub type __m128_u = [f32; 4usize];
pub type __v4su = [::core::ffi::c_uint; 4usize];
pub type __cfloat128 = __BindgenComplex<u128>;
pub type _Float128 = u128;
pub type _Float32 = f32;
pub type _Float64 = f64;
pub type _Float32x = f64;
pub type _Float64x = u128;
pub type u_char = __u_char;
pub type u_short = __u_short;
pub type u_int = __u_int;
pub type u_long = __u_long;
pub type quad_t = __quad_t;
pub type u_quad_t = __u_quad_t;
pub type fsid_t = __fsid_t;
pub type loff_t = __loff_t;
pub type ino_t = __ino_t;
pub type dev_t = __dev_t;
pub type gid_t = __gid_t;
pub type mode_t = __mode_t;
pub type nlink_t = __nlink_t;
pub type uid_t = __uid_t;
pub type off_t = __off_t;
pub type pid_t = __pid_t;
pub type id_t = __id_t;
pub type daddr_t = __daddr_t;
pub type caddr_t = __caddr_t;
pub type key_t = __key_t;
pub type clock_t = __clock_t;
pub type clockid_t = __clockid_t;
pub type time_t = __time_t;
pub type timer_t = __timer_t;
pub type ulong = ::core::ffi::c_ulong;
pub type ushort = ::core::ffi::c_ushort;
pub type uint = ::core::ffi::c_uint;
pub type u_int8_t = __uint8_t;
pub type u_int16_t = __uint16_t;
pub type u_int32_t = __uint32_t;
pub type u_int64_t = __uint64_t;
pub type register_t = ::core::ffi::c_long;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __sigset_t {
    pub __val: [::core::ffi::c_ulong; 16usize],
}
pub type sigset_t = __sigset_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type suseconds_t = __suseconds_t;
pub type __fd_mask = ::core::ffi::c_long;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fd_set {
    pub __fds_bits: [__fd_mask; 16usize],
}
pub type fd_mask = __fd_mask;
unsafe extern "C" {
    pub fn select(
        __nfds: ::core::ffi::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *mut timeval,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn pselect(
        __nfds: ::core::ffi::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *const timespec,
        __sigmask: *const __sigset_t,
    ) -> ::core::ffi::c_int;
}
pub type blksize_t = __blksize_t;
pub type blkcnt_t = __blkcnt_t;
pub type fsblkcnt_t = __fsblkcnt_t;
pub type fsfilcnt_t = __fsfilcnt_t;
#[repr(C)]
#[derive(Copy, Clone)]
pub union __atomic_wide_counter {
    pub __value64: ::core::ffi::c_ulonglong,
    pub __value32: __atomic_wide_counter__bindgen_ty_1,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __atomic_wide_counter__bindgen_ty_1 {
    pub __low: ::core::ffi::c_uint,
    pub __high: ::core::ffi::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pthread_internal_slist {
    pub __next: *mut __pthread_internal_slist,
}
pub type __pthread_slist_t = __pthread_internal_slist;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pthread_mutex_s {
    pub __lock: ::core::ffi::c_int,
    pub __count: ::core::ffi::c_uint,
    pub __owner: ::core::ffi::c_int,
    pub __nusers: ::core::ffi::c_uint,
    pub __kind: ::core::ffi::c_int,
    pub __spins: ::core::ffi::c_short,
    pub __elision: ::core::ffi::c_short,
    pub __list: __pthread_list_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pthread_rwlock_arch_t {
    pub __readers: ::core::ffi::c_uint,
    pub __writers: ::core::ffi::c_uint,
    pub __wrphase_futex: ::core::ffi::c_uint,
    pub __writers_futex: ::core::ffi::c_uint,
    pub __pad3: ::core::ffi::c_uint,
    pub __pad4: ::core::ffi::c_uint,
    pub __cur_writer: ::core::ffi::c_int,
    pub __shared: ::core::ffi::c_int,
    pub __rwelision: ::core::ffi::c_schar,
    pub __pad1: [::core::ffi::c_uchar; 7usize],
    pub __pad2: ::core::ffi::c_ulong,
    pub __flags: ::core::ffi::c_uint,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __pthread_cond_s {
    pub __wseq: __atomic_wide_counter,
    pub __g1_start: __atomic_wide_counter,
    pub __g_size: [::core::ffi::c_uint; 2usize],
    pub __g1_orig_size: ::core::ffi::c_uint,
    pub __wrefs: ::core::ffi::c_uint,
    pub __g_signals: [::core::ffi::c_uint; 2usize],
}
pub type __tss_t = ::core::ffi::c_uint;
pub type __thrd_t = ::core::ffi::c_ulong;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __once_flag {
    pub __data: ::core::ffi::c_int,
}
unsafe extern "C" {
    pub fn alloca(__size: ::core::ffi::c_ulong) -> *mut ::core::ffi::c_void;
}
unsafe extern "C" {
    pub fn posix_memalign(
        __memptr: *mut *mut ::core::ffi::c_void,
        __alignment: usize,
        __size: usize,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn _mm_sfence();
}
unsafe extern "C" {
    pub fn _mm_getcsr() -> ::core::ffi::c_uint;
}
unsafe extern "C" {
    pub fn _mm_setcsr(__i: ::core::ffi::c_uint);
}
pub type __m128d = [f64; 2usize];
pub type __m128i = [::core::ffi::c_longlong; 2usize];
pub type __m128d_u = [f64; 2usize];
pub type __m128i_u = [::core::ffi::c_longlong; 2usize];
pub type __v2df = [f64; 2usize];
pub type __v2di = [::core::ffi::c_longlong; 2usize];
pub type __v8hi = [::core::ffi::c_short; 8usize];
pub type __v16qi = [::core::ffi::c_char; 16usize];
pub type __v2du = [::core::ffi::c_ulonglong; 2usize];
pub type __v8hu = [::core::ffi::c_ushort; 8usize];
pub type __v16qu = [::core::ffi::c_uchar; 16usize];
pub type __v16qs = [::core::ffi::c_schar; 16usize];
pub type __v8hf = [__BindgenFloat16; 8usize];
pub type __m128h = [__BindgenFloat16; 8usize];
pub type __m128h_u = [__BindgenFloat16; 8usize];
pub type __v8bf = u128;
pub type __m128bh = u128;
unsafe extern "C" {
    pub fn _mm_clflush(__p: *const ::core::ffi::c_void);
}
unsafe extern "C" {
    pub fn _mm_lfence();
}
unsafe extern "C" {
    pub fn _mm_mfence();
}
unsafe extern "C" {
    pub fn _mm_pause();
}
unsafe extern "C" {
    pub static mut CLAY__ELEMENT_DEFINITION_LATCH: u8;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Clay_String {
    pub length: i32,
    pub chars: *const ::core::ffi::c_char,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Clay_StringSlice {
    pub length: i32,
    pub chars: *const ::core::ffi::c_char,
    pub baseChars: *const ::core::ffi::c_char,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Clay_Context {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Clay_Arena {
    pub nextAllocation: usize,
    pub capacity: usize,
    pub memory: *mut ::core::ffi::c_char,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Clay_Dimensions {
    pub width: f32,
    pub height: f32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Clay_Vector2 {
    pub x: f32,
    pub y: f32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Clay_Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Clay_BoundingBox {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Clay_ElementId {
    pub id: u32,
    pub offset: u32,
    pub baseId: u32,
    pub stringId: Clay_String,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Clay_CornerRadius {
    pub topLeft: f32,
    pub topRight: f32,
    pub bottomLeft: f32,
    pub bottomRight: f32,
}
pub const Clay_LayoutDirection_CLAY_LEFT_TO_RIGHT: Clay_LayoutDirection = 0;
pub const Clay_LayoutDirection_CLAY_TOP_TO_BOTTOM: Clay_LayoutDirection = 1;
pub type Clay_LayoutDirection = ::core::ffi::c_uchar;
pub const Clay_LayoutAlignmentX_CLAY_ALIGN_X_LEFT: Clay_LayoutAlignmentX = 0;
pub const Clay_LayoutAlignmentX_CLAY_ALIGN_X_RIGHT: Clay_LayoutAlignmentX = 1;
pub const Clay_LayoutAlignmentX_CLAY_ALIGN_X_CENTER: Clay_LayoutAlignmentX = 2;
pub type Clay_LayoutAlignmentX = ::core::ffi::c_uchar;
pub const Clay_LayoutAlignmentY_CLAY_ALIGN_Y_TOP: Clay_LayoutAlignmentY = 0;
pub const Clay_LayoutAlignmentY_CLAY_ALIGN_Y_BOTTOM: Clay_LayoutAlignmentY = 1;
pub const Clay_LayoutAlignmentY_CLAY_ALIGN_Y_CENTER: Clay_LayoutAlignmentY = 2;
pub type Clay_LayoutAlignmentY = ::core::ffi::c_uchar;
pub const Clay__SizingType_CLAY__SIZING_TYPE_FIT: Clay__SizingType = 0;
pub const Clay__SizingType_CLAY__SIZING_TYPE_GROW: Clay__SizingType = 1;
pub const Clay__SizingType_CLAY__SIZING_TYPE_PERCENT: Clay__SizingType = 2;
pub const Clay__SizingType_CLAY__SIZING_TYPE_FIXED: Clay__SizingType = 3;
pub type Clay__SizingType = ::core::ffi::c_uchar;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Clay_ChildAlignment {
    pub x: Clay_LayoutAlignmentX,
    pub y: Clay_LayoutAlignmentY,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Clay_SizingMinMax {
    pub min: f32,
    pub max: f32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Clay_SizingAxis {
    pub size: Clay_SizingAxis__bindgen_ty_1,
    pub type_: Clay__SizingType,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union Clay_SizingAxis__bindgen_ty_1 {
    pub minMax: Clay_SizingMinMax,
    pub percent: f32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Clay_Sizing {
    pub width: Clay_SizingAxis,
    pub height: Clay_SizingAxis,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Clay_Padding {
    pub left: u16,
    pub right: u16,
    pub top: u16,
    pub bottom: u16,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Clay__Clay_PaddingWrapper {
    pub wrapped: Clay_Padding,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Clay_LayoutConfig {
    pub sizing: Clay_Sizing,
    pub padding: Clay_Padding,
    pub childGap: u16,
    pub childAlignment: Clay_ChildAlignment,
    pub layoutDirection: Clay_LayoutDirection,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Clay__Clay_LayoutConfigWrapper {
    pub wrapped: Clay_LayoutConfig,
}
unsafe extern "C" {
    pub static mut CLAY_LAYOUT_DEFAULT: Clay_LayoutConfig;
}
pub const Clay_TextElementConfigWrapMode_CLAY_TEXT_WRAP_WORDS: Clay_TextElementConfigWrapMode = 0;
pub const Clay_TextElementConfigWrapMode_CLAY_TEXT_WRAP_NEWLINES: Clay_TextElementConfigWrapMode =
    1;
pub const Clay_TextElementConfigWrapMode_CLAY_TEXT_WRAP_NONE: Clay_TextElementConfigWrapMode = 2;
pub type Clay_TextElementConfigWrapMode = ::core::ffi::c_uchar;
pub const Clay_TextAlignment_CLAY_TEXT_ALIGN_LEFT: Clay_TextAlignment = 0;
pub const Clay_TextAlignment_CLAY_TEXT_ALIGN_CENTER: Clay_TextAlignment = 1;
pub const Clay_TextAlignment_CLAY_TEXT_ALIGN_RIGHT: Clay_TextAlignment = 2;
pub type Clay_TextAlignment = ::core::ffi::c_uchar;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Clay_TextElementConfig {
    pub textColor: Clay_Color,
    pub fontId: u16,
    pub fontSize: u16,
    pub letterSpacing: u16,
    pub lineHeight: u16,
    pub wrapMode: Clay_TextElementConfigWrapMode,
    pub textAlignment: Clay_TextAlignment,
    pub hashStringContents: bool,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Clay__Clay_TextElementConfigWrapper {
    pub wrapped: Clay_TextElementConfig,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Clay_ImageElementConfig {
    pub imageData: *mut ::core::ffi::c_void,
    pub sourceDimensions: Clay_Dimensions,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Clay__Clay_ImageElementConfigWrapper {
    pub wrapped: Clay_ImageElementConfig,
}
pub const Clay_FloatingAttachPointType_CLAY_ATTACH_POINT_LEFT_TOP: Clay_FloatingAttachPointType = 0;
pub const Clay_FloatingAttachPointType_CLAY_ATTACH_POINT_LEFT_CENTER: Clay_FloatingAttachPointType =
    1;
pub const Clay_FloatingAttachPointType_CLAY_ATTACH_POINT_LEFT_BOTTOM: Clay_FloatingAttachPointType =
    2;
pub const Clay_FloatingAttachPointType_CLAY_ATTACH_POINT_CENTER_TOP: Clay_FloatingAttachPointType =
    3;
pub const Clay_FloatingAttachPointType_CLAY_ATTACH_POINT_CENTER_CENTER:
    Clay_FloatingAttachPointType = 4;
pub const Clay_FloatingAttachPointType_CLAY_ATTACH_POINT_CENTER_BOTTOM:
    Clay_FloatingAttachPointType = 5;
pub const Clay_FloatingAttachPointType_CLAY_ATTACH_POINT_RIGHT_TOP: Clay_FloatingAttachPointType =
    6;
pub const Clay_FloatingAttachPointType_CLAY_ATTACH_POINT_RIGHT_CENTER:
    Clay_FloatingAttachPointType = 7;
pub const Clay_FloatingAttachPointType_CLAY_ATTACH_POINT_RIGHT_BOTTOM:
    Clay_FloatingAttachPointType = 8;
pub type Clay_FloatingAttachPointType = ::core::ffi::c_uchar;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Clay_FloatingAttachPoints {
    pub element: Clay_FloatingAttachPointType,
    pub parent: Clay_FloatingAttachPointType,
}
pub const Clay_PointerCaptureMode_CLAY_POINTER_CAPTURE_MODE_CAPTURE: Clay_PointerCaptureMode = 0;
pub const Clay_PointerCaptureMode_CLAY_POINTER_CAPTURE_MODE_PASSTHROUGH: Clay_PointerCaptureMode =
    1;
pub type Clay_PointerCaptureMode = ::core::ffi::c_uchar;
pub const Clay_FloatingAttachToElement_CLAY_ATTACH_TO_NONE: Clay_FloatingAttachToElement = 0;
pub const Clay_FloatingAttachToElement_CLAY_ATTACH_TO_PARENT: Clay_FloatingAttachToElement = 1;
pub const Clay_FloatingAttachToElement_CLAY_ATTACH_TO_ELEMENT_WITH_ID:
    Clay_FloatingAttachToElement = 2;
pub const Clay_FloatingAttachToElement_CLAY_ATTACH_TO_ROOT: Clay_FloatingAttachToElement = 3;
pub type Clay_FloatingAttachToElement = ::core::ffi::c_uchar;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Clay_FloatingElementConfig {
    pub offset: Clay_Vector2,
    pub expand: Clay_Dimensions,
    pub parentId: u32,
    pub zIndex: i16,
    pub attachPoints: Clay_FloatingAttachPoints,
    pub pointerCaptureMode: Clay_PointerCaptureMode,
    pub attachTo: Clay_FloatingAttachToElement,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Clay__Clay_FloatingElementConfigWrapper {
    pub wrapped: Clay_FloatingElementConfig,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Clay_CustomElementConfig {
    pub customData: *mut ::core::ffi::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Clay__Clay_CustomElementConfigWrapper {
    pub wrapped: Clay_CustomElementConfig,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Clay_ScrollElementConfig {
    pub horizontal: bool,
    pub vertical: bool,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Clay__Clay_ScrollElementConfigWrapper {
    pub wrapped: Clay_ScrollElementConfig,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Clay_BorderWidth {
    pub left: u16,
    pub right: u16,
    pub top: u16,
    pub bottom: u16,
    pub betweenChildren: u16,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Clay_BorderElementConfig {
    pub color: Clay_Color,
    pub width: Clay_BorderWidth,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Clay__Clay_BorderElementConfigWrapper {
    pub wrapped: Clay_BorderElementConfig,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Clay_TextRenderData {
    pub stringContents: Clay_StringSlice,
    pub textColor: Clay_Color,
    pub fontId: u16,
    pub fontSize: u16,
    pub letterSpacing: u16,
    pub lineHeight: u16,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Clay_RectangleRenderData {
    pub backgroundColor: Clay_Color,
    pub cornerRadius: Clay_CornerRadius,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Clay_ImageRenderData {
    pub backgroundColor: Clay_Color,
    pub cornerRadius: Clay_CornerRadius,
    pub sourceDimensions: Clay_Dimensions,
    pub imageData: *mut ::core::ffi::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Clay_CustomRenderData {
    pub backgroundColor: Clay_Color,
    pub cornerRadius: Clay_CornerRadius,
    pub customData: *mut ::core::ffi::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Clay_ScrollRenderData {
    pub horizontal: bool,
    pub vertical: bool,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Clay_BorderRenderData {
    pub color: Clay_Color,
    pub cornerRadius: Clay_CornerRadius,
    pub width: Clay_BorderWidth,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union Clay_RenderData {
    pub rectangle: Clay_RectangleRenderData,
    pub text: Clay_TextRenderData,
    pub image: Clay_ImageRenderData,
    pub custom: Clay_CustomRenderData,
    pub border: Clay_BorderRenderData,
    pub scroll: Clay_ScrollRenderData,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Clay_ScrollContainerData {
    pub scrollPosition: *mut Clay_Vector2,
    pub scrollContainerDimensions: Clay_Dimensions,
    pub contentDimensions: Clay_Dimensions,
    pub config: Clay_ScrollElementConfig,
    pub found: bool,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Clay_ElementData {
    pub boundingBox: Clay_BoundingBox,
    pub found: bool,
}
pub const Clay_RenderCommandType_CLAY_RENDER_COMMAND_TYPE_NONE: Clay_RenderCommandType = 0;
pub const Clay_RenderCommandType_CLAY_RENDER_COMMAND_TYPE_RECTANGLE: Clay_RenderCommandType = 1;
pub const Clay_RenderCommandType_CLAY_RENDER_COMMAND_TYPE_BORDER: Clay_RenderCommandType = 2;
pub const Clay_RenderCommandType_CLAY_RENDER_COMMAND_TYPE_TEXT: Clay_RenderCommandType = 3;
pub const Clay_RenderCommandType_CLAY_RENDER_COMMAND_TYPE_IMAGE: Clay_RenderCommandType = 4;
pub const Clay_RenderCommandType_CLAY_RENDER_COMMAND_TYPE_SCISSOR_START: Clay_RenderCommandType = 5;
pub const Clay_RenderCommandType_CLAY_RENDER_COMMAND_TYPE_SCISSOR_END: Clay_RenderCommandType = 6;
pub const Clay_RenderCommandType_CLAY_RENDER_COMMAND_TYPE_CUSTOM: Clay_RenderCommandType = 7;
pub type Clay_RenderCommandType = ::core::ffi::c_uchar;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Clay_RenderCommand {
    pub boundingBox: Clay_BoundingBox,
    pub renderData: Clay_RenderData,
    pub userData: *mut ::core::ffi::c_void,
    pub id: u32,
    pub zIndex: i16,
    pub commandType: Clay_RenderCommandType,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Clay_RenderCommandArray {
    pub capacity: i32,
    pub length: i32,
    pub internalArray: *mut Clay_RenderCommand,
}
pub const Clay_PointerDataInteractionState_CLAY_POINTER_DATA_PRESSED_THIS_FRAME:
    Clay_PointerDataInteractionState = 0;
pub const Clay_PointerDataInteractionState_CLAY_POINTER_DATA_PRESSED:
    Clay_PointerDataInteractionState = 1;
pub const Clay_PointerDataInteractionState_CLAY_POINTER_DATA_RELEASED_THIS_FRAME:
    Clay_PointerDataInteractionState = 2;
pub const Clay_PointerDataInteractionState_CLAY_POINTER_DATA_RELEASED:
    Clay_PointerDataInteractionState = 3;
pub type Clay_PointerDataInteractionState = ::core::ffi::c_uchar;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Clay_PointerData {
    pub position: Clay_Vector2,
    pub state: Clay_PointerDataInteractionState,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Clay_ElementDeclaration {
    pub id: Clay_ElementId,
    pub layout: Clay_LayoutConfig,
    pub backgroundColor: Clay_Color,
    pub cornerRadius: Clay_CornerRadius,
    pub image: Clay_ImageElementConfig,
    pub floating: Clay_FloatingElementConfig,
    pub custom: Clay_CustomElementConfig,
    pub scroll: Clay_ScrollElementConfig,
    pub border: Clay_BorderElementConfig,
    pub userData: *mut ::core::ffi::c_void,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Clay__Clay_ElementDeclarationWrapper {
    pub wrapped: Clay_ElementDeclaration,
}
pub const Clay_ErrorType_CLAY_ERROR_TYPE_TEXT_MEASUREMENT_FUNCTION_NOT_PROVIDED: Clay_ErrorType = 0;
pub const Clay_ErrorType_CLAY_ERROR_TYPE_ARENA_CAPACITY_EXCEEDED: Clay_ErrorType = 1;
pub const Clay_ErrorType_CLAY_ERROR_TYPE_ELEMENTS_CAPACITY_EXCEEDED: Clay_ErrorType = 2;
pub const Clay_ErrorType_CLAY_ERROR_TYPE_TEXT_MEASUREMENT_CAPACITY_EXCEEDED: Clay_ErrorType = 3;
pub const Clay_ErrorType_CLAY_ERROR_TYPE_DUPLICATE_ID: Clay_ErrorType = 4;
pub const Clay_ErrorType_CLAY_ERROR_TYPE_FLOATING_CONTAINER_PARENT_NOT_FOUND: Clay_ErrorType = 5;
pub const Clay_ErrorType_CLAY_ERROR_TYPE_PERCENTAGE_OVER_1: Clay_ErrorType = 6;
pub const Clay_ErrorType_CLAY_ERROR_TYPE_INTERNAL_ERROR: Clay_ErrorType = 7;
pub type Clay_ErrorType = ::core::ffi::c_uchar;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Clay_ErrorData {
    pub errorType: Clay_ErrorType,
    pub errorText: Clay_String,
    pub userData: *mut ::core::ffi::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Clay_ErrorHandler {
    pub errorHandlerFunction:
        ::core::option::Option<unsafe extern "C" fn(errorText: Clay_ErrorData)>,
    pub userData: *mut ::core::ffi::c_void,
}
unsafe extern "C" {
    pub fn Clay_MinMemorySize() -> u32;
}
unsafe extern "C" {
    pub fn Clay_CreateArenaWithCapacityAndMemory(
        capacity: usize,
        memory: *mut ::core::ffi::c_void,
    ) -> Clay_Arena;
}
unsafe extern "C" {
    pub fn Clay_SetPointerState(position: Clay_Vector2, pointerDown: bool);
}
unsafe extern "C" {
    pub fn Clay_Initialize(
        arena: Clay_Arena,
        layoutDimensions: Clay_Dimensions,
        errorHandler: Clay_ErrorHandler,
    ) -> *mut Clay_Context;
}
unsafe extern "C" {
    pub fn Clay_GetCurrentContext() -> *mut Clay_Context;
}
unsafe extern "C" {
    pub fn Clay_SetCurrentContext(context: *mut Clay_Context);
}
unsafe extern "C" {
    pub fn Clay_UpdateScrollContainers(
        enableDragScrolling: bool,
        scrollDelta: Clay_Vector2,
        deltaTime: f32,
    );
}
unsafe extern "C" {
    pub fn Clay_SetLayoutDimensions(dimensions: Clay_Dimensions);
}
unsafe extern "C" {
    pub fn Clay_BeginLayout();
}
unsafe extern "C" {
    pub fn Clay_EndLayout() -> Clay_RenderCommandArray;
}
unsafe extern "C" {
    pub fn Clay_GetElementId(idString: Clay_String) -> Clay_ElementId;
}
unsafe extern "C" {
    pub fn Clay_GetElementIdWithIndex(idString: Clay_String, index: u32) -> Clay_ElementId;
}
unsafe extern "C" {
    pub fn Clay_GetElementData(id: Clay_ElementId) -> Clay_ElementData;
}
unsafe extern "C" {
    pub fn Clay_Hovered() -> bool;
}
unsafe extern "C" {
    pub fn Clay_OnHover(
        onHoverFunction: ::core::option::Option<
            unsafe extern "C" fn(
                elementId: Clay_ElementId,
                pointerData: Clay_PointerData,
                userData: isize,
            ),
        >,
        userData: isize,
    );
}
unsafe extern "C" {
    pub fn Clay_PointerOver(elementId: Clay_ElementId) -> bool;
}
unsafe extern "C" {
    pub fn Clay_GetScrollContainerData(id: Clay_ElementId) -> Clay_ScrollContainerData;
}
unsafe extern "C" {
    pub fn Clay_SetMeasureTextFunction(
        measureTextFunction: ::core::option::Option<
            unsafe extern "C" fn(
                text: Clay_StringSlice,
                config: *mut Clay_TextElementConfig,
                userData: *mut ::core::ffi::c_void,
            ) -> Clay_Dimensions,
        >,
        userData: *mut ::core::ffi::c_void,
    );
}
unsafe extern "C" {
    pub fn Clay_SetQueryScrollOffsetFunction(
        queryScrollOffsetFunction: ::core::option::Option<
            unsafe extern "C" fn(
                elementId: u32,
                userData: *mut ::core::ffi::c_void,
            ) -> Clay_Vector2,
        >,
        userData: *mut ::core::ffi::c_void,
    );
}
unsafe extern "C" {
    pub fn Clay_RenderCommandArray_Get(
        array: *mut Clay_RenderCommandArray,
        index: i32,
    ) -> *mut Clay_RenderCommand;
}
unsafe extern "C" {
    pub fn Clay_SetDebugModeEnabled(enabled: bool);
}
unsafe extern "C" {
    pub fn Clay_IsDebugModeEnabled() -> bool;
}
unsafe extern "C" {
    pub fn Clay_SetCullingEnabled(enabled: bool);
}
unsafe extern "C" {
    pub fn Clay_GetMaxElementCount() -> i32;
}
unsafe extern "C" {
    pub fn Clay_SetMaxElementCount(maxElementCount: i32);
}
unsafe extern "C" {
    pub fn Clay_GetMaxMeasureTextCacheWordCount() -> i32;
}
unsafe extern "C" {
    pub fn Clay_SetMaxMeasureTextCacheWordCount(maxMeasureTextCacheWordCount: i32);
}
unsafe extern "C" {
    pub fn Clay_ResetMeasureTextCache();
}
unsafe extern "C" {
    pub fn Clay__OpenElement();
}
unsafe extern "C" {
    pub fn Clay__ConfigureOpenElement(config: Clay_ElementDeclaration);
}
unsafe extern "C" {
    pub fn Clay__CloseElement();
}
unsafe extern "C" {
    pub fn Clay__HashString(key: Clay_String, offset: u32, seed: u32) -> Clay_ElementId;
}
unsafe extern "C" {
    pub fn Clay__OpenTextElement(text: Clay_String, textConfig: *mut Clay_TextElementConfig);
}
unsafe extern "C" {
    pub fn Clay__StoreTextElementConfig(
        config: Clay_TextElementConfig,
    ) -> *mut Clay_TextElementConfig;
}
unsafe extern "C" {
    pub fn Clay__GetParentElementId() -> u32;
}
unsafe extern "C" {
    pub static mut Clay__debugViewHighlightColor: Clay_Color;
}
unsafe extern "C" {
    pub static mut Clay__debugViewWidth: u32;
}
