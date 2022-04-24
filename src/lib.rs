#![deny(warnings)]
#![doc(test(attr(deny(warnings))))]
#![doc(test(attr(allow(dead_code))))]
#![doc(test(attr(allow(unused_variables))))]

#![no_std]

use libc::{c_char, c_int, size_t};

include!(concat!(env!("OUT_DIR"), "/iconv_types.rs"));

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(transparent)]
pub struct iconv_t(pub IconvTRaw);

impl iconv_t {
    pub const ERROR: iconv_t = iconv_t(-1);
}

extern "C" {
    pub fn iconv_open(tocode: *const c_char, fromcode: *const c_char) -> iconv_t;
    pub fn iconv(
        cd: iconv_t,
        inbuf: *mut *mut c_char,
        inbytesleft: *mut size_t,
        outbuf: *mut *mut c_char,
        outbytesleft: *mut size_t,
    ) -> size_t;
    pub fn iconv_close(cd: iconv_t) -> c_int;
}
