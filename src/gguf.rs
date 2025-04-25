// src/gguf.rs
use crate::ffi::*;
use std::ffi::{CStr, CString};
//use std::ptr;

pub struct GgufFile {
    pub ctx: *mut gguf_context,
}

impl GgufFile {
    pub fn load(path: &str) -> Option<(Self, *mut ggml_context)> {
        let c_path = CString::new(path).ok()?;

        let mut ctx_data: *mut ggml_context = std::ptr::null_mut();

        let params = gguf_init_params {
            no_alloc: true,
            ctx: &mut ctx_data,
        };

        let ctx = unsafe { gguf_init_from_file(c_path.as_ptr(), params) };
        if ctx.is_null() {
            None
        } else {
            Some((Self { ctx }, ctx_data))
        }
    }

    pub fn tensor_count(&self) -> usize {
        unsafe { gguf_get_n_tensors(self.ctx) as usize }
    }

    pub fn tensor_name(&self, idx: usize) -> Option<String> {
        let name_ptr = unsafe { gguf_get_tensor_name(self.ctx, idx as i64) };
        if name_ptr.is_null() {
            None
        } else {
            Some(
                unsafe { CStr::from_ptr(name_ptr) }
                    .to_string_lossy()
                    .into_owned(),
            )
        }
    }

    pub fn metadata_count(&self) -> usize {
        unsafe { gguf_get_n_kv(self.ctx) as usize }
    }

    pub fn metadata_key(&self, idx: usize) -> Option<String> {
        let key_ptr = unsafe { gguf_get_key(self.ctx, idx as i64) };
        if key_ptr.is_null() {
            None
        } else {
            Some(
                unsafe { CStr::from_ptr(key_ptr) }
                    .to_string_lossy()
                    .into_owned(),
            )
        }
    }
}

impl Drop for GgufFile {
    fn drop(&mut self) {
        unsafe { gguf_free(self.ctx) };
    }
}
