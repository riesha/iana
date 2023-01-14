use core::ptr;

use anyhow::*;
use winapi::um::{memoryapi::VirtualProtect, winnt::PAGE_EXECUTE_READWRITE};

pub(crate) fn write_readonly(address: *mut u8, data: *const u8, len: i32) -> Result<()>
{
    #[allow(unused_mut)]
    let mut old = 0u32;

    let protect = unsafe {
        VirtualProtect(
            address as _,
            len as _,
            PAGE_EXECUTE_READWRITE,
            ptr::addr_of_mut!(old),
        )
    };
    ensure!(protect != 0, " Error while setting PAGE_EXECUTE_READWRITE");

    unsafe {
        address.copy_from_nonoverlapping(data as _, len as _);
    }

    let protect =
        unsafe { VirtualProtect(address as _, len as _, old as _, ptr::addr_of_mut!(old)) };
    ensure!(protect != 0, " Error while restoring old flags");

    Ok(())
}

#[macro_export]

macro_rules! map_err {
    ($thing:expr) => {
        $thing.map_err(Error::msg)?
    };
}
