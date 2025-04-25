// src/types.rs
use crate::ffi::ggml_type;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum Type {
    F32 = 0,
    F16 = 1,
    Q4_0 = 2,
    Q4_1 = 3,
    Q5_0 = 6,
    Q5_1 = 7,
    Q8_0 = 8,
    Q8_1 = 9,
    I8 = 10,
    I16 = 11,
    I32 = 12,
    Count = 13,
    Invalid = -1,
}

impl From<Type> for ggml_type {
    fn from(t: Type) -> Self {
        t as u32
    }
}

impl From<ggml_type> for Type {
    fn from(t: ggml_type) -> Self {
        match t {
            0 => Type::F32,
            1 => Type::F16,
            2 => Type::Q4_0,
            3 => Type::Q4_1,
            6 => Type::Q5_0,
            7 => Type::Q5_1,
            8 => Type::Q8_0,
            9 => Type::Q8_1,
            10 => Type::I8,
            11 => Type::I16,
            12 => Type::I32,
            13 => Type::Count,
            _ => Type::Invalid,
        }
    }
}
