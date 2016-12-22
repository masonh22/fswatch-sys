use {ffi, FswSession, FswError, FswMonitorType, FswMonitorFilter, FswFilterType, FswEventFlag};

fn get_default_session() -> FswSession {
  FswSession::default().unwrap()
}

fn create_sample_filter() -> FswMonitorFilter {
  FswMonitorFilter::new("\\w+\\.txt$", FswFilterType::Include, false, false)
}

#[test]
fn create_and_destroy_session() {
  let handle = {
    let session = get_default_session();
    session.handle
  };
  // Check that the handle was created successfully.
  assert!(handle != ffi::FSW_INVALID_HANDLE);
  // Check that trying to destroy the handle after the session wrapper goes out of scope fails.
  // This should fail because the wrapper going out of scope should automatically destroy the
  // session.
  assert!(unsafe { ffi::fsw_destroy_session(handle) } != ffi::FSW_OK);
}

#[test]
fn create_session_from_builder() {
  FswSession::builder()
    .add_path("./")
    .property("test_name", "test_value")
    .overflow(Some(true))
    .monitor(FswMonitorType::SystemDefault)
    .latency(Some(1.0))
    .recursive(Some(true))
    .directory_only(Some(false))
    .follow_symlinks(Some(true))
    .add_event_filter(FswEventFlag::Created)
    .add_filter(create_sample_filter())
    .build_callback(|events| println!("{:#?}", events))
    .unwrap();
}

#[test]
fn start_empty() {
  assert_eq!(Err(FswError::MissingRequiredParameters), get_default_session().start_monitor());
}

#[test]
fn start_without_callback() {
  let session = get_default_session();
  session.add_path("./").unwrap();
  assert_eq!(Err(FswError::MissingRequiredParameters), session.start_monitor());
}

#[test]
fn start_without_path() {
  let session = get_default_session();
  session.set_callback(|_| println!("Hello")).unwrap();
  assert_eq!(Err(FswError::MissingRequiredParameters), session.start_monitor());
}
