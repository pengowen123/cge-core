//! A freestanding version of the [CGE] library for use in embedded systems, or places where the
//! standard library is unavailable. Provides minimal functionality for evaluating a neural
//! network. This library does not train neural networks. If you need to train a network, use the
//! [eant2] library instead.
//!
//! [1]: https://github.com/pengowen123/cge
//! [2]: https://github.com/pengowen123/cge-core

// Keep this library extremely basic and minimal
// Optimize as much as possible (even if it means sacraficing some performance) to reduce file
// size (aim for < 50kb)
#![no_std]

extern crate rlibc;

mod stack;
mod gene;
pub mod network;

pub use network::Network;
