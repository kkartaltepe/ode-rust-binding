/* automatically generated by rust-bindgen */

use common::*;

#[link(name="ode")]
extern "C" {
    pub fn dWorldExportDIF(w: dWorldID, file: *mut FILE,
                           world_name: *const ::libc::c_char) -> ();
}
