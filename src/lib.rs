#![allow(non_camel_case_types)]

extern crate libc;

use libc::{c_int, c_uint, c_void, c_double, c_char};
use std::ops::Drop;
use std::ffi::{CString, CStr};
use std::path::{Path, PathBuf};
use std::collections::HashMap;

#[cfg(test)]
mod test;

type FswResult<T> = Result<T, FswError>;

type FSW_STATUS = c_int;
// type FSW_HANDLE = c_uint;
// Fun story here. FSW_HANDLE is defined as an unsigned int, but it can return a signed int,
// FSW_INVALID_HANDLE, which is -1. So, we're calling FSW_HANDLE c_int, not c_unit.
type FSW_HANDLE = c_int;
type FSW_CEVENT_CALLBACK = extern fn(events: *const fsw_cevent, event_num: c_uint, data: *mut c_void);

const FSW_INVALID_HANDLE: FSW_HANDLE = -1;

const FSW_OK: FSW_STATUS = 0;
const FSW_ERR_UNKNOWN_ERROR: FSW_STATUS = (1 << 0);
const FSW_ERR_SESSION_UNKNOWN: FSW_STATUS = (1 << 1);
const FSW_ERR_MONITOR_ALREADY_EXISTS: FSW_STATUS = (1 << 2);
const FSW_ERR_MEMORY: FSW_STATUS = (1 << 3);
const FSW_ERR_UNKNOWN_MONITOR_TYPE: FSW_STATUS = (1 << 4);
const FSW_ERR_CALLBACK_NOT_SET: FSW_STATUS = (1 << 5);
const FSW_ERR_PATHS_NOT_SET: FSW_STATUS = (1 << 6);
const FSW_ERR_MISSING_CONTEXT: FSW_STATUS = (1 << 7);
const FSW_ERR_INVALID_PATH: FSW_STATUS = (1 << 8);
const FSW_ERR_INVALID_CALLBACK: FSW_STATUS = (1 << 9);
const FSW_ERR_INVALID_LATENCY: FSW_STATUS = (1 << 10);
const FSW_ERR_INVALID_REGEX: FSW_STATUS = (1 << 11);
const FSW_ERR_MONITOR_ALREADY_RUNNING: FSW_STATUS = (1 << 12);
const FSW_ERR_UNKNOWN_VALUE: FSW_STATUS = (1 << 13);
const FSW_ERR_INVALID_PROPERTY: FSW_STATUS = (1 << 14);

#[derive(Debug)]
pub enum FswError {
  FromFSW(FswStatus),
  NulError(std::ffi::NulError)
}

#[derive(Debug, PartialEq)]
pub enum FswStatus {
  Ok,
  UnknownError,
  SessionUnknown,
  MonitorAlreadyExists,
  Memory,
  UnknownMonitorType,
  CallbackNotSet,
  PathsNotSet,
  MissingContext,
  InvalidPath,
  InvalidCallback,
  InvalidLatency,
  InvalidRegex,
  MonitorAlreadyRunning,
  UnknownValue,
  InvalidProprety
}

impl From<FSW_STATUS> for FswStatus {
  fn from(status: FSW_STATUS) -> FswStatus {
    match status {
      FSW_OK => FswStatus::Ok,
      FSW_ERR_UNKNOWN_ERROR => FswStatus::UnknownError,
      FSW_ERR_SESSION_UNKNOWN => FswStatus::SessionUnknown,
      FSW_ERR_MONITOR_ALREADY_EXISTS => FswStatus::MonitorAlreadyExists,
      FSW_ERR_MEMORY => FswStatus::Memory,
      FSW_ERR_UNKNOWN_MONITOR_TYPE => FswStatus::UnknownMonitorType,
      FSW_ERR_CALLBACK_NOT_SET => FswStatus::CallbackNotSet,
      FSW_ERR_PATHS_NOT_SET => FswStatus::PathsNotSet,
      FSW_ERR_MISSING_CONTEXT => FswStatus::MissingContext,
      FSW_ERR_INVALID_PATH => FswStatus::InvalidPath,
      FSW_ERR_INVALID_CALLBACK => FswStatus::InvalidCallback,
      FSW_ERR_INVALID_LATENCY => FswStatus::InvalidLatency,
      FSW_ERR_INVALID_REGEX => FswStatus::InvalidRegex,
      FSW_ERR_MONITOR_ALREADY_RUNNING => FswStatus::MonitorAlreadyRunning,
      FSW_ERR_UNKNOWN_VALUE => FswStatus::UnknownValue,
      FSW_ERR_INVALID_PROPERTY => FswStatus::InvalidProprety,
      _ => FswStatus::UnknownError
    }
  }
}

