use core::mem::size_of_val;

use alloc::vec::Vec;
use anyhow::*;

use crate::utils;

#[derive(Default)]
pub struct VmtHook
{
    pub original: usize,
    clone:        Vec<usize>,
    backup:       Vec<usize>,
}

impl VmtHook
{
    pub fn init(&mut self) -> Result<&Self> { todo!() }
    pub fn create(original: *mut u8) -> Result<Self> //TODO: implement vtable cloning and RTTI info saving :(((((((((((((((((((((((((( brain small
    {
        ensure!(!original.is_null(), "Original cannot be null");

        let mut ret = Self {
            original: original as _,
            clone:    Vec::new(),
            backup:   Vec::new(),
        };
        let count = ret.count_methods();
        ret.clone.resize(count, 0);
        Ok(ret)
    }

    pub fn hook_method(&mut self, idx: usize, target: *mut u8) -> Result<&Self>
    {
        self.clone[idx] = target as _;

        let original = self.original as *mut *mut usize;

        utils::write_readonly(
            unsafe { original.read().add(idx) } as _,
            target,
            size_of_val(&target) as _,
        )?;

        Ok(self)
    }

    pub fn destroy(&mut self, idx: usize) -> Result<&Self>
    {
        self.hook_method(idx, self.backup[idx] as _)?;
        Ok(self)
    }

    fn count_methods(&mut self) -> usize
    {
        let original = self.original as *mut *mut usize;
        let mut fn_count = 0usize;

        // self.backup
        //     .insert(0, unsafe { original.read().sub(1).read() as _ }); // Preserve RTTI

        unsafe {
            while original.read().add(fn_count).read() > 0
            {
                self.backup.push(original.read().add(fn_count).read() as _);

                fn_count += 1;
            }
        }
        fn_count
    }
}
