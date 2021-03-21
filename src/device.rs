use crate::ffi;
use crate::Node;
use crate::Protocol;
use libffi::high::*;
use std::{
    ffi::{c_void, CStr},
    os::raw::c_char,
};

pub struct Device(pub(crate) ffi::ossia_device_t);

pub enum NodeCallbackId {
    NodeCreated(ffi::ossia_node_callback_idx_t),
    NodeRemoving(ffi::ossia_node_callback_idx_t),
    ParameterDeleting(ffi::ossia_parameter_callback_idx_t),
}

impl Device {
    pub fn new(protocol: Protocol, name: &str) -> Self {
        Self(unsafe { ffi::ossia_device_create(protocol.0, name.as_ptr() as *const c_char) })
    }

    pub fn reset() {
        unsafe {
            ffi::ossia_device_reset_static();
        }
    }

    pub fn update_namespace(&mut self) {
        // TODO what is the semantic of the return value?
        let _err = unsafe { ffi::ossia_device_update_namespace(self.0) };
    }

    pub fn root(&self) -> Node {
        Node(unsafe { ffi::ossia_device_get_root_node(self.0) })
    }

    pub fn name(&self) -> &str {
        unsafe { CStr::from_ptr(ffi::ossia_device_get_name(self.0)) }
            .to_str()
            .unwrap()
    }

    pub fn on_node_created<F>(&mut self, cb: F, ctx: *mut c_void) -> NodeCallbackId
    where
        F: Fn(*mut c_void, ffi::ossia_node_t),
    {
        let closure = Closure2::new(&cb);

        NodeCallbackId::NodeCreated(unsafe {
            ffi::ossia_device_add_node_created_callback(self.0, Some(*closure.code_ptr()), ctx)
        })
    }

    pub fn on_node_removing<F>(&mut self, cb: F, ctx: *mut c_void) -> NodeCallbackId
    where
        F: Fn(*mut c_void, ffi::ossia_node_t),
    {
        let closure = Closure2::new(&cb);

        NodeCallbackId::NodeRemoving(unsafe {
            ffi::ossia_device_add_node_removing_callback(self.0, Some(*closure.code_ptr()), ctx)
        })
    }

    pub fn on_parameter_deleting<F>(&mut self, cb: F, ctx: *mut c_void) -> NodeCallbackId
    where
        F: Fn(*mut c_void, ffi::ossia_parameter_t),
    {
        let closure = Closure2::new(&cb);

        NodeCallbackId::ParameterDeleting(unsafe {
            ffi::ossia_device_add_parameter_deleting_callback(
                self.0,
                Some(*closure.code_ptr()),
                ctx,
            )
        })
    }

    pub fn rm_callback(&mut self, id: NodeCallbackId) {
        match id {
            NodeCallbackId::NodeCreated(x) => unsafe {
                ffi::ossia_device_remove_node_created_callback(self.0, x);
            },
            NodeCallbackId::NodeRemoving(x) => unsafe {
                ffi::ossia_device_remove_node_removing_callback(self.0, x);
            },
            NodeCallbackId::ParameterDeleting(x) => unsafe {
                ffi::ossia_device_remove_parameter_deleting_callback(self.0, x);
            },
        }
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        unsafe {
            ffi::ossia_device_free(self.0);
        }
    }
}
