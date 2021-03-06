/* automatically generated by rust-bindgen */

use common::*;

pub const dContactMu2: ::libc::c_uint = 1;
pub const dContactAxisDep: ::libc::c_uint = 1;
pub const dContactFDir1: ::libc::c_uint = 2;
pub const dContactBounce: ::libc::c_uint = 4;
pub const dContactSoftERP: ::libc::c_uint = 8;
pub const dContactSoftCFM: ::libc::c_uint = 16;
pub const dContactMotion1: ::libc::c_uint = 32;
pub const dContactMotion2: ::libc::c_uint = 64;
pub const dContactMotionN: ::libc::c_uint = 128;
pub const dContactSlip1: ::libc::c_uint = 256;
pub const dContactSlip2: ::libc::c_uint = 512;
pub const dContactRolling: ::libc::c_uint = 1024;
pub const dContactApprox0: ::libc::c_uint = 0;
pub const dContactApprox1_1: ::libc::c_uint = 4096;
pub const dContactApprox1_2: ::libc::c_uint = 8192;
pub const dContactApprox1_N: ::libc::c_uint = 16384;
pub const dContactApprox1: ::libc::c_uint = 28672;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_dSurfaceParameters {
    pub mode: ::libc::c_int,
    pub mu: dReal,
    pub mu2: dReal,
    pub rho: dReal,
    pub rho2: dReal,
    pub rhoN: dReal,
    pub bounce: dReal,
    pub bounce_vel: dReal,
    pub soft_erp: dReal,
    pub soft_cfm: dReal,
    pub motion1: dReal,
    pub motion2: dReal,
    pub motionN: dReal,
    pub slip1: dReal,
    pub slip2: dReal,
}
impl ::std::clone::Clone for Struct_dSurfaceParameters {
    fn clone(&self) -> Struct_dSurfaceParameters { *self }
}
impl ::std::default::Default for Struct_dSurfaceParameters {
    fn default() -> Struct_dSurfaceParameters {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type dSurfaceParameters = Struct_dSurfaceParameters;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_dContactGeom {
    pub pos: dVector3,
    pub normal: dVector3,
    pub depth: dReal,
    pub g1: dGeomID,
    pub g2: dGeomID,
    pub side1: ::libc::c_int,
    pub side2: ::libc::c_int,
}
impl ::std::clone::Clone for Struct_dContactGeom {
    fn clone(&self) -> Struct_dContactGeom { *self }
}
impl ::std::default::Default for Struct_dContactGeom {
    fn default() -> Struct_dContactGeom { unsafe { ::std::mem::zeroed() } }
}
pub type dContactGeom = Struct_dContactGeom;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_dContact {
    pub surface: dSurfaceParameters,
    pub geom: dContactGeom,
    pub fdir1: dVector3,
}
impl ::std::clone::Clone for Struct_dContact {
    fn clone(&self) -> Struct_dContact { *self }
}
impl ::std::default::Default for Struct_dContact {
    fn default() -> Struct_dContact { unsafe { ::std::mem::zeroed() } }
}
pub type dContact = Struct_dContact;
