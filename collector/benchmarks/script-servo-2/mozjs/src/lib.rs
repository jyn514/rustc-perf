#![allow(warnings)]

/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#[allow(unused_extern_crates)]
extern crate encoding_c;
#[allow(unused_extern_crates)]
extern crate encoding_c_mem;
extern crate libc;
#[allow(unused_extern_crates)]
extern crate libz_sys;

// The jsimpls module just implements traits so can be private
mod jsimpls;

// Modules with public definitions
pub mod jsgc;
pub mod jsid;
pub mod jsval;

pub mod jsapi;
