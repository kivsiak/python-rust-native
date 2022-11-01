use std::os::raw::c_int;
use libhello_sys::adder as adder_sys;

pub fn adder(a: i64, b: i64) -> i64 {
    unsafe {
        adder_sys(a as c_int, b as c_int) as i64
    }
}