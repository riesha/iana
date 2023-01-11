#![no_std]
pub mod hook;
pub mod utils;
extern crate alloc;
#[cfg(target_pointer_width = "32")]
pub(crate) const ARCH: u32 = 32;
#[cfg(target_pointer_width = "32")]
pub(crate) const JMP_SIZE: usize = 7;

#[cfg(target_pointer_width = "64")]
pub(crate) const ARCH: u32 = 64;
#[cfg(target_pointer_width = "64")]
pub(crate) const JMP_SIZE: usize = 13;
