use crate::{ffi, Device, Parameter, Type, Value};
use std::{
    ffi::{c_void, CStr},
    ops::Range,
    os::raw::c_char,
};

pub struct Node(pub(crate) ffi::ossia_node_t);

pub struct NodeCallbackId(ffi::ossia_node_callback_idx_t);

impl Node {
    pub fn new(&self, path: &str) -> Self {
        Node(unsafe { ffi::ossia_node_create(self.0, path.as_ptr() as *const c_char) })
    }

    pub fn from_pattern(&self, pattern: &str) -> Vec<Node> {
        let mut data = std::ptr::null_mut();
        let mut size = 0;
        unsafe {
            ffi::ossia_node_create_pattern(
                self.0,
                pattern.as_ptr() as *const c_char,
                &mut data,
                &mut size,
            );
        }
        todo!()
    }

    pub fn find(&self, path: &str) -> Self {
        Node(unsafe { ffi::ossia_node_find(self.0, path.as_ptr() as *const c_char) })
    }

    pub fn find_pattern(&self, pattern: &str) -> Vec<Node> {
        let mut size: ffi::size_t = 0;
        let mut ptr: *mut ffi::ossia_node_t = std::ptr::null_mut();

        unsafe {
            ffi::ossia_node_find_pattern(
                self.0,
                pattern.as_ptr() as *const c_char,
                &mut ptr,
                &mut size,
            );
            Vec::from_raw_parts(ptr as *mut Node, size as usize, size as usize)
        }
    }

    pub fn add_child(&self, name: &str) -> Node {
        Node(unsafe { ffi::ossia_node_add_child(self.0, name.as_ptr() as *const c_char) })
    }

    pub fn rm_child(&self, name: Node) {
        unsafe { ffi::ossia_node_remove_child(self.0, name.0) }
    }

    pub fn name(&self) -> &str {
        unsafe { CStr::from_ptr(ffi::ossia_node_get_name(self.0)) }
            .to_str()
            .unwrap()
    }

    pub fn device(&self) -> Device {
        Device(unsafe { ffi::ossia_node_get_device(self.0) })
    }

    pub fn num_children(&self) -> i32 {
        unsafe { ffi::ossia_node_child_size(self.0) }
    }

    pub fn child(&self, idx: i32) -> Node {
        Node(unsafe { ffi::ossia_node_get_child(self.0, idx) })
    }

    pub fn find_child(&self, name: &str) -> Node {
        Node(unsafe { ffi::ossia_node_find_child(self.0, name.as_ptr() as *const c_char) })
    }

    pub fn add_parameter(&self, typ: Type) {
        // ossia_node_create_parameter or ossia_create_parameter
        todo!()
    }

    pub fn parameter(&self) -> Parameter {
        Parameter(unsafe { ffi::ossia_node_get_parameter(self.0) })
    }

    pub fn rm_parameter(&self) {
        unsafe { ffi::ossia_node_remove_parameter(self.0) }
    }

    pub fn add_callback<F>(&mut self, cb: F, ctx: *mut c_void) -> NodeCallbackId
    where
        F: Fn(*mut c_void, Node),
    {
        todo!()
    }

    pub fn description(&self) -> &str {
        unsafe { CStr::from_ptr(ffi::ossia_node_get_description(self.0)) }
            .to_str()
            .unwrap()
    }

    pub fn set_description(&mut self, desc: &str) {
        unsafe { ffi::ossia_node_set_description(self.0, desc.as_ptr() as *const c_char) }
    }

    pub fn extended_type(&self) -> &str {
        todo!()
    }

    pub fn set_extended_type(&mut self, ext_type: &str) {
        todo!()
    }

    pub fn tags(&self) -> Vec<&str> {
        todo!()
    }

    pub fn set_tags(&mut self, tags: &[&str]) {
        todo!()
    }

    // pub fn tags_free

    pub fn hidden(&self) -> bool {
        todo!()
    }

    pub fn set_hidden(&mut self, hidden: bool) {
        todo!()
    }

    pub fn refresh_rate(&self) -> i32 {
        todo!()
    }

    pub fn set_refresh_rate(&mut self, rate: i32) {
        todo!()
    }

    pub fn unset_refresh_rate(&mut self) {
        todo!()
    }

    pub fn priority(&self) -> f32 {
        todo!()
    }

    pub fn set_priority(&mut self, priority: f32) {
        todo!()
    }

    pub fn unset_priority(&mut self) {
        todo!()
    }

    pub fn step_size(&self) -> f64 {
        todo!()
    }

    pub fn set_step_size(&mut self, step_size: f64) {
        todo!()
    }

    pub fn unset_step_size(&mut self) {
        todo!()
    }

    pub fn instance_bounds(&self) -> Range<i32> {
        todo!()
    }

    pub fn set_instance_bounds(&mut self, instance_bounds: Range<i32>) {
        todo!()
    }

    pub fn unset_instance_bounds(&mut self) {
        todo!()
    }

    pub fn default_value(&self) -> Value {
        todo!()
    }

    pub fn set_default_value(&mut self, value: Value) {
        todo!()
    }
}
