/* automatically generated by rust-bindgen */

use common::*;

#[link(name="ode")]
extern "C" {
    pub fn dSetZero(a: *mut dReal, n: ::libc::c_int) -> ();
    pub fn dSetValue(a: *mut dReal, n: ::libc::c_int, value: dReal) -> ();
    pub fn dDot(a: *const dReal, b: *const dReal, n: ::libc::c_int) -> dReal;
    pub fn dMultiply0(A: *mut dReal, B: *const dReal, C: *const dReal,
                      p: ::libc::c_int, q: ::libc::c_int, r: ::libc::c_int)
     -> ();
    pub fn dMultiply1(A: *mut dReal, B: *const dReal, C: *const dReal,
                      p: ::libc::c_int, q: ::libc::c_int, r: ::libc::c_int)
     -> ();
    pub fn dMultiply2(A: *mut dReal, B: *const dReal, C: *const dReal,
                      p: ::libc::c_int, q: ::libc::c_int, r: ::libc::c_int)
     -> ();
    pub fn dFactorCholesky(A: *mut dReal, n: ::libc::c_int) -> ::libc::c_int;
    pub fn dSolveCholesky(L: *const dReal, b: *mut dReal, n: ::libc::c_int)
     -> ();
    pub fn dInvertPDMatrix(A: *const dReal, Ainv: *mut dReal,
                           n: ::libc::c_int) -> ::libc::c_int;
    pub fn dIsPositiveDefinite(A: *const dReal, n: ::libc::c_int)
     -> ::libc::c_int;
    pub fn dFactorLDLT(A: *mut dReal, d: *mut dReal, n: ::libc::c_int,
                       nskip: ::libc::c_int) -> ();
    pub fn dSolveL1(L: *const dReal, b: *mut dReal, n: ::libc::c_int,
                    nskip: ::libc::c_int) -> ();
    pub fn dSolveL1T(L: *const dReal, b: *mut dReal, n: ::libc::c_int,
                     nskip: ::libc::c_int) -> ();
    pub fn dVectorScale(a: *mut dReal, d: *const dReal, n: ::libc::c_int)
     -> ();
    pub fn dSolveLDLT(L: *const dReal, d: *const dReal, b: *mut dReal,
                      n: ::libc::c_int, nskip: ::libc::c_int) -> ();
    pub fn dLDLTAddTL(L: *mut dReal, d: *mut dReal, a: *const dReal,
                      n: ::libc::c_int, nskip: ::libc::c_int) -> ();
    pub fn dLDLTRemove(A: *mut *mut dReal, p: *const ::libc::c_int,
                       L: *mut dReal, d: *mut dReal, n1: ::libc::c_int,
                       n2: ::libc::c_int, r: ::libc::c_int,
                       nskip: ::libc::c_int) -> ();
    pub fn dRemoveRowCol(A: *mut dReal, n: ::libc::c_int,
                         nskip: ::libc::c_int, r: ::libc::c_int) -> ();
}