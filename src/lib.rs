//! Splay tree based data structures
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), feature(alloc))]
#![warn(missing_docs)]

#[cfg(feature = "serde")]
#[macro_use]
extern crate serde;

#[cfg(not(feature = "std"))]
#[macro_use]
pub extern crate alloc;

#[cfg(not(feature = "std"))]
mod std {
    pub use alloc::*;
    pub use core::{borrow, cmp, fmt, hash, iter, mem, ops, slice, u32};
}

mod tree_core;
mod iter;
mod vec_like;
pub mod map;
pub mod set;
pub mod heap;

#[doc(inline)]
pub use map::SplayMap;

#[doc(inline)]
pub use set::SplaySet;

#[doc(inline)]
pub use heap::SplayHeap;
