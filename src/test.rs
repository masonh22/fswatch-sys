use FswSession;

fn get_default_session() -> FswSession {
  FswSession::default().unwrap()
}

#[test]
fn create_and_destroy_session() {
  let handle = {
    let session = get_default_session();
    session.handle
  };
  // Check that the handle was created successfully.
  assert!(handle != ::FSW_INVALID_HANDLE);
  // Check that trying to destroy the handle after the session wrapper goes out of scope fails.
  // This should fail because the wrapper going out of scope should automatically destroy the
  // session.
  assert!(unsafe { ::fsw_destroy_session(handle) } != ::FSW_OK);
}

#[test]
fn add_path() {
  let session = get_default_session();
  session.add_path("./").unwrap();
}

#[test]
fn add_property() {
  let session = get_default_session();
  session.add_property("test_name", "test_value").unwrap();
}

#[test]
fn set_allow_overflow() {
  let session = get_default_session();
  session.set_allow_overflow(true).unwrap();
}

#[test]
fn set_callback() {
  let session = get_default_session();
  session.set_callback(|_| {
    println!("Hi!");
  }).unwrap();
}

#[test]
#[should_panic]
fn start_empty() {
  get_default_session().start_monitor().unwrap();
}

#[test]
fn start_without_callback() {
  let session = get_default_session();
  session.add_path("./").unwrap();
  assert!(session.start_monitor().is_err());
}

#[test]
fn start_without_path() {
  let session = get_default_session();
  session.set_callback(|_| println!("Hello")).unwrap();
  assert!(session.start_monitor().is_err());
}
