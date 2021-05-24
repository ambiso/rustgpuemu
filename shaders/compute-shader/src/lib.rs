#![cfg_attr(
    target_arch = "spirv",
    no_std,
    feature(register_attr),
    register_attr(spirv)
)]

extern crate spirv_std;

#[cfg(not(target_arch = "spirv"))]
use spirv_std::macros::spirv;

// LocalSize/numthreads of (x = 32, y = 1, z = 1)
#[spirv(compute(threads(32)))]
pub fn main_cs() {}