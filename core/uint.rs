// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[cfg(target_word_size = "32")]
#[inline(always)]
pub fn add_with_overflow(x: uint, y: uint) -> (uint, bool) {
    let (a, b) = ::u32::add_with_overflow(x as u32, y as u32);
    (a as uint, b)
}

#[cfg(target_word_size = "64")]
#[inline(always)]
pub fn add_with_overflow(x: uint, y: uint) -> (uint, bool) {
    let (a, b) = ::u64::add_with_overflow(x as u64, y as u64);
    (a as uint, b)
}

#[cfg(target_word_size = "32")]
#[inline(always)]
pub fn sub_with_overflow(x: uint, y: uint) -> (uint, bool) {
    let (a, b) = ::u32::sub_with_overflow(x as u32, y as u32);
    (a as uint, b)
}

#[cfg(target_word_size = "64")]
#[inline(always)]
pub fn sub_with_overflow(x: uint, y: uint) -> (uint, bool) {
    let (a, b) = ::u64::sub_with_overflow(x as u64, y as u64);
    (a as uint, b)
}

#[cfg(target_word_size = "32")]
#[inline(always)]
pub fn mul_with_overflow(x: uint, y: uint) -> (uint, bool) {
    let (a, b) = ::u32::mul_with_overflow(x as u32, y as u32);
    (a as uint, b)
}

#[cfg(target_word_size = "64")]
#[inline(always)]
pub fn mul_with_overflow(x: uint, y: uint) -> (uint, bool) {
    let (a, b) = ::u64::mul_with_overflow(x as u64, y as u64);
    (a as uint, b)
}

#[cfg(target_word_size = "32")]
pub fn bswap(x: uint) -> uint {
    ::u32::bswap(x as u32) as uint
}

#[cfg(target_word_size = "64")]
pub fn bswap(x: uint) -> uint {
    ::i64::bswap(x as u64) as uint
}

#[cfg(target_endian = "big")]
pub fn to_be(x: uint) -> uint {
    x
}

#[cfg(target_endian = "little")]
pub fn to_be(x: uint) -> uint {
    bswap(x)
}

#[cfg(target_endian = "big")]
pub fn to_le(x: uint) -> uint {
    bswap(x)
}

#[cfg(target_endian = "little")]
pub fn to_le(x: uint) -> uint {
    x
}
