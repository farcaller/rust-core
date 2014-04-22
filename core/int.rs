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
pub fn add_with_overflow(x: int, y: int) -> (int, bool) {
    let (a, b) = ::i32::add_with_overflow(x as i32, y as i32);
    (a as int, b)
}

#[cfg(target_word_size = "64")]
#[inline(always)]
pub fn add_with_overflow(x: int, y: int) -> (int, bool) {
    let (a, b) = ::i64::add_with_overflow(x as i64, y as i64);
    (a as int, b)
}

#[cfg(target_word_size = "32")]
#[inline(always)]
pub fn sub_with_overflow(x: int, y: int) -> (int, bool) {
    let (a, b) = ::i32::sub_with_overflow(x as i32, y as i32);
    (a as int, b)
}

#[cfg(target_word_size = "64")]
#[inline(always)]
pub fn sub_with_overflow(x: int, y: int) -> (int, bool) {
    let (a, b) = ::i64::sub_with_overflow(x as i64, y as i64);
    (a as int, b)
}

#[cfg(target_word_size = "32")]
#[inline(always)]
pub fn mul_with_overflow(x: int, y: int) -> (int, bool) {
    let (a, b) = ::i32::mul_with_overflow(x as i32, y as i32);
    (a as int, b)
}

#[cfg(target_word_size = "64")]
#[inline(always)]
pub fn mul_with_overflow(x: int, y: int) -> (int, bool) {
    let (a, b) = ::i64::mul_with_overflow(x as i64, y as i64);
    (a as int, b)
}

#[cfg(target_word_size = "32")]
pub fn bswap(x: int) -> int {
    ::i32::bswap(x as u32) as int
}

#[cfg(target_word_size = "64")]
pub fn bswap(x: int) -> int {
    ::i64::bswap(x as u64) as int
}

#[cfg(target_endian = "big")]
pub fn to_be(x: int) -> int {
    x
}

#[cfg(target_endian = "little")]
pub fn to_be(x: int) -> int {
    bswap(x)
}

#[cfg(target_endian = "big")]
pub fn to_le(x: int) -> int {
    bswap(x)
}

#[cfg(target_endian = "little")]
pub fn to_le(x: int) -> int {
    x
}
