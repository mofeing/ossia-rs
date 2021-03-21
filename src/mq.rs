use crate::{ffi, Device, Parameter, Value};

pub struct MessageQueue(pub(crate) ffi::ossia_mq_t);

impl MessageQueue {
    pub fn new(device: Device) -> MessageQueue {
        MessageQueue(unsafe { ffi::ossia_mq_create(device.0) })
    }

    pub fn register(&mut self, param: Parameter) {
        unsafe { ffi::ossia_mq_register(self.0, param.0) }
    }

    pub fn unregister(&mut self, param: Parameter) {
        unsafe { ffi::ossia_mq_unregister(self.0, param.0) }
    }

    pub fn pop(&mut self) -> Option<(Parameter, Value)> {
        let param: *mut ffi::ossia_parameter_t = std::ptr::null_mut();
        let value: *mut ffi::ossia_value_t = std::ptr::null_mut();
        let r = unsafe { ffi::ossia_mq_pop(self.0, param, value) };

        if r == 1 {
            Some((Parameter(unsafe { *param }), Value(unsafe { *value })))
        } else {
            None
        }
    }
}

impl Drop for MessageQueue {
    fn drop(&mut self) {
        unsafe { ffi::ossia_mq_free(self.0) }
    }
}
