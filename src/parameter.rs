use crate::Domain;
use crate::Node;
use crate::{
    ffi::{self, ossia_access_mode, ossia_bounding_mode, ossia_type},
    Value,
};
use enum_repr::EnumRepr;
use libffi::high::Closure2;
use num_enum::TryFromPrimitive;
use std::{
    convert::{TryFrom, TryInto},
    ffi::{c_void, CStr},
    os::raw::{c_char, c_int},
};

#[EnumRepr(type = "ossia_type")]
#[derive(TryFromPrimitive)]
pub enum Type {
    Float = ffi::ossia_type_FLOAT_T,
    Int = ffi::ossia_type_INT_T,
    Vec2f = ffi::ossia_type_VEC2F_T,
    Vec3f = ffi::ossia_type_VEC3F_T,
    Vec4f = ffi::ossia_type_VEC4F_T,
    Impulse = ffi::ossia_type_IMPULSE_T,
    Bool = ffi::ossia_type_BOOL_T,
    String = ffi::ossia_type_STRING_T,
    List = ffi::ossia_type_LIST_T,
    Char = ffi::ossia_type_CHAR_T,
}

#[EnumRepr(type = "ossia_access_mode")]
#[derive(TryFromPrimitive)]
pub enum Access {
    Bi = ffi::ossia_access_mode_BI,
    Get = ffi::ossia_access_mode_GET,
    Set = ffi::ossia_access_mode_SET,
}

#[EnumRepr(type = "ossia_bounding_mode")]
#[derive(TryFromPrimitive)]
pub enum Bounding {
    Free = ffi::ossia_bounding_mode_FREE,
    Clip = ffi::ossia_bounding_mode_CLIP,
    Wrap = ffi::ossia_bounding_mode_WRAP,
    Fold = ffi::ossia_bounding_mode_FOLD,
    Low = ffi::ossia_bounding_mode_LOW,
    High = ffi::ossia_bounding_mode_HIGH,
}

pub struct Parameter(pub(crate) ffi::ossia_node_t);

pub struct ValueCallbackIdx(pub(crate) ffi::ossia_value_callback_idx_t);

impl Parameter {
    pub fn node(&self) -> Node {
        Node(unsafe { ffi::ossia_parameter_get_node(self.0) })
    }

    pub fn set_access_mode(&mut self, am: Access) {
        unsafe { ffi::ossia_parameter_set_access_mode(self.0, am as ossia_access_mode) }
    }

    pub fn get_access_mode(&self) -> Access {
        Access::try_from(unsafe { ffi::ossia_parameter_get_access_mode(self.0) as isize }).unwrap()
    }

    pub fn set_bounding_mode(&mut self, bm: Bounding) {
        unsafe { ffi::ossia_parameter_set_bounding_mode(self.0, bm as ossia_bounding_mode) }
    }

    pub fn get_bounding_mode(&self) -> Bounding {
        Bounding::try_from(unsafe { ffi::ossia_parameter_get_bounding_mode(self.0) as isize })
            .unwrap()
    }

    pub fn set_domain(&mut self, domain: Domain) {
        unsafe { ffi::ossia_parameter_set_domain(self.0, domain.0) }
    }

    pub fn get_domain(&self) -> Domain {
        Domain(unsafe { ffi::ossia_parameter_get_domain(self.0) })
    }

    pub fn set_unit(&mut self, unit: &str) {
        unsafe { ffi::ossia_parameter_set_unit(self.0, unit.as_ptr() as *const c_char) }
    }

    pub fn get_unit(&self) -> &str {
        unsafe { CStr::from_ptr(ffi::ossia_parameter_get_unit(self.0)) }
            .to_str()
            .unwrap()
    }

    pub fn set_muted(&mut self, muted: bool) {
        unsafe { ffi::ossia_parameter_set_muted(self.0, muted as c_int) }
    }

    pub fn get_muted(&self) -> bool {
        unsafe { ffi::ossia_parameter_get_muted(self.0) == 0 }
    }

