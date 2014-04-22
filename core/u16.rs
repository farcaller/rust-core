// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern "rust-intrinsic" {
    fn bswap16(x: u16) -> u16;
    fn u16_add_with_overflow(x: u16, y: u16) -> (u16, bool);
    fn u16_sub_with_overflow(x: u16, y: u16) -> (u16, bool);
    fn u16_mul_with_overflow(x: u16, y: u16) -> (u16, bool);
}

#[inline(always)]
pub fn add_with_overflow(x: u16, y: u16) -> (u16, bool) {
    unsafe { u16_add_with_overflow(x, y) }
}

#[inline(always)]
pub fn sub_with_overflow(x: u16, y: u16) -> (u16, bool) {
    unsafe { u16_sub_with_overflow(x, y) }
}

#[inline(always)]
pub fn mul_with_overflow(x: u16, y: u16) -> (u16, bool) {
    unsafe { u16_mul_with_overflow(x, y) }
}

#[allow(dead_code)]
pub fn bswap(x: u16) -> u16 {
    ::i16::bswap(x as u16) as u16
}

#[cfg(target_endian = "big")]
pub fn to_be(x: u16) -> u16 {
    x
}

#[cfg(target_endian = "little")]
pub fn to_be(x: u16) -> u16 {
    bswap(x)
}

#[cfg(target_endian = "big")]
pub fn to_le(x: u16) -> u16 {
    bswap(x)
}

#[cfg(target_endian = "little")]
pub fn to_le(x: u16) -> u16 {
    x
}
