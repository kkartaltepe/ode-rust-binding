/* automatically generated by rust-bindgen */

use common::*;

pub type Enum_dInitODEFlags = ::libc::c_uint;
pub const dInitFlagManualThreadCleanup: ::libc::c_uint = 1;
pub type Enum_dAllocateODEDataFlags = ::libc::c_int;
pub const dAllocateFlagBasicData: ::libc::c_int = 0;
pub const dAllocateFlagCollisionData: ::libc::c_int = 1;
pub const dAllocateMaskAll: ::libc::c_int = -1;

#[link(name="ode")]
extern "C" {
    pub fn dInitODE() -> ();
    pub fn dInitODE2(uiInitFlags: ::libc::c_uint) -> ::libc::c_int;
    pub fn dAllocateODEDataForThread(uiAllocateFlags: ::libc::c_uint)
     -> ::libc::c_int;
    pub fn dCleanupODEAllDataForThread() -> ();
    pub fn dCloseODE() -> ();
}