#[derive(Debug, PartialEq)]
#[repr(C)]
pub enum FswMonitorType {
  SystemDefaultMonitorType,
  FSEventsMonitorType,
  KQueueMonitorType,
  INotifyMonitorType,
  WindowsMonitorType,
  PollMonitorType,
  FenMonitorType
}

#[derive(Debug)]
#[repr(C)]
struct fsw_event_type_filter {
  flag: FswEventFlag
}

#[derive(Debug, PartialEq, Clone)]
#[repr(u32)]
pub enum FswEventFlag {
  NoOp = 0,
  PlatformSpecific = (1 << 0),
  Created = (1 << 1),
  Updated = (1 << 2),
  Removed = (1 << 3),
  Renamed = (1 << 4),
  OwnerModified = (1 << 5),
  AttributeModified = (1 << 6),
  MovedFrom = (1 << 7),
  MovedTo = (1 << 8),
  IsFile = (1 << 9),
  IsDir = (1 << 10),
  IsSymLink = (1 << 11),
  Link = (1 << 12),
  Overflow = (1 << 13)
}

#[derive(Debug)]
#[repr(C)]
struct fsw_cmonitor_filter {
  text: *const c_char,
  filter_type: FswFilterType,
  case_sensitive: bool,
  extended: bool
}

#[derive(Debug)]
pub struct FswCMonitorFilter {
  pub text: String,
  pub filter_type: FswFilterType,
  pub case_sensitive: bool,
  pub extended: bool
}

impl FswCMonitorFilter {
  pub fn new(text: String, filter_type: FswFilterType, case_sensitive: bool, extended: bool) -> Self {
    FswCMonitorFilter {
      text: text,
      filter_type: filter_type,
      case_sensitive: case_sensitive,
      extended: extended
    }
  }
}

#[derive(Debug)]
#[repr(C)]
pub enum FswFilterType {
  Include,
  Exclude
}

#[derive(Debug)]
#[repr(C)]
struct fsw_cevent {
  path: *const c_char,
  evt_time: libc::time_t,
  flags: *const FswEventFlag,
  flags_num: c_uint
}

#[derive(Debug)]
pub struct FswCEvent {
  pub path: String,
  pub evt_time: i64, // FIXME: Tm,
  pub flags: Vec<FswEventFlag>
}

#[link(name = "fswatch")]
extern "C" {
  fn fsw_init_library() -> FSW_STATUS;

  fn fsw_init_session(monitor_type: FswMonitorType) -> FSW_HANDLE;

  fn fsw_add_path(handle: FSW_HANDLE, path: *const c_char) -> FSW_STATUS;

  fn fsw_add_property(handle: FSW_HANDLE, name: *const c_char, value: *const c_char) -> FSW_STATUS;

  fn fsw_set_allow_overflow(handle: FSW_HANDLE, allow_overflow: bool) -> FSW_STATUS;

  fn fsw_set_callback(handle: FSW_HANDLE, callback: FSW_CEVENT_CALLBACK, data: *const c_void) -> FSW_STATUS;

  fn fsw_set_latency(handle: FSW_HANDLE, latency: c_double) -> FSW_STATUS;

  fn fsw_set_recursive(handle: FSW_HANDLE, recursive: bool) -> FSW_STATUS;

  fn fsw_set_directory_only(handle: FSW_HANDLE, directory_only: bool) -> FSW_STATUS;

  fn fsw_set_follow_symlinks(handle: FSW_HANDLE, follow_symlinks: bool) -> FSW_STATUS;

  fn fsw_add_event_type_filter(handle: FSW_HANDLE, event_type: fsw_event_type_filter) -> FSW_STATUS;

  fn fsw_add_filter(handle: FSW_HANDLE, filter: fsw_cmonitor_filter) -> FSW_STATUS;

  fn fsw_start_monitor(handle: FSW_HANDLE) -> FSW_STATUS;

  fn fsw_destroy_session(handle: FSW_HANDLE) -> FSW_STATUS;

  fn fsw_last_error() -> FSW_STATUS;

  fn fsw_is_verbose() -> bool;

  fn fsw_set_verbose(verbose: bool);
}

pub struct Fsw;

impl Fsw {
  pub fn init_library() -> FswResult<()> {
    let result = unsafe { fsw_init_library() };
    FswSession::map_result((), result)
  }