    pub fn set_disabled(&mut self, disabled: bool) {
        unsafe { ffi::ossia_parameter_set_disabled(self.0, disabled as c_int) }
    }

    pub fn get_disabled(&self) -> bool {
        unsafe { ffi::ossia_parameter_get_disabled(self.0) == 0 }
    }

    pub fn set_critical(&mut self, critical: bool) {
        unsafe { ffi::ossia_parameter_set_critical(self.0, critical as c_int) }
    }

    pub fn get_critical(&self) -> bool {
        unsafe { ffi::ossia_parameter_get_critical(self.0) == 0 }
    }

    pub fn set_repetition_filter(&mut self, repetition_filter: bool) {
        unsafe { ffi::ossia_parameter_set_repetition_filter(self.0, repetition_filter as c_int) }
    }

    pub fn get_repetition_filter(&self) -> bool {
        unsafe { ffi::ossia_parameter_get_repetition_filter(self.0) == 0 }
    }

    pub fn set_value(&mut self, value: Value) {
        unsafe { ffi::ossia_parameter_set_value(self.0, value.0) }
    }

    pub fn get_value(&self) -> Value {
        Value(unsafe { ffi::ossia_parameter_get_value(self.0) })
    }

    pub fn set_listening(&mut self, listening: bool) {
        unsafe { ffi::ossia_parameter_set_listening(self.0, listening as c_int) };
    }

    pub fn add_callback<F>(&mut self, cb: F, ctx: *mut c_void) -> ValueCallbackIdx
    where
        F: Fn(*mut c_void, ffi::ossia_value_t),
    {
        let closure = Closure2::new(&cb);

        ValueCallbackIdx(unsafe {
            ffi::ossia_parameter_add_callback(self.0, Some(*closure.code_ptr()), ctx)
        })
    }

    pub fn push_callback<F>(&mut self, cb: F, ctx: *mut c_void)
    where
        F: Fn(*mut c_void, ffi::ossia_value_t),
    {
        let closure = Closure2::new(&cb);
        unsafe { ffi::ossia_parameter_add_callback(self.0, Some(*closure.code_ptr()), ctx) };
    }

    pub fn rm_callback(&mut self, index: ValueCallbackIdx) {
        unsafe { ffi::ossia_parameter_remove_callback(self.0, index.0) };
    }
}

impl Into<i32> for Parameter {
    fn into(self) -> i32 {
        unsafe { ffi::ossia_parameter_to_int(self.0) }
    }
}

impl Into<f32> for Parameter {
    fn into(self) -> f32 {
        unsafe { ffi::ossia_parameter_to_float(self.0) }
    }
}

impl Into<(f32, f32)> for Parameter {
    fn into(self) -> (f32, f32) {
        let o = unsafe { ffi::ossia_parameter_to_2f(self.0) };
        (o.val[0], o.val[1])
    }
}

impl Into<(f32, f32, f32)> for Parameter {
    fn into(self) -> (f32, f32, f32) {
        let o = unsafe { ffi::ossia_parameter_to_3f(self.0) };
        (o.val[0], o.val[1], o.val[2])
    }
}

impl Into<(f32, f32, f32, f32)> for Parameter {
    fn into(self) -> (f32, f32, f32, f32) {
        let o = unsafe { ffi::ossia_parameter_to_4f(self.0) };
        (o.val[0], o.val[1], o.val[2], o.val[3])
    }
}

impl Into<bool> for Parameter {
    fn into(self) -> bool {
        unsafe { ffi::ossia_parameter_to_bool(self.0) == 0 }
    }
}

impl Into<c_char> for Parameter {
    fn into(self) -> c_char {
        unsafe { ffi::ossia_parameter_to_char(self.0) }
    }
}

