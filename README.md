# fswatch-sys

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
  // Star the monitor, unwrapping any possible error. This will most likely be a blocking call.
  // See the libfswatch documentation for more information.
  session.start_monitor().unwrap();
}
```

```rust
extern crate fswatch_sys;

use fswatch_sys::{Fsw, FswSession, FswSessionBuilder};

fn main() {
  Fsw::init_library().expect("Could not start fswatch");

  FswSessionBuilder::new(vec!["./"], |events| println!("{:#?}", events))
    .build()
    .unwrap()
    .start_monitor()
    .unwrap();
}
```