  pub fn last_error() -> FswStatus {
    let result = unsafe { fsw_last_error() };
    result.into()
  }

  pub fn verbose() -> bool {
    unsafe { fsw_is_verbose() }
  }

  pub fn set_verbose(verbose: bool) {
    unsafe { fsw_set_verbose(verbose) };
  }
}

pub struct FswSessionBuilder<F> {
  paths: Vec<PathBuf>,
  callback: Box<F>,
  monitor_type: FswMonitorType,
  properties: HashMap<String, String>,
  overflow: Option<bool>,
  latency: Option<c_double>,
  recursive: Option<bool>,
  directory_only: Option<bool>,
  follow_symlinks: Option<bool>,
  event_type_filters: Vec<FswEventFlag>,
  filters: Vec<FswCMonitorFilter>
}

impl<F> FswSessionBuilder<F>
  where F: Fn(Vec<FswCEvent>) + 'static
{
  pub fn new<P>(paths: Vec<P>, callback: F) -> Self
    where P: AsRef<Path>
  {
    let paths = paths.iter().map(|x| x.as_ref().to_owned()).collect();
    FswSessionBuilder {
      paths: paths,
      callback: Box::new(callback),
      monitor_type: FswMonitorType::SystemDefaultMonitorType,
      properties: Default::default(),
      overflow: Default::default(),
      latency: Default::default(),
      recursive: Default::default(),
      directory_only: Default::default(),
      follow_symlinks: Default::default(),
      event_type_filters: Default::default(),
      filters: Default::default()
    }
  }

  pub fn build(self) -> FswResult<FswSession> {
    let session = FswSession::new(self.monitor_type)?;
    for path in self.paths {
      session.add_path(path)?;
    }
    session.set_callback(*self.callback)?;
    for (name, value) in self.properties {
      session.add_property(&name, &value)?;
    }
    if let Some(overflow) = self.overflow {
      session.set_allow_overflow(overflow)?;
    }
    if let Some(latency) = self.latency {
      session.set_latency(latency)?;
    }
    if let Some(recursive) = self.recursive {
      session.set_recursive(recursive)?;
    }
    if let Some(directory_only) = self.directory_only {
      session.set_directory_only(directory_only)?;
    }
    if let Some(follow_symlinks) = self.follow_symlinks {
      session.set_follow_symlinks(follow_symlinks)?;
    }
    for event_type in self.event_type_filters {
      session.add_event_type_filter(event_type)?;
    }
    for filter in self.filters {
      session.add_filter(filter)?;
    }
    Ok(session)
  }

  pub fn monitor(mut self, monitor: FswMonitorType) -> Self {
    self.monitor_type = monitor;
    self
  }

  pub fn property(mut self, name: &str, value: &str) -> Self {
    self.properties.insert(name.to_owned(), value.to_owned());
    self
  }

  pub fn overflow(mut self, overflow: Option<bool>) -> Self {
    self.overflow = overflow;
    self
  }

  pub fn latency(mut self, latency: Option<c_double>) -> Self {
    self.latency = latency;
    self
  }

  pub fn recursive(mut self, recursive: Option<bool>) -> Self {
    self.recursive = recursive;
    self
  }

  pub fn directory_only(mut self, directory_only: Option<bool>) -> Self {
    self.directory_only = directory_only;
    self
  }

  pub fn follow_symlinks(mut self, follow_symlinks: Option<bool>) -> Self {
    self.follow_symlinks = follow_symlinks;
    self
  }

  pub fn event_filter(mut self, filter_type: FswEventFlag) -> Self {
    self.event_type_filters.push(filter_type);
    self
  }

  pub fn filter(mut self, filter: FswCMonitorFilter) -> Self {
    self.filters.push(filter);
    self
  }
}

pub struct FswSession {
  handle: FSW_HANDLE
}

impl FswSession {
  pub fn new(monitor_type: FswMonitorType) -> FswResult<FswSession> {
    let handle = unsafe { fsw_init_session(monitor_type) };
    if handle == FSW_INVALID_HANDLE {
      return Err(FswError::FromFSW(FswStatus::UnknownError));
    }
    Ok(FswSession {
      handle: handle
    })
  }

  pub fn default() -> FswResult<FswSession> {
    FswSession::new(FswMonitorType::SystemDefaultMonitorType)
  }

  fn map_result<T>(ret: T, result: FSW_STATUS) -> Result<T, FswError> {
    let result: FswStatus = result.into();
    match result {
      FswStatus::Ok => Ok(ret),
      _ => Err(FswError::FromFSW(result))
    }
  }

