// Copyright 2015-2017 Parity Technologies
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Efficient large, fixed-size big integers and hashes.

#![cfg_attr(not(feature="std"), no_std)]
#![cfg_attr(all(not(feature="std"), test), feature(alloc))]

#[doc(hidden)]
pub extern crate byteorder;

#[cfg(feature="heapsizeof")]
#[doc(hidden)]
pub extern crate heapsize;

#[cfg(feature="std")]
#[doc(hidden)]
pub extern crate core;

#[doc(hidden)]
pub extern crate rustc_hex;

#[cfg(feature="impl_quickcheck_arbitrary")]
#[doc(hidden)]
pub extern crate quickcheck;

#[cfg(all(not(feature = "std"), test))]
extern crate alloc;

#[macro_use]
extern crate crunchy;

mod uint;
pub use uint::*;
