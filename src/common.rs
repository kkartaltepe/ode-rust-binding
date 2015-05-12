/* automatically generated by rust-bindgen */

pub use error::*;
pub use odeconfig::*;

pub type dReal = ::libc::c_float;
pub type dTriIndex = duint32;
pub type dVector3 = [dReal; 4usize];
pub type dVector4 = [dReal; 4usize];
pub type dMatrix3 = [dReal; 12usize];
pub type dMatrix4 = [dReal; 16usize];
pub type dMatrix6 = [dReal; 48usize];
pub type dQuaternion = [dReal; 4usize];
pub enum Struct_dxWorld { }
pub enum Struct_dxSpace { }
pub enum Struct_dxBody { }
pub enum Struct_dxGeom { }
pub enum Struct_dxJoint { }
pub enum Struct_dxJointNode { }
pub enum Struct_dxJointGroup { }
pub enum Struct_dxWorldProcessThreadingManager { }
pub type dWorldID = *mut Struct_dxWorld;
pub type dSpaceID = *mut Struct_dxSpace;
pub type dBodyID = *mut Struct_dxBody;
pub type dGeomID = *mut Struct_dxGeom;
pub type dJointID = *mut Struct_dxJoint;
pub type dJointGroupID = *mut Struct_dxJointGroup;
pub type dWorldStepThreadingManagerID =
    *mut Struct_dxWorldProcessThreadingManager;
