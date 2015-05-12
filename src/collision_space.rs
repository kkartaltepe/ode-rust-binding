/* automatically generated by rust-bindgen */

use common::*;

pub type dNearCallback =
    extern "C" fn(data: *mut ::libc::c_void, o1: dGeomID, o2: dGeomID) -> ();

#[link(name="ode")]
extern "C" {
    pub fn dSimpleSpaceCreate(space: dSpaceID) -> dSpaceID;
    pub fn dHashSpaceCreate(space: dSpaceID) -> dSpaceID;
    pub fn dQuadTreeSpaceCreate(space: dSpaceID, Center: dVector3,
                                Extents: dVector3, Depth: ::libc::c_int)
     -> dSpaceID;
    pub fn dSweepAndPruneSpaceCreate(space: dSpaceID,
                                     axisorder: ::libc::c_int) -> dSpaceID;
    pub fn dSpaceDestroy(arg1: dSpaceID) -> ();
    pub fn dHashSpaceSetLevels(space: dSpaceID, minlevel: ::libc::c_int,
                               maxlevel: ::libc::c_int) -> ();
    pub fn dHashSpaceGetLevels(space: dSpaceID, minlevel: *mut ::libc::c_int,
                               maxlevel: *mut ::libc::c_int) -> ();
    pub fn dSpaceSetCleanup(space: dSpaceID, mode: ::libc::c_int) -> ();
    pub fn dSpaceGetCleanup(space: dSpaceID) -> ::libc::c_int;
    pub fn dSpaceSetSublevel(space: dSpaceID, sublevel: ::libc::c_int) -> ();
    pub fn dSpaceGetSublevel(space: dSpaceID) -> ::libc::c_int;
    pub fn dSpaceSetManualCleanup(space: dSpaceID, mode: ::libc::c_int) -> ();
    pub fn dSpaceGetManualCleanup(space: dSpaceID) -> ::libc::c_int;
    pub fn dSpaceAdd(arg1: dSpaceID, arg2: dGeomID) -> ();
    pub fn dSpaceRemove(arg1: dSpaceID, arg2: dGeomID) -> ();
    pub fn dSpaceQuery(arg1: dSpaceID, arg2: dGeomID) -> ::libc::c_int;
    pub fn dSpaceClean(arg1: dSpaceID) -> ();
    pub fn dSpaceGetNumGeoms(arg1: dSpaceID) -> ::libc::c_int;
    pub fn dSpaceGetGeom(arg1: dSpaceID, i: ::libc::c_int) -> dGeomID;
    pub fn dSpaceGetClass(space: dSpaceID) -> ::libc::c_int;
}