impl Into<Vec<u8>> for Parameter {
    fn into(self) -> Vec<u8> {
        let mut ptr: *mut c_char = std::ptr::null_mut();
        let mut size: ffi::size_t = 0;
        unsafe { ffi::ossia_parameter_to_byte_array(self.0, &mut ptr, &mut size) };

        unsafe {
            Vec::from_raw_parts(
                ptr as *mut u8,
                size.try_into().unwrap(),
                size.try_into().unwrap(),
            )
        }
    }
}

impl Into<String> for Parameter {
    fn into(self) -> String {
        unsafe { CStr::from_ptr(ffi::ossia_parameter_to_string(self.0)) }
            .to_str()
            .unwrap()
            .to_owned()
    }
}

impl Into<Vec<Parameter>> for Parameter {
    fn into(self) -> Vec<Parameter> {
        todo!()
    }
}

impl Into<Vec<f32>> for Parameter {
    fn into(self) -> Vec<f32> {
        todo!()
    }
}

impl Into<Vec<i32>> for Parameter {
    fn into(self) -> Vec<i32> {
        todo!()
    }
}

pub trait Push<T> {
    fn push(&mut self, value: T);
}

impl Push<Value> for Parameter {
    fn push(&mut self, value: Value) {
        unsafe { ffi::ossia_parameter_push_value(self.0, value.0) };
    }
}

impl Push<()> for Parameter {
    fn push(&mut self, _: ()) {
        unsafe { ffi::ossia_parameter_push_impulse(self.0) };
    }
}

impl Push<i32> for Parameter {
    fn push(&mut self, value: i32) {
        unsafe { ffi::ossia_parameter_push_i(self.0, value) };
    }
}

impl Push<bool> for Parameter {
    fn push(&mut self, value: bool) {
        unsafe { ffi::ossia_parameter_push_b(self.0, value as c_int) };
    }
}

impl Push<f32> for Parameter {
    fn push(&mut self, value: f32) {
        unsafe { ffi::ossia_parameter_push_f(self.0, value) };
    }
}

impl Push<(f32, f32)> for Parameter {
    fn push(&mut self, value: (f32, f32)) {
        unsafe { ffi::ossia_parameter_push_2f(self.0, value.0, value.1) };
    }
}

impl Push<(f32, f32, f32)> for Parameter {
    fn push(&mut self, value: (f32, f32, f32)) {
        unsafe { ffi::ossia_parameter_push_3f(self.0, value.0, value.1, value.2) };
    }
}

impl Push<(f32, f32, f32, f32)> for Parameter {
    fn push(&mut self, value: (f32, f32, f32, f32)) {
        unsafe { ffi::ossia_parameter_push_4f(self.0, value.0, value.1, value.2, value.3) };
    }
}

impl Push<c_char> for Parameter {
    fn push(&mut self, value: c_char) {
        unsafe { ffi::ossia_parameter_push_c(self.0, value) };
    }
}

impl Push<&str> for Parameter {
    fn push(&mut self, value: &str) {
        // ffi::ossia_parameter_push_s
        unsafe {
            ffi::ossia_parameter_push_cn(
                self.0,
                value.as_ptr() as *const c_char,
                value.len() as ffi::size_t,
            )
        };
    }
}

impl Push<&[i32]> for Parameter {
    fn push(&mut self, value: &[i32]) {
        unsafe { ffi::ossia_parameter_push_in(self.0, value.as_ptr(), value.len() as ffi::size_t) };
    }
}

impl Push<&[f32]> for Parameter {
    fn push(&mut self, value: &[f32]) {
        unsafe { ffi::ossia_parameter_push_fn(self.0, value.as_ptr(), value.len() as ffi::size_t) };
    }
}

impl Push<&[Value]> for Parameter {
    fn push(&mut self, value: &[Value]) {
        unsafe {
            ffi::ossia_parameter_push_list(
                self.0,
                value.as_ptr() as *const ffi::ossia_value_t,
                value.len() as ffi::size_t,
            )
        };
    }
}

impl Parameter {
    pub fn fetch(&self) -> Value {
        Value(unsafe { ffi::ossia_parameter_fetch_value(self.0) })
    }
}
