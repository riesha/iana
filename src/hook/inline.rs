use core::slice;

use alloc::vec::Vec;
use anyhow::*;
use iced_x86::{
    code_asm::{eax, rax, CodeAssembler},
    Decoder,
};

use crate::{map_err, utils::write_readonly, ARCH, JMP_SIZE};

pub struct InlineHook
{
    pub original:     usize,
    pub target:       usize,
    pub enabled:      bool,
    pub stolen_bytes: Vec<u8>,
}

impl InlineHook
{
    pub fn create(original: *mut u8, target: *mut u8) -> Result<Self>
    {
        ensure!(!original.is_null(), "Original cannot be null");

        let bytes = unsafe { slice::from_raw_parts(original, 30) };

        let dec = Decoder::new(ARCH, bytes, 0);
        let mut asm = CodeAssembler::new(ARCH).map_err(Error::msg)?;

        let mut size = 0;
        for asm in dec
        {
            size += asm.len();
            if size >= JMP_SIZE
            {
                break;
            }
        }

        if cfg!(target_pointer_width = "64")
        {
            map_err!(asm.mov(rax, target as u64));
            map_err!(asm.jmp(rax));
        }
        else
        {
            map_err!(asm.mov(eax, target as u32));
            map_err!(asm.jmp(eax));
        }

        let ret = Self {
            original:     original as _,
            target:       target as _,
            enabled:      true,
            stolen_bytes: map_err!(unsafe { slice::from_raw_parts(original, size) }.try_into()),
        };

        write_readonly(original, map_err!(asm.assemble(0x0)).as_ptr(), size as _)?;

        Ok(ret)
    }

    pub fn destroy(&mut self) -> Result<&Self>
    {
        ensure!(self.enabled, "Hook already disabled");

        write_readonly(
            self.original as _,
            self.stolen_bytes.as_ptr(),
            self.stolen_bytes.len() as _,
        )?;

        self.enabled = false;

        Ok(self)
    }
}
