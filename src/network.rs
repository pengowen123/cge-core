//! The neural network struct
use stack::Stack;
use gene::{Gene, Variant};

/// A struct representing a neural network
///
/// Call `new` with a `&str` to create a neural network (see [here] for format details)
/// Call `evaluate` with a `&[f64]` to the output of the network
/// Call `reset` to clear the internal state
///
/// [1]: http://pengowen123.github.io/cge/cge/network/struct.Network.html#method.load_from_file
pub struct Network<'a> {
    genome: &'a [Gene],
}

impl<'a> Network<'a> {}
