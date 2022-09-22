#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(deref_nullptr)]
#![allow(improper_ctypes)]
#![no_std]

extern crate std;

#[cfg(not(target_arch="wasm32"))]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(target_arch="wasm32")]
include!("bindings.rs");

unsafe impl Send for pxtnService {}