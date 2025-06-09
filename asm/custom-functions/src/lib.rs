#![no_std]
#![feature(allocator_api)]
#![feature(split_array)]
#![feature(const_trait_impl)]
#![allow(dead_code)]
#![feature(slice_ptr_get)]

#[macro_use]
extern crate alloc;

use core::compile_error;

#[cfg(feature = "static")]
include!("static.rs");

#[cfg(feature = "dynamic")]
include!("dynamic.rs");

#[cfg(not(any(feature = "static", feature = "dynamic")))]
compile_error!("Must specify either 'static' or 'dynamic' feature");