pub const d_ERR_UNKNOWN: ::libc::c_uint = 0;
pub const d_ERR_IASSERT: ::libc::c_uint = 1;
pub const d_ERR_UASSERT: ::libc::c_uint = 2;
pub const d_ERR_LCP: ::libc::c_uint = 3;
pub const dJointTypeNone: ::libc::c_uint = 0;
pub const dJointTypeBall: ::libc::c_uint = 1;
pub const dJointTypeHinge: ::libc::c_uint = 2;
pub const dJointTypeSlider: ::libc::c_uint = 3;
pub const dJointTypeContact: ::libc::c_uint = 4;
pub const dJointTypeUniversal: ::libc::c_uint = 5;
pub const dJointTypeHinge2: ::libc::c_uint = 6;
pub const dJointTypeFixed: ::libc::c_uint = 7;
pub const dJointTypeNull: ::libc::c_uint = 8;
pub const dJointTypeAMotor: ::libc::c_uint = 9;
pub const dJointTypeLMotor: ::libc::c_uint = 10;
pub const dJointTypePlane2D: ::libc::c_uint = 11;
pub const dJointTypePR: ::libc::c_uint = 12;
pub const dJointTypePU: ::libc::c_uint = 13;
pub const dJointTypePiston: ::libc::c_uint = 14;
pub const dJointTypeDBall: ::libc::c_uint = 15;
pub const dJointTypeDHinge: ::libc::c_uint = 16;
pub const dJointTypeTransmission: ::libc::c_uint = 17;
pub type dJointType = ::libc::c_uint;
pub const dParamLoStop: ::libc::c_uint = 0;
pub const dParamHiStop: ::libc::c_uint = 1;
pub const dParamVel: ::libc::c_uint = 2;
pub const dParamLoVel: ::libc::c_uint = 3;
pub const dParamHiVel: ::libc::c_uint = 4;
pub const dParamFMax: ::libc::c_uint = 5;
pub const dParamFudgeFactor: ::libc::c_uint = 6;
pub const dParamBounce: ::libc::c_uint = 7;
pub const dParamCFM: ::libc::c_uint = 8;
pub const dParamStopERP: ::libc::c_uint = 9;
pub const dParamStopCFM: ::libc::c_uint = 10;
pub const dParamSuspensionERP: ::libc::c_uint = 11;
pub const dParamSuspensionCFM: ::libc::c_uint = 12;
pub const dParamERP: ::libc::c_uint = 13;
pub const dParamsInGroup: ::libc::c_uint = 14;
pub const dParamGroup1: ::libc::c_uint = 0;
pub const dParamLoStop1: ::libc::c_uint = 0;
pub const dParamHiStop1: ::libc::c_uint = 1;
pub const dParamVel1: ::libc::c_uint = 2;
pub const dParamLoVel1: ::libc::c_uint = 3;
pub const dParamHiVel1: ::libc::c_uint = 4;
pub const dParamFMax1: ::libc::c_uint = 5;
pub const dParamFudgeFactor1: ::libc::c_uint = 6;
pub const dParamBounce1: ::libc::c_uint = 7;
pub const dParamCFM1: ::libc::c_uint = 8;
pub const dParamStopERP1: ::libc::c_uint = 9;
pub const dParamStopCFM1: ::libc::c_uint = 10;
pub const dParamSuspensionERP1: ::libc::c_uint = 11;
pub const dParamSuspensionCFM1: ::libc::c_uint = 12;
pub const dParamERP1: ::libc::c_uint = 13;
pub const dParamGroup2: ::libc::c_uint = 256;
pub const dParamLoStop2: ::libc::c_uint = 256;
pub const dParamHiStop2: ::libc::c_uint = 257;
pub const dParamVel2: ::libc::c_uint = 258;
pub const dParamLoVel2: ::libc::c_uint = 259;
pub const dParamHiVel2: ::libc::c_uint = 260;
pub const dParamFMax2: ::libc::c_uint = 261;
pub const dParamFudgeFactor2: ::libc::c_uint = 262;
pub const dParamBounce2: ::libc::c_uint = 263;
pub const dParamCFM2: ::libc::c_uint = 264;
pub const dParamStopERP2: ::libc::c_uint = 265;
pub const dParamStopCFM2: ::libc::c_uint = 266;
pub const dParamSuspensionERP2: ::libc::c_uint = 267;
pub const dParamSuspensionCFM2: ::libc::c_uint = 268;
pub const dParamERP2: ::libc::c_uint = 269;
pub const dParamGroup3: ::libc::c_uint = 512;
pub const dParamLoStop3: ::libc::c_uint = 512;
pub const dParamHiStop3: ::libc::c_uint = 513;
pub const dParamVel3: ::libc::c_uint = 514;
pub const dParamLoVel3: ::libc::c_uint = 515;
pub const dParamHiVel3: ::libc::c_uint = 516;
pub const dParamFMax3: ::libc::c_uint = 517;
pub const dParamFudgeFactor3: ::libc::c_uint = 518;
pub const dParamBounce3: ::libc::c_uint = 519;
pub const dParamCFM3: ::libc::c_uint = 520;
pub const dParamStopERP3: ::libc::c_uint = 521;
pub const dParamStopCFM3: ::libc::c_uint = 522;
pub const dParamSuspensionERP3: ::libc::c_uint = 523;
pub const dParamSuspensionCFM3: ::libc::c_uint = 524;
pub const dParamERP3: ::libc::c_uint = 525;
pub const dParamGroup: ::libc::c_uint = 256;
pub const dAMotorUser: ::libc::c_uint = 0;
pub const dAMotorEuler: ::libc::c_uint = 1;
pub const dTransmissionParallelAxes: ::libc::c_uint = 0;
pub const dTransmissionIntersectingAxes: ::libc::c_uint = 1;
pub const dTransmissionChainDrive: ::libc::c_uint = 2;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_dJointFeedback {
    pub f1: dVector3,
    pub t1: dVector3,
    pub f2: dVector3,
    pub t2: dVector3,
}
impl ::std::clone::Clone for Struct_dJointFeedback {
    fn clone(&self) -> Struct_dJointFeedback { *self }
}
impl ::std::default::Default for Struct_dJointFeedback {
    fn default() -> Struct_dJointFeedback { unsafe { ::std::mem::zeroed() } }
}
pub type dJointFeedback = Struct_dJointFeedback;

#[link(name="ode")]
extern "C" {
    pub fn dGeomMoved(arg1: dGeomID) -> ();
    pub fn dGeomGetBodyNext(arg1: dGeomID) -> dGeomID;
    pub fn dGetConfiguration() -> *const ::libc::c_char;
    pub fn dCheckConfiguration(token: *const ::libc::c_char) -> ::libc::c_int;
}
