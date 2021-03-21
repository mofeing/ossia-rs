use crate::{ffi, Type};
use std::{
    convert::{TryFrom, TryInto},
    ffi::CStr,
    os::raw::c_char,
};

pub struct Value(pub(crate) ffi::ossia_value_t);

impl Into<i32> for Value {
    fn into(self) -> i32 {
        unsafe { ffi::ossia_value_to_int(self.0) }
    }
}

impl Into<f32> for Value {
    fn into(self) -> f32 {
        unsafe { ffi::ossia_value_to_float(self.0) }
    }
}

impl Into<(f32, f32)> for Value {
    fn into(self) -> (f32, f32) {
        let o = unsafe { ffi::ossia_value_to_2f(self.0) };
        (o.val[0], o.val[1])
    }
}

impl Into<(f32, f32, f32)> for Value {
    fn into(self) -> (f32, f32, f32) {
        let o = unsafe { ffi::ossia_value_to_3f(self.0) };
        (o.val[0], o.val[1], o.val[2])
    }
}

impl Into<(f32, f32, f32, f32)> for Value {
    fn into(self) -> (f32, f32, f32, f32) {
        let o = unsafe { ffi::ossia_value_to_4f(self.0) };
        (o.val[0], o.val[1], o.val[2], o.val[3])
    }
}

impl Into<bool> for Value {
    fn into(self) -> bool {
        unsafe { ffi::ossia_value_to_bool(self.0) == 0 }
    }
}

impl Into<c_char> for Value {
    fn into(self) -> c_char {
        unsafe { ffi::ossia_value_to_char(self.0) }
    }
}

impl Into<Vec<u8>> for Value {
    fn into(self) -> Vec<u8> {
        let mut ptr: *mut c_char = std::ptr::null_mut();
        let mut size: ffi::size_t = 0;
        unsafe { ffi::ossia_value_to_byte_array(self.0, &mut ptr, &mut size) };

        unsafe {
            Vec::from_raw_parts(
                ptr as *mut u8,
                size.try_into().unwrap(),
                size.try_into().unwrap(),
            )
        }
    }
}

impl Into<String> for Value {
    fn into(self) -> String {
        unsafe { CStr::from_ptr(ffi::ossia_value_to_string(self.0)) }
            .to_str()
            .unwrap()
            .to_owned()
    }
}

impl Into<Vec<Value>> for Value {
    fn into(self) -> Vec<Value> {
        todo!()
    }
}

impl Into<Type> for Value {
    fn into(self) -> Type {
        Type::try_from(unsafe { ffi::ossia_value_get_type(self.0) as isize }).unwrap()
    }
}

impl From<()> for Value {
    fn from(_: ()) -> Value {
        Value(unsafe { ffi::ossia_value_create_impulse() })
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Value {
        Value(unsafe { ffi::ossia_value_create_int(value) })
    }
}

impl From<f32> for Value {
    fn from(value: f32) -> Value {
        Value(unsafe { ffi::ossia_value_create_float(value) })
    }
}

impl From<(f32, f32)> for Value {
    fn from(value: (f32, f32)) -> Value {
        Value(unsafe { ffi::ossia_value_create_2f(value.0, value.1) })
    }
}

impl From<(f32, f32, f32)> for Value {
    fn from(value: (f32, f32, f32)) -> Value {
        Value(unsafe { ffi::ossia_value_create_3f(value.0, value.1, value.2) })
    }
}

impl From<(f32, f32, f32, f32)> for Value {
    fn from(value: (f32, f32, f32, f32)) -> Value {
        Value(unsafe { ffi::ossia_value_create_4f(value.0, value.1, value.2, value.3) })
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Value {
        Value(unsafe { ffi::ossia_value_create_bool(value as i32) })
    }
}

impl From<c_char> for Value {
    fn from(value: c_char) -> Value {
        Value(unsafe { ffi::ossia_value_create_char(value) })
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Value {
        Value(unsafe { ffi::ossia_value_create_string(value.as_ptr() as *const c_char) })
    }
}

impl From<&[u8]> for Value {
    fn from(value: &[u8]) -> Value {
        Value(unsafe {
            ffi::ossia_value_create_byte_array(
                value.as_ptr() as *const c_char,
                value.len() as ffi::size_t,
            )
        })
    }
}

impl From<&[Value]> for Value {
    fn from(value: &[Value]) -> Value {
        Value(unsafe {
            ffi::ossia_value_create_list(
                value.as_ptr() as *const *mut ffi::ossia_value,
                value.len() as ffi::size_t,
            )
        })
    }
}

impl From<&[i32]> for Value {
    fn from(value: &[i32]) -> Value {
        Value(unsafe { ffi::ossia_value_create_in(value.as_ptr(), value.len() as ffi::size_t) })
    }
}

impl From<&[f32]> for Value {
    fn from(value: &[f32]) -> Value {
        Value(unsafe { ffi::ossia_value_create_fn(value.as_ptr(), value.len() as ffi::size_t) })
    }
}

impl Drop for Value {
    fn drop(&mut self) {
        unsafe { ffi::ossia_value_free(self.0) };
    }
}
