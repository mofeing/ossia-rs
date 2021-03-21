use crate::ffi;
use crate::Value;
use std::{ops::Range, os::raw::c_char};

pub struct Domain(pub(crate) ffi::ossia_domain_t);

impl Domain {
    pub fn new() -> Self {
        Self(unsafe { ffi::ossia_domain_create() })
    }

    pub fn min(&self) -> Value {
        Value(unsafe { ffi::ossia_domain_get_min(self.0) })
    }

    pub fn max(&self) -> Value {
        Value(unsafe { ffi::ossia_domain_get_max(self.0) })
    }

    pub fn values(&self) -> Vec<Value> {
        let values: *mut *mut ffi::ossia_value_t = std::ptr::null_mut();
        let mut n: ffi::size_t = 0;
        unsafe {
            ffi::ossia_domain_get_values(self.0, values, &mut n);
            Vec::from_raw_parts(values as *mut Value, n as usize, n as usize)
        }
    }

    pub fn set_min(&self, value: Value) {
        unsafe { ffi::ossia_domain_set_min(self.0, value.0) };
    }

    pub fn set_max(&self, value: Value) {
        unsafe { ffi::ossia_domain_set_max(self.0, value.0) };
    }

    pub fn set_values(&self, values: &[Value]) {
        unsafe {
            ffi::ossia_domain_set_values(
                self.0,
                values.as_ptr() as *const ffi::ossia_value_t,
                values.len() as ffi::size_t,
            )
        }
    }
}

impl Drop for Domain {
    fn drop(&mut self) {
        unsafe { ffi::ossia_domain_free(self.0) };
    }
}

impl From<Range<Value>> for Domain {
    fn from(range: Range<Value>) -> Self {
        Self(unsafe { ffi::ossia_domain_make_min_max(range.start.0, range.end.0) })
    }
}

impl From<&[&str]> for Domain {
    fn from(set: &[&str]) -> Self {
        Self(unsafe {
            ffi::ossia_domain_make_string_set(
                set.as_ptr() as *mut *const c_char,
                set.len() as ffi::size_t,
            )
        })
    }
}

impl From<&[i32]> for Domain {
    fn from(set: &[i32]) -> Self {
        Self(unsafe { ffi::ossia_domain_make_int_set(set.as_ptr(), set.len() as ffi::size_t) })
    }
}

impl From<&[f32]> for Domain {
    fn from(set: &[f32]) -> Self {
        Self(unsafe { ffi::ossia_domain_make_float_set(set.as_ptr(), set.len() as ffi::size_t) })
    }
}

impl From<&[Value]> for Domain {
    fn from(set: &[Value]) -> Self {
        Self(unsafe {
            ffi::ossia_domain_make_value_set(
                set.as_ptr() as *const ffi::ossia_value_t,
                set.len() as ffi::size_t,
            )
        })
    }
}