  pub fn add_path<T: AsRef<Path>>(&self, path: T) -> FswResult<()> {
    let path = path.as_ref().to_string_lossy().into_owned();
    let c_path = CString::new(path).map_err(|x| FswError::NulError(x))?;
    let result = unsafe { fsw_add_path(self.handle, c_path.as_ptr()) };
    FswSession::map_result((), result)
  }

  pub fn add_property(&self, name: &str, value: &str) -> FswResult<()> {
    let c_name = CString::new(name).map_err(|x| FswError::NulError(x))?;
    let c_value = CString::new(value).map_err(|x| FswError::NulError(x))?;
    let result = unsafe { fsw_add_property(self.handle, c_name.as_ptr(), c_value.as_ptr()) };
    FswSession::map_result((), result)
  }

  pub fn set_allow_overflow(&self, allow_overflow: bool) -> FswResult<()> {
    let result = unsafe { fsw_set_allow_overflow(self.handle, allow_overflow) };
    FswSession::map_result((), result)
  }

  extern fn callback_wrapper(events: *const fsw_cevent, event_num: c_uint, data: *mut c_void) {
    let events: &[fsw_cevent] = unsafe { std::slice::from_raw_parts(events, event_num as usize) };
    let mapped_events = events.iter()
      .map(|x| {
        let path = unsafe { CStr::from_ptr(x.path) }.to_string_lossy().to_string();
        let flags = unsafe { std::slice::from_raw_parts(x.flags, x.flags_num as usize) };
        FswCEvent {
          path: path,
          evt_time: x.evt_time,
          flags: flags.to_vec()
        }
      })
      .collect();
    let closure: &Box<Fn(Vec<FswCEvent>) + 'static> = unsafe { std::mem::transmute(data) };
    closure(mapped_events);
  }

  pub fn set_callback<F>(&self, callback: F) -> FswResult<()>
    where F: Fn(Vec<FswCEvent>) + 'static
  {
    let cb: Box<Box<Fn(Vec<FswCEvent>) + 'static>> = Box::new(Box::new(callback));
    let raw = Box::into_raw(cb) as *mut _;
    let result = unsafe { fsw_set_callback(self.handle, FswSession::callback_wrapper, raw) };
    FswSession::map_result((), result)
  }

  pub fn set_latency(&self, latency: c_double) -> FswResult<()> {
    let result = unsafe { fsw_set_latency(self.handle, latency) };
    FswSession::map_result((), result)
  }

  pub fn set_recursive(&self, recursive: bool) -> FswResult<()> {
    let result = unsafe { fsw_set_recursive(self.handle, recursive) };
    FswSession::map_result((), result)
  }

  pub fn set_directory_only(&self, directory_only: bool) -> FswResult<()> {
    let result = unsafe { fsw_set_directory_only(self.handle, directory_only) };
    FswSession::map_result((), result)
  }

  pub fn set_follow_symlinks(&self, follow_symlinks: bool) -> FswResult<()> {
    let result = unsafe { fsw_set_follow_symlinks(self.handle, follow_symlinks) };
    FswSession::map_result((), result)
  }

  pub fn add_event_type_filter(&self, event_type: FswEventFlag) -> FswResult<()> {
    let filter = fsw_event_type_filter {
      flag: event_type
    };
    let result = unsafe { fsw_add_event_type_filter(self.handle, filter) };
    FswSession::map_result((), result)
  }

  pub fn add_filter(&self, filter: FswCMonitorFilter) -> FswResult<()> {
    let c_text = CString::new(filter.text).map_err(|x| FswError::NulError(x))?;
    let c_filter = fsw_cmonitor_filter {
      text: c_text.as_ptr(),
      filter_type: filter.filter_type,
      case_sensitive: filter.case_sensitive,
      extended: filter.extended
    };
    let result = unsafe { fsw_add_filter(self.handle, c_filter) };
    FswSession::map_result((), result)
  }

  pub fn start_monitor(&self) -> FswResult<()> {
    let result = unsafe { fsw_start_monitor(self.handle) };
    FswSession::map_result((), result)
  }

  pub fn destroy_session(&self) -> FswResult<()> {
    let result = unsafe { fsw_destroy_session(self.handle) };
    FswSession::map_result((), result)
  }
}

impl Drop for FswSession {
  fn drop(&mut self) {
    unsafe {
      fsw_destroy_session(self.handle);
    }
  }
}
