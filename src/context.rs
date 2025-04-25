// src/context.rs
use crate::ffi::*;
use crate::types::Type;
use std::ptr;

pub struct Context {
    ctx: *mut ggml_context,
}

impl Context {
    pub fn new(mem_size: usize) -> Option<Self> {
        let params = ggml_init_params {
            mem_size,
            mem_buffer: ptr::null_mut(),
            no_alloc: false,
        };

        let ctx = unsafe { ggml_init(params) };
        if ctx.is_null() {
            None
        } else {
            Some(Self { ctx })
        }
    }

    pub fn new_tensor_1d(&self, typ: Type, ne0: usize) -> Option<*mut ggml_tensor> {
        let tensor = unsafe { ggml_new_tensor_1d(self.ctx, typ.into(), ne0 as _) };
        if tensor.is_null() { None } else { Some(tensor) }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe { ggml_free(self.ctx) }
    }
}
