# cge-core

[![Build Status](https://travis-ci.org/pengowen123/cge-core.svg?branch=master)](https://travis-ci.org/pengowen123/cge-core)

A no_std version of the CGE library. Not complete yet, will be released soon.

# Usage

Add this to your Cargo.toml:

```
[dependencies]

cge_core = { git = "https://github.com/pengowen123/cge-core" }
```

And this to your crate root:

```rust
extern crate cge_core;
```

To load a neural network, first create one using [EANT2][1], save it to a file, and copy the data in the file into a string:

```rust
use cge_core::*;

fn main() {
    let string = "n 0.5 0 1, i 0.4 0"; // This string should be the data in the network file

    let mut network = Network::new(string);
    
    let result = network.evaluate(&[1.0, 1.0]);

    network.reset();
}
```

See the [documentation][2] for more instructions.

[1]: https://github.com/pengowen123/eant2
[2]: http://pengowen123.github.io/cge-core/cge_core/index.html
