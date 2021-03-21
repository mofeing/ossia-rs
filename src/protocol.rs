#![allow(dead_code)]

use std::os::raw::c_char;

use crate::ffi;

pub struct Protocol(pub(crate) ffi::ossia_protocol_t);

impl Protocol {
    pub fn multiplex(local: Protocol, other: Protocol) -> Protocol {
        let protocol = Protocol(unsafe { ffi::ossia_protocol_multiplex_create() });
        unsafe { ffi::ossia_protocol_multiplex_expose_to(local.0, other.0) };

        protocol
    }

    pub fn osc(ip: &str, remote_port: i32, local_port: i32) -> Protocol {
        Protocol(unsafe {
            ffi::ossia_protocol_osc_create(ip.as_ptr() as *const c_char, remote_port, local_port)
        })
    }

    pub fn minuit(local_name: &str, ip: &str, remote_port: i32, local_port: i32) -> Protocol {
        Protocol(unsafe {
            ffi::ossia_protocol_minuit_create(
                local_name.as_ptr() as *const c_char,
                ip.as_ptr() as *const c_char,
                remote_port,
                local_port,
            )
        })
    }

    pub fn oscquery_server(osc_port: i32, ws_port: i32) -> Protocol {
        Protocol(unsafe { ffi::ossia_protocol_oscquery_server_create(osc_port, ws_port) })
    }

    pub fn oscquery_mirror(host: &str) -> Protocol {
        Protocol(unsafe {
            ffi::ossia_protocol_oscquery_mirror_create(host.as_ptr() as *const c_char)
        })
    }
}

impl Drop for Protocol {
    fn drop(&mut self) {
        unsafe { ffi::ossia_protocol_free(self.0) }
    }
}
