# fswatch-sys
[![Travis](https://img.shields.io/travis/jkcclemens/fswatch-sys.svg)](https://travis-ci.org/jkcclemens/fswatch-sys) [![Crates.io](https://img.shields.io/crates/v/fswatch-sys.svg)](https://crates.io/crates/fswatch-sys)

This is a Rust crate to integrate with the C API of
[`libfswatch`](https://github.com/emcrisostomo/fswatch).

```rust
extern crate fswatch_sys;

use fswatch_sys::{Fsw, FswSession};

fn main() {
  // Initialize the library. This must be called before anything else can be done.
  Fsw::init_library().expect("Could not start fswatch");

  // Create a new session.
  let session = FswSession::default().unwrap();
  // Add a monitoring path, unwrapping any possible error.
  session.add_path("./").unwrap();
  // Set the callback for when events are fired, unwrapping any possible error.
  session.set_callback(|events| {
    // Prettily print out the vector of events.
    println!("{:#?}", events);
  }).unwrap();
  // Start the monitor, unwrapping any possible error. This will most likely be a blocking call.
  // See the libfswatch documentation for more information.
  session.start_monitor().unwrap();
}
```

```rust
extern crate fswatch_sys;

use fswatch_sys::{Fsw, FswSessionBuilder};

fn main() {
  Fsw::init_library().expect("Could not start fswatch");

  FswSessionBuilder::new(vec!["./"])
    .build_callback(|events| println!("{:#?}", events))
    .unwrap()
    .start_monitor()
    .unwrap();
}
```

```rust
extern crate fswatch_sys;

use fswatch_sys::{Fsw, FswSession};

fn main() {
  Fsw::init_library().expect("Could not start fswatch");

  let session = FswSessionBuilder::empty().add_path("./").build().unwrap();
  for event in session {
    println!("{:#?}", event);
  }
}
```
