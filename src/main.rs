// https://huggingface.co/docs/hub/en/gguf
pub mod context;
pub mod ffi;
pub mod gguf;
pub mod types; // <- add this

use crate::ffi::*;
use context::Context;
use gguf::GgufFile;
use std::{marker::PhantomData, slice};
use types::Type;

pub fn ggml_ne(tensor: *const ggml_tensor, i: i32) -> i64 {
    unsafe { (*tensor).ne[i as usize] as i64 }
}
pub struct Tensor<'a> {
    pub ptr: *mut ggml_tensor,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Tensor<'a> {
    pub fn from_ptr(ptr: *mut ggml_tensor) -> Option<Self> {
        if ptr.is_null() {
            None
        } else {
            Some(Self {
                ptr,
                phantom: PhantomData,
            })
        }
    }

    pub fn as_f32_slice(&self) -> Option<&[f32]> {
        // let dtype = unsafe { ggml_get_type(self.ptr) };
        // if dtype != ggml_type::GGML_TYPE_F32 {
        //     return None;
        // }

        let ndims = unsafe { ggml_n_dims(self.ptr) };
        let mut count = 1;
        for i in 0..ndims {
            count *= unsafe { ggml_ne(self.ptr, i) as usize };
        }

        let ptr = unsafe { ggml_get_data(self.ptr) } as *const f32;
        if ptr.is_null() {
            return None;
        }

        Some(unsafe { slice::from_raw_parts(ptr, count as usize) })
    }

    pub fn shape(&self) -> Vec<usize> {
        let ndims: i32 = unsafe { ggml_n_dims(self.ptr) };
        (0..ndims)
            .map(|i: i32| unsafe { ggml_ne(self.ptr, i) as usize })
            .collect()
    }

    // pub fn dtype(&self) -> ggml_type {
    //     unsafe { ggml_type(self.ptr) }
    // }
}

fn create_tensor() {
    println!("[DEBUG] create_tensor start");
    let ctx = Context::new(1024 * 1024).expect("Failed to init context");
    let tensor = ctx
        .new_tensor_1d(Type::F32, 16)
        .expect("Tensor creation failed");

    assert!(!tensor.is_null());
    println!("[DEBUG] create_tensor end");
}

fn load_gguf_and_print_info() {
    let path = "../../models/llama-2-13b-ensemble-v5.Q4_K_M.gguf"; // <<< Update path
    //let file = GgufFile::load(path).expect("Failed to load .gguf file");
    let (gguf_file, ctx) = GgufFile::load(path).expect("Failed to load");

    let tensor_count = gguf_file.tensor_count();
    println!("Tensors: {tensor_count}");

    for i in 0..tensor_count.min(5) {
        // Show only first 5
        let name = gguf_file.tensor_name(i).unwrap_or("<unknown>".to_string());
        println!("Tensor[{i}]: {name}");

        println!("Tensor[{i}]: {name}");

        // Look up the tensor by name
        let tensor_ptr = {
            let c_name = std::ffi::CString::new(name.clone()).unwrap();
            unsafe { ggml_get_tensor(ctx, c_name.as_ptr()) }
        };

        let tensor = Tensor::from_ptr(tensor_ptr).expect("null tensor pointer");

        println!("  Shape: {:?}", tensor.shape());
        //println!("  Type: {:?}", tensor.dtype());

        if let Some(values) = tensor.as_f32_slice() {
            println!("  First few values: {:?}", &values[0..values.len().min(5)]);
        } else {
            println!("  [not f32 or data not loaded]");
        }
    }

    let meta_count = gguf_file.metadata_count();
    println!("Metadata keys: {meta_count}");

    for i in 0..meta_count.min(5) {
        let key = gguf_file.metadata_key(i).unwrap_or("<unknown>".to_string());
        println!("Metadata[{i}]: {key}");
    }
}

fn main() {
    create_tensor();
    load_gguf_and_print_info();
}
