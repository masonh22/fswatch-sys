extern crate fswatch_sys;

use fswatch_sys::{Fsw, FswSession};

fn main() {
  Fsw::init_library().unwrap();

  let session = FswSession::builder_paths(vec!["./"]).build().unwrap();
  for event in session {
    println!("{:#?}", event);
    #[cfg(feature = "use_time")]
    { println!("{}", event.time.ctime()); }
  }
}
