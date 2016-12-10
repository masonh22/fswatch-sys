#![allow(non_camel_case_types)]

extern crate libc;

use libc::{c_int, c_uint, c_void, c_double, c_char};
use std::ops::Drop;
use std::ffi::CString;
use std::path::Path;

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

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
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
  text: String,
  filter_type: FswFilterType,
  case_sensitive: bool,
  extended: bool
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
  path: String,
  evt_time: i64, // FIXME: Tm,
  flags: Vec<FswEventFlag>
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
    let events: Vec<fsw_cevent> = unsafe { Vec::from_raw_parts(events as *mut _, event_num as usize, event_num as usize) };
    let mapped_events = events.iter()
      .map(|x| {
        let path = unsafe { CString::from_raw(x.path as *mut _) }.to_string_lossy().to_string();
        let flags = unsafe { Vec::from_raw_parts(x.flags as *mut _, x.flags_num as usize, x.flags_num as usize) };
        FswCEvent {
          path: path,
          evt_time: x.evt_time,
          flags: flags
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
