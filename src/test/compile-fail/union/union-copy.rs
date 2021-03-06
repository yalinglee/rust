// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(untagged_unions)]

union U {
    a: u8
}

union W {
    a: String
}

impl Clone for U { fn clone(&self) { panic!(); } }
impl Clone for W { fn clone(&self) { panic!(); } }
impl Copy for U {} // OK
impl Copy for W {} //~ ERROR the trait `Copy` may not be implemented for this type

fn main() {}
