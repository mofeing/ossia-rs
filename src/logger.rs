use std::os::raw::c_char;

use crate::ffi::{self, log_level};
use enum_repr::EnumRepr;

#[EnumRepr(type = "log_level")]
pub enum LogLevel {
    Trace = ffi::log_level_trace,
    Debug = ffi::log_level_debug,
    Info = ffi::log_level_info,
    Warn = ffi::log_level_warn,
    Error = ffi::log_level_error,
    Critical = ffi::log_level_critical,
    Off = ffi::log_level_off,
}

pub struct Logger(ffi::ossia_logger_t);

impl Logger {
    pub fn new(host: &str, app: &str) -> Logger {
        Self(unsafe {
            ffi::ossia_logger_create(
                host.as_ptr() as *const c_char,
                app.as_ptr() as *const c_char,
            )
        })
    }

    pub fn heatbeat(&self, pid: i32, cmdline: &str) {
        unsafe { ffi::ossia_logger_init_heartbeat(self.0, pid, cmdline.as_ptr() as *const c_char) };
    }

    pub fn level(&mut self, lvl: LogLevel) {
        unsafe { ffi::ossia_logger_set_level(self.0, lvl as log_level) };
    }

    pub fn log(&mut self, lvl: LogLevel, message: &str) {
        unsafe { ffi::ossia_log(self.0, lvl as log_level, message.as_ptr() as *const c_char) };
    }
}

impl Drop for Logger {
    fn drop(&mut self) {
        unsafe { ffi::ossia_logger_free(self.0) };
    }
}
