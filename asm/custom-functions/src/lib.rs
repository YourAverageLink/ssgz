#![no_std]
#![feature(allocator_api)]
#![feature(split_array)]
#![feature(const_trait_impl)]
#![allow(dead_code)]
#![feature(slice_ptr_get)]
#![allow(unused_imports)]

#[macro_use]
extern crate alloc;

#[cfg(feature = "static")]
include!("static.rs");

#[cfg(feature = "dynamic")]
include!("dynamic.rs");

#[cfg(not(any(feature = "static", feature = "dynamic")))]
core::compile_error!(
    "Must build with either 'static' (for main.dol code) or 'dynamic' (for rel code) feature."
);
