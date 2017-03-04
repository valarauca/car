#![allow(bad_style)]

extern crate lzma_sys;
extern crate libc;

use libc::*;
use lzma_sys::*;

include!(concat!(env!("OUT_DIR"), "/all.rs"));
